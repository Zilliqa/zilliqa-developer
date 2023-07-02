import os
import re
import subprocess
import sys

GIT_SHORT_HASH_LEGNTH = 9


def is_git_dirty(path):
    p = subprocess.Popen(["git", "status", "-s"], cwd=path, stdout=subprocess.PIPE)
    (out, err) = p.communicate()
    if p.returncode != 0:
        sys.exit(p.returncode)
    return out.decode("ascii").strip() != ""


def get_dummy_version():
    ret = {
        "major": 0,
        "minor": 0,
        "placeholder": 0,
        "prerelease": "",
        "patch": 0,
        "build": "unknown",
        "is_dirty": True,
        "commit_hash": "000000",
        "describe": "unknown.0",
        "regex_match": "no_match",
        "intersection_point": "main",
        "dist_from_main_intersection": -1,
        "branch": "unknown",
        "branches": "",
    }
    return ret


def get_version_from_git(path):
    subprocess.check_output("git fetch --all --tags", shell=True)
    ret = {
        "major": 0,
        "minor": 0,
        "placeholder": 0,
        "prerelease": "",
        "patch": 0,
        "build": "unknown",
    }

    git_is_dirty = is_git_dirty(".")
    ret["is_dirty"] = git_is_dirty

    ret["commit_hash"] = get_git_hash(".")

    pattern = re.compile(
        r"(v\.? ?)?(?P<major>\d+)(\.(?P<minor>\d\d?))(\.(?P<placeholder>[\d\w]\d?))*(\-(?P<prerelease>[a-zA-Z][\w\d]+))?(\-(?P<patch>\d+)\-g(?P<build>[\w\d]{6,10}))?"
    )

    # Getting git description
    p = subprocess.Popen(
        ["git", "describe", "--tags"],
        cwd=path,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    (out, err) = p.communicate()
    if p.returncode != 0:
        ret["describe"] = "error"
        out = "0.0.0"
    else:
        out = out.decode("ascii").strip()
        ret["describe"] = out

        if "fatal" in out.lower():
            out = "0.0.0"

    m = pattern.search(out)
    ret["regex_match"] = "no match"
    if m:
        ret["prerelease"] = ""
        ret.update(m.groupdict())
        ret["regex_match"] = ",".join(
            "{}={}".format(str(k).strip(), str(v).strip())
            for k, v in m.groupdict().items()
        )
        if ret["patch"] is None:
            ret["patch"] = 0
        if ret["prerelease"] is None:
            ret["prerelease"] = ""

    ret["major"] = int(ret["major"])
    ret["minor"] = int(ret["minor"])
    ret["patch"] = int(ret["patch"])

    if git_is_dirty:
        # We increate the patch number by one if we are working on a dirty branch
        # since technically we are working on the next version.
        ret["patch"] = int(ret["patch"]) + 1

    # Finding intersectoin point between main and head
    p = subprocess.Popen(
        ["git", "merge-base", "HEAD", "main"],
        cwd=path,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    intersection_point = "main"
    (out, err) = p.communicate()
    if p.returncode == 0:
        intersection_point = out.decode("ascii").strip()

    ret["intersection_point"] = intersection_point

    # Compunting commits since intersection
    p = subprocess.Popen(
        ["git", "rev-list", "--count", "HEAD", "^{}".format(intersection_point)],
        cwd=path,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    # Getting the distance to intersection
    (out, err) = p.communicate()
    dist_from_main_intersection = -1
    if p.returncode == 0:
        dist_from_main_intersection = int(out.decode("ascii").strip())

    ret["dist_from_main_intersection"] = dist_from_main_intersection

    # Getting list of all branches
    p = subprocess.Popen(
        ["git", "branch"], cwd=path, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )

    # Checking if we are starting a new release
    (out, err) = p.communicate()
    ret["branch"] = "unknown"
    if p.returncode == 0:
        out = out.decode("ascii").strip().split("\n")
        ret["branches"] = ",".join([x.strip() for x in out])

        for line in out:
            line = line.strip()
            if line.startswith("*"):
                line = line[2:].strip()
                ret["branch"] = line
                if line.startswith("pre-release/") or line.startswith("release/"):
                    _, line = line.split("/", 1)
                    if line.startswith("v"):
                        line = line[1:].strip()
                    if line.startswith("."):
                        line = line[1:].strip()

                    try:
                        try:
                            major, minor = line.split(".")
                        except ValueError:
                            major, minor, _ = line.split(".")

                        major = int(major)
                        minor = int(minor)
                        if (
                            major >= ret["major"]
                            or (major == ret["major"] and minor >= ret["minor"])
                            and ret["prerelease"] != ""
                        ):
                            ret["major"] = major
                            ret["minor"] = minor
                            ret["patch"] = 0
                            ret["prerelease"] = "rc{}".format(
                                dist_from_main_intersection
                            )
                    except ValueError:
                        raise

    return ret


def get_version(major, minor, patch, prerelease, commit_hash, is_dirty, **kwargs):
    full = "{}.{}.{}".format(major, minor, patch)
    if prerelease != "":
        full += "-{}".format(prerelease)

    if is_dirty:
        full += "-wip"

    return full


def main():
    # Use this to disable status when you have a yubikey and would
    # otherwise need to keep touching it for every op
    # - rrw 2023-04-25
    if os.environ.get("DISABLE_WORKSPACE_STATUS") is not None:
        version = get_dummy_version()
    else:
        version = get_version_from_git(".")

    version["version"] = get_version(**version)
    git_hash = version["commit_hash"]
    git_is_dirty = version["is_dirty"]
    version["full_version"] = "{}+{}".format(
        version["version"], git_hash[:GIT_SHORT_HASH_LEGNTH]
    ).lower()
    version["full_version_tag"] = version["full_version"].replace("+", "-").lower()
    version["full_version_uri"] = version["full_version_tag"].replace(".", "-").lower()

    version["partial_version_uri"] = (
        version["full_version"].split("+", 1)[0].replace(".", "-").lower()
    )

    version["build_uri_suffix"] = os.environ.get(
        "BUILD_URI_SUFFIX", git_hash[:GIT_SHORT_HASH_LEGNTH]
    )
    if "/" in version["build_uri_suffix"]:
        version["build_uri_suffix"] = version["build_uri_suffix"].rsplit("/", 1)[1]
        version["build_uri_suffix"] = (
            version["build_uri_suffix"].replace(".", "-").replace("+", "-").lower()
        )

    print("STABLE_VERSION {version}".format(**version))
    print("STABLE_GIT_MAJOR {major}".format(**version))
    print("STABLE_GIT_MINOR {minor}".format(**version))
    print("STABLE_GIT_PATCH {patch}".format(**version))
    print("STABLE_GIT_PRERELEASE {prerelease}".format(**version))

    print("FULL_VERSION {full_version}".format(**version))
    print("FULL_VERSION_TAG {full_version_tag}".format(**version))
    print("FULL_VERSION_URI {full_version_uri}".format(**version))

    print("GIT_MAIN_INTERSECTION {intersection_point}".format(**version))
    print(
        "GIT_DISTANCE_FROM_MAIN_INTERSECTION {dist_from_main_intersection}".format(
            **version
        )
    )
    print("GIT_DIRTY {}".format("1" if git_is_dirty else "0"))
    print("GIT_COMMIT_HASH {}".format(git_hash))
    print("GIT_SHORT_HASH {}".format(git_hash[:GIT_SHORT_HASH_LEGNTH]))
    print("GIT_BRANCH {branch}".format(**version))
    print("GIT_BRANCHES {branches}".format(**version))
    print("GIT_DESCRIBE {describe}".format(**version))

    if version["build_uri_suffix"]:
        print(
            "CUSTOM_VERSION_URI {partial_version_uri}-{build_uri_suffix}".format(
                **version
            )
        )
    else:
        print("CUSTOM_VERSION_URI {partial_version_uri}".format(**version))

    print("REGEX_DESCRIBE_MATCH {regex_match}".format(**version))


def get_git_hash(path):
    p = subprocess.Popen(["git", "rev-parse", "HEAD"], cwd=path, stdout=subprocess.PIPE)
    (out, err) = p.communicate()
    if p.returncode != 0:
        sys.exit(p.returncode)
    return out.decode("ascii").strip()


if __name__ == "__main__":
    main()
