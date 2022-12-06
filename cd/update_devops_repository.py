import os
import subprocess
import sys

import yaml
from github import Github

from cd import version


def is_production(pr_ref):
    # It is production if we are merging to a release branch
    return (
        pr_ref.startswith("release/")
        or pr_ref.startswith("pre-release/")
        or pr_ref == "!production"
    )


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

    n = 0
    for p in pulls:
        if p.head.ref == pr_ref and (p.base.ref == "main" or "release/" in p.base.ref):
            current_pull = p
            n += 1

    if not current_pull:
        print("Could not find PR {}".format(pr_ref))
        return None

    if n != 1:
        print(
            "This branch is being merged into several other branches - cannot create a preview."
        )
        return None

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
        type = "preview"
        if is_production(orig_branch):
            type = "production"
            body = """
            SUMMARY
            Automated production pull request
            """
            title = "Production roll out of zilliqa-developer"
        else:
            body = """
            SUMMARY
            Automated pull request to preview zilliqa-developer/{}
            """.format(
                orig_branch
            )
            title = "Preview of zilliqa-developer:{}".format(orig_branch)

        print("- Creating PR")
        pull = repo.create_pull(
            title=title,
            body=body,
            head=branch_id,
            base="main",
        )

        # Creating comment on original PR if not production
        current_pull = get_main_pull_request(github, orig_branch)
        if current_pull:
            current_pull.create_issue_comment(
                "A {} PR was openened at {}".format(type, pull.html_url)
            )

    # Adding preview label if this not a production PR
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

    # Attempting to intepret updates and create messages from it
    messages = []
    if is_production(pr_ref):
        messages = ["Production version {}".format(version.full_version)]
    else:
        messages = ["Preview version {}".format(version.full_version)]

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
                            "Host updated to [{}](https://{}) for application {}".format(
                                x["value"], x["value"], application
                            )
                        )

    # Getting pull request object
    current_pull = get_main_pull_request(github, pr_ref)

    if not current_pull:
        print("Messages could not be sent to PR:\n\n" + "\n\n".join(messages))
        return

    # Sending messages
    current_pull.create_issue_comment("\n\n".join(messages))


def main():
    # Preparing the Github interaction
    github = Github(os.environ["DEVOPS_ACCESS_TOKEN"])

    # Defining branch name
    full_version = version.full_version
    patches = sys.argv[1:-1]
    orig_branch = sys.argv[-1]

    if is_production(orig_branch):
        branch_id = "zilliqa-developer/production/{}".format(sys.argv[-1])
    else:
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
            'git commit -m "Preparing preview for commit: {}"'.format(full_version)
        )
        os.system("git push --set-upstream origin {}".format(branch_id))

        # Creating Devops PR
        create_pr(github, orig_branch, branch_id)

        # Commenting updates to main PR
        create_messages(github, orig_branch)


if __name__ == "__main__":
    main()
