import os
import subprocess
import sys

from cd import version

# Defining branch name
stable_git_hash = version.stable_git_hash
branch_id = version.branch_id
patches = sys.argv[1:]

print("Branch: {}".format(branch_id))
print(patches)

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

# Applying patch
print("Applying patch")
for patch in patches:
    os.system("tar xvf {}".format(patch))


# Pushing
print("Pushing")
# TODO: Check if anything updated
os.system("git add . -A")
os.system('git commit -m "Preparing preview for commit: {}"'.format(stable_git_hash))
os.system("git push --set-upstream origin {}".format(branch_id))
