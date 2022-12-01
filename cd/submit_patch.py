import os
import subprocess
import sys

from github import Github

from cd import version


def is_git_dirty(path):
    p = subprocess.Popen(["git", "status", "-s"], cwd=path, stdout=subprocess.PIPE)
    (out, err) = p.communicate()
    if p.returncode != 0:
        sys.exit(p.returncode)
    return out.decode("ascii").strip() != ""


def main():
    # Preparing the Github interaction
    github = Github(os.environ["DEVOPS_ACCESS_TOKEN"])
    repo = github.get_repo("Zilliqa/devops")

    # Defining branch name
    stable_git_hash = version.stable_git_hash
    patches = sys.argv[1:-1]
    orig_branch = sys.argv[-1]
    branch_id = "zilliqa-developer/{}".format(sys.argv[-1])

    print("Branch: {}".format(branch_id))

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

        github = Github(os.environ["DEVOPS_ACCESS_TOKEN"])
        repo = github.get_repo("Zilliqa/devops")

        pulls = repo.get_pulls(state="open", sort="created", base="main")
        pull_branches = []
        for p in pulls:
            pull_branches.append(p.head.ref)

        if branch_id not in pull_branches:
            body = """
            SUMMARY
            Automated pull request to preview zilliqa-developer/{}
            """.format(
                orig_branch
            )
            repo.create_pull(
                title="Preview of zilliqa-developer:{}".format(orig_branch),
                body=body,
                head=branch_id,
                base="main",
            )


if __name__ == "__main__":
    main()
