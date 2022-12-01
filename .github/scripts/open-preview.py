import os
import subprocess

workspace_status = subprocess.check_output(
    "python config/workspace-status.py", shell=True
).decode("utf-8")
status = dict([tuple(x.split(" ", 1)) for x in workspace_status.strip().split("\n")])
print(status)

branch_id = "preview/developer-{}".format(status["STABLE_GIT_SHORT_HASH"])

subprocess.check_output(
    "git clone git@github.com:Zilliqa/devops.git .infra", shell=True
)
os.chdir(".infra")
branch_output = subprocess.check_output(
    "git branch", stderr=subprocess.STDOUT, shell=True
).decode("utf-8")
branches = [x.strip() for x in branch_output.split("\n") if x.strip() != ""]

for branch in branches:
    print("-", branch)


if branch_id in branches:
    print("Checking preview branch out")
    subprocess.check_output("git checkout {}".format(branch_id), shell=True)
else:
    print("Creating branch")
    subprocess.check_output("git checkout -b {}".format(branch_id), shell=True)

#          # Placeholder for doing real change
#          echo "DO NOT MERGE" >> cd/applications/devportal/overlays/preview/kustomization.yaml
#
#          git add . -A
#          git commit -m "Preparing preview for commit: ${{ github.sha }}
#          git push


exit(-1)
