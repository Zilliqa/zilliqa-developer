import os
import subprocess
import sys

import yaml
from github import Github

from cd import version


def is_git_dirty(path):
    p = subprocess.Popen(["git", "status", "-s"], cwd=path, stdout=subprocess.PIPE)
    (out, err) = p.communicate()
    if p.returncode != 0:
        sys.exit(p.returncode)
    return out.decode("ascii").strip() != ""


def get_main_pull_request(github, pr_ref):
    repo = github.get_repo("Zilliqa/zilliqa-developer")
    current_pull = None
    pulls = repo.get_pulls(state="open", sort="created", head=pr_ref)

    for p in pulls:
        if p.head.ref == pr_ref:
            current_pull = p
            break

    if not current_pull:
        print("Could not find PR {}".format(pr_ref))
        exit(-1)

    return current_pull


def create_pr(github, orig_branch, branch_id):
    repo = github.get_repo("Zilliqa/devops")

    pull = None
    pulls = repo.get_pulls(state="open", sort="created", base="main")
    for p in pulls:
        if branch_id == p.head.ref:
            pull = p
            break

    if pull is None:
        body = """
        SUMMARY
        Automated pull request to preview zilliqa-developer/{}
        """.format(
            orig_branch
        )

        pull = repo.create_pull(
            title="Preview of zilliqa-developer:{}".format(orig_branch),
            body=body,
            head=branch_id,
            base="main",
        )

        # Creating comment
        current_pull = get_main_pull_request(github, orig_branch)
        current_pull.create_issue_comment(
            "A preview PR was openened at {}".format(pull.html_url)
        )

    has_preview = False
    for label in pull.get_labels():
        if label.name == "preview":
            has_preview = True
            break

    if not has_preview:
        pull.set_labels("preview")


def create_messages(github, pr_ref):
    # Getting list of changed files
    file_list_raw = subprocess.check_output(
        "git diff --name-only main", stderr=subprocess.STDOUT, shell=True
    ).decode("utf-8")
    file_list = file_list_raw.strip().split("\n")

    # Getting pull request object
    current_pull = get_main_pull_request(github, pr_ref)

    # Attempting to intepret updates and create messages from it
    messages = []
    for f in file_list:
        with open(f, "r") as fb:
            obj = yaml.safe_load(fb.read())

        application = "Unknown"
        if "applications/" in f:
            _, application = f.split("applications/", 1)
            if "/" in application:
                application, _ = application.split("/", 1)

        if "patches" in obj:
            for patch in obj["patches"]:
                lst = yaml.safe_load(patch["patch"])
                for x in lst:
                    if "path" in x and "value" in x and x["path"].endswith("/host"):
                        messages.append(
                            "Host updated to {} for application {}".format(
                                x["value"], application
                            )
                        )

    # Sending messages
    for m in messages:
        current_pull.create_issue_comment(m)


def main():
    # Preparing the Github interaction
    github = Github(os.environ["DEVOPS_ACCESS_TOKEN"])

    # Defining branch name
    stable_git_hash = version.stable_git_hash
    patches = sys.argv[1:-1]
    orig_branch = sys.argv[-1]
    branch_id = "zilliqa-developer/{}".format(sys.argv[-1])

    print("Branch: {}".format(branch_id))

    ## Getting the devops repo
    if not os.path.exists(".infra"):
        os.system("git clone git@github.com:Zilliqa/devops.git .infra")

    ## Checking out the branch
    print("Switching branch")
    os.chdir(".infra")

    os.system("git fetch --all")
    os.system("git pull --all")

    branch_output = subprocess.check_output(
        "git branch", stderr=subprocess.STDOUT, shell=True
    ).decode("utf-8")
    branches = [x.strip() for x in branch_output.split("\n") if x.strip() != ""]
    branches = [b[2:].strip() if b.startswith("* ") else b for b in branches]

    print("Branches available:", branches)
    # exit(0)

    try:
        print("Attempting to checking out the branch")
        subprocess.check_output("git checkout {}".format(branch_id), shell=True)
        subprocess.check_output("git pull", shell=True)
    except subprocess.CalledProcessError:
        print("Creating branch")
        subprocess.check_output("git checkout -b {}".format(branch_id), shell=True)

    # Applying patch
    print("Applying patch")
    for patch in patches:
        os.system("tar xvf ../{}".format(patch))

    # Pushing
    if not is_git_dirty("."):
        print("No changes made - not pushing")
    else:
        print("Pushing changes")
        os.system("git add . -A")
        os.system(
            'git commit -m "Preparing preview for commit: {}"'.format(stable_git_hash)
        )
        os.system("git push --set-upstream origin {}".format(branch_id))

        # Creating Devops PR
        create_pr(github, orig_branch, branch_id)

        # Commenting updates to main PR
        create_messages(github, orig_branch)


if __name__ == "__main__":
    main()
