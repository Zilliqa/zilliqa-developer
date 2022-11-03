import argparse
import os
import sys
from string import Template

commit_hash_name = "GIT_COMMIT_HASH"
workspace_dirty_name = "GIT_DIRTY"
major_key = "GIT_MAJOR"
minor_key = "GIT_MINOR"
revision_key = "GIT_REVISION"
channel_key = "GIT_CHANNEL"
patch_key = "GIT_PATCH"


class VariableStore:
    def __init__(self, values, is_reliable, name_prefix):
        self.values = values
        self.name_prefix = name_prefix
        self.is_reliable = is_reliable

    def get(self, name):
        return self.values.get(self.name_prefix + name)


class MultipleVariableStore:
    def __init__(self):
        self.values = []

    def add_file(self, path, is_reliable, name_prefix=""):
        result = {}
        with open(path, "r") as f:
            for entry in f.read().split("\n"):
                if entry:
                    key_value = entry.split(" ", 1)
                    key = key_value[0].strip()
                    if key in result:
                        sys.stderr.write("Error: Duplicate key '{}'\n".format(key))
                        sys.exit(1)
                    else:
                        result[key] = key_value[1].strip()
        self.values.append(VariableStore(result, is_reliable, name_prefix))

    def get(self, name):
        for l in self.values:
            result = l.get(name)
            if result is not None:
                return (result, l.is_reliable)
        return (None, False)


def setup_path(file_path):
    header_dir = os.path.normpath(os.path.join(file_path, ".."))
    if not os.path.exists(header_dir):
        os.makedirs(header_dir)


def main():

    parser = argparse.ArgumentParser(
        description="Bake a git hash into a header & source."
    )
    parser.add_argument("--output", required=True, help="output file")

    parser.add_argument("--template", required=True, help="template  file")
    parser.add_argument("--true_value", required=True, help="value that denotes true")
    parser.add_argument("--false_value", required=True, help="value that denotes false")

    parser.add_argument(
        "--volatile_file", required=True, help="file containing the volatile variables"
    )
    parser.add_argument(
        "--stable_file", required=True, help="file containing the stable variables"
    )

    args = parser.parse_args()

    variables = MultipleVariableStore()
    variables.add_file(args.stable_file, True, "STABLE_")
    variables.add_file(args.volatile_file, False)

    (commit_hash, commit_hash_reliable) = variables.get(commit_hash_name.strip())
    (is_dirty_str, is_dirty_reliable) = variables.get(workspace_dirty_name.strip())

    (major, major_reliable) = variables.get(major_key.strip())
    (minor, minor_reliable) = variables.get(minor_key.strip())
    (revision, revision_reliable) = variables.get(revision_key.strip())
    (channel, channel_reliable) = variables.get(channel_key.strip())
    (patch, patch_reliable) = variables.get(patch_key.strip())

    is_dirty = "0" != is_dirty_str

    full = "%s.%s.%s" % (major, minor, revision)
    if channel != "release":
        full += "-%s" % channel

    if patch != "0":
        full += "-patch-%s-%s" % (patch, commit_hash[:10])

    if is_dirty:
        full += "-wip"

    with open(args.template, "r") as fb:
        template = Template(fb.read())

    setup_path(args.output)
    with open(args.output, "w") as f:
        f.write(
            template.substitute(
                commit_hash=commit_hash,
                hash_len=len(commit_hash),
                hash_reliable=args.true_value
                if commit_hash_reliable
                else args.false_value,
                is_dirty=args.true_value if is_dirty else args.false_value,
                is_dirty_reliable=args.true_value
                if is_dirty_reliable
                else args.false_value,
                major=major,
                minor=minor,
                revision=revision,
                channel=channel,
                patch=patch,
                full_version=full,
            )
        )


if __name__ == "__main__":
    main()
