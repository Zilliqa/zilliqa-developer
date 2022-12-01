import os
import subprocess

subprocess.check_output(
    "git clone git@github.com:Zilliqa/devops.git .infra", shell=True
)
os.chdir(".infra")
branch_output = subprocess.checkout("git branch", stderr=subprocess.STDOUT, shell=True)
branches = [x.strip() for x in branch_output.split("\n") if x.strip() != ""]

#          # Placeholder for doing real change
#          echo "DO NOT MERGE" >> cd/applications/devportal/overlays/preview/kustomization.yaml
#
#          git add . -A
#          git commit -m "Preparing preview for commit: ${{ github.sha }}
#          git push
