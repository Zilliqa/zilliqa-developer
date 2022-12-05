import re
import subprocess
import sys


def is_git_dirty(path):
    p = subprocess.Popen(["git", "status", "-s"], cwd=path, stdout=subprocess.PIPE)
    (out, err) = p.communicate()
    if p.returncode != 0:
        sys.exit(p.returncode)
    return out.decode("ascii").strip() != ""


def get_version_from_git(path):
    subprocess.check_output("git fetch --all --tags", shell=True)
    ret = {
        "major": 0,
        "minor": 0,
        "placeholder": 0,
        "prerelease": "",
        "patch": 0,
        "build": "unkown",
    }

    git_is_dirty = is_git_dirty(".")
    ret["is_dirty"] = git_is_dirty
    if git_is_dirty:
        # We increate the patch number by one if we are working on a dirty branch
        # since technically we are working on the next version.
        ret["patch"] = int(ret["patch"]) + 1

    ret["commit_hash"] = get_git_hash(".")

    pattern = re.compile(
        r"(v\.? ?)?(?P<major>\d+)(\.(?P<minor>\d\d?))(\.(?P<placeholder>[\d\w]\d?))*(\-(?P<prerelease>\w[\w\d]+))?(\-(?P<patch>\d+)\-(?P<build>[\w\d]{10}))?"
    )
    p = subprocess.Popen(
        ["git", "describe"], cwd=path, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )

    (out, err) = p.communicate()
    if p.returncode != 0:
        return ret

    out = out.decode("ascii").strip()

    if "fatal" in out.lower():
        return ret

    m = pattern.search(out)
    if m:
        ret["prerelease"] = ""
        ret.update(m.groupdict())
        if ret["patch"] is None:
            ret["patch"] = 0
        if ret["prerelease"] is None:
            ret["prerelease"] = ""

    return ret


def get_version(major, minor, patch, prerelease, commit_hash, is_dirty, **kwargs):
    full = "{}.{}.{}".format(major, minor, patch)
    if prerelease != "":
        full += "-{}".format(prerelease)

    if is_dirty:
        full += "-wip"

    return full


def main():
    version = get_version_from_git(".")
    version["version"] = get_version(**version)
    git_hash = version["commit_hash"]
    git_is_dirty = version["is_dirty"]
    version["full_version"] = "{}+{}".format(version["version"], git_hash[:7])
    version["full_version_uri"] = (
        version["full_version"].replace(".", "-").replace("+", "_")
    )

    print("STABLE_VERSION {version}".format(**version))
    print("STABLE_FULL_VERSION {full_version}".format(**version))
    print("STABLE_FULL_VERSION_URI {full_version_uri}".format(**version))
    print("STABLE_GIT_MAJOR {major}".format(**version))
    print("STABLE_GIT_MINOR {minor}".format(**version))
    print("STABLE_GIT_PATCH {patch}".format(**version))
    print("STABLE_GIT_CHANNEL {prerelease}".format(**version))
    print("STABLE_GIT_DIRTY {}".format("1" if git_is_dirty else "0"))
    print("STABLE_GIT_COMMIT_HASH {}".format(git_hash))
    print("STABLE_GIT_SHORT_HASH {}".format(git_hash[:7]))


def get_git_hash(path):
    p = subprocess.Popen(["git", "rev-parse", "HEAD"], cwd=path, stdout=subprocess.PIPE)
    (out, err) = p.communicate()
    if p.returncode != 0:
        sys.exit(p.returncode)
    return out.decode("ascii").strip()


if __name__ == "__main__":
    main()
