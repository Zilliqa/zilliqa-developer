import os
import subprocess

# Defining branch name
print("Getting workspace status")
workspace_status = subprocess.check_output(
    "python config/workspace-status.py", shell=True
).decode("utf-8")
status = dict([tuple(x.split(" ", 1)) for x in workspace_status.strip().split("\n")])

branch_id = "preview/developer-{}".format(status["STABLE_GIT_SHORT_HASH"])

## Getting the devops repo
subprocess.check_output(
    "git clone git@github.com:Zilliqa/devops.git .infra", shell=True
)

## Checking out the branch
print("Switching branch")
os.chdir(".infra")

branch_output = subprocess.check_output(
    "git branch", stderr=subprocess.STDOUT, shell=True
).decode("utf-8")
branches = [x.strip() for x in branch_output.split("\n") if x.strip() != ""]

if branch_id in branches:
    print("Checking preview branch out")
    subprocess.check_output("git checkout {}".format(branch_id), shell=True)
else:
    print("Creating branch")
    subprocess.check_output("git checkout -b {}".format(branch_id), shell=True)

# Creating the patch
print("Creating patch")
os.chdir("..")

# TODO: Update this to one single patch
subprocess.check_output(
    "bazelisk build //products/developer-portal:cd_preview", shell=True
)

# Applying patch
print("Applying patch")
os.chdir(".infra")
subprocess.check_output(
    "tar xvf ../bazel-bin/products/developer-portal/cd_preview.tar", shell=True
)


# Pushing
print("Pushing")
# TODO: Check if anything updated
subprocess.check_output("git add . -A", shell=True)
subprocess.check_output(
    'git commit -m "Preparing preview for commit: {}"'.format(
        status["STABLE_GIT_SHORT_HASH"]
    ),
    shell=True,
)
subprocess.check_output("git push", shell=True)


exit(-1)
