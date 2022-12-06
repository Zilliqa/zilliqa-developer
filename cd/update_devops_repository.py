import os
import subprocess
import sys

import yaml
from github import Github

from cd import version


class PrepareDevops(object):
    def is_git_dirty(self, path):
        p = subprocess.Popen(["git", "status", "-s"], cwd=path, stdout=subprocess.PIPE)
        (out, err) = p.communicate()
        if p.returncode != 0:
            sys.exit(p.returncode)
        return out.decode("ascii").strip() != ""

    def is_production(self, pr_ref):
        # It is production if we are merging to a release branch
        return (
            pr_ref.startswith("release/")
            or pr_ref.startswith("pre-release/")
            or pr_ref == "!production"
        )

    def find_pr_from_head(self):
        pulls = self.developer_repo.get_pulls(
            state="open", sort="created", head=self.head_branch
        )

        n = 0
        for p in pulls:
            if p.head.ref == self.head_branch and (
                p.base.ref == "main" or "release/" in p.base.ref
            ):
                self.pr = p
                n += 1

        if not self.pr:
            print("Could not find PR {}".format(self.head_branch))
            return

        if n != 1:
            print(
                "This branch is being merged into several other branches - cannot create a preview."
            )
            return

    def find_pr_from_id(self, id):
        pulls = self.developer_repo.get_pulls(state="open", sort="created")
        for p in pulls:
            if p.number == id:
                self.pr = p

    def get_devops_repo(self):
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

        try:
            print("Attempting to checking out the branch")
            subprocess.check_output(
                "git checkout {}".format(self.devops_branch_id), shell=True
            )
            try:
                subprocess.check_output("git pull", shell=True)
            except subprocess.CalledProcessError:
                pass
        except subprocess.CalledProcessError:
            print("Creating branch")
            subprocess.check_output(
                "git checkout -b {}".format(self.devops_branch_id), shell=True
            )

    def apply_patches(self):
        assert os.getcwd().endswith(".infra")

        for patch in self.patches:
            os.system("tar xvf ../{}".format(patch))

    def __init__(self, patches, update_type, source):

        self.github = Github(os.environ["DEVOPS_ACCESS_TOKEN"])
        self.full_version = version.full_version

        self.developer_repo = self.github.get_repo("Zilliqa/zilliqa-developer")
        self.devops_repo = self.github.get_repo("Zilliqa/devops")

        self.patches = sys.argv[1:-2]
        self.type = update_type
        self.pr = None

        if self.type == "HEAD":
            self.head_branch = source
            self.find_pr_from_head()
        elif self.type == "PR":
            self.find_pr_from_id(int(source))
            if not self.pr:
                raise BaseException("PR not found.")
            self.head_branch = self.pr.head.ref
        else:
            raise BaseException(
                "Rollout takes either a head branch or a PR as argument. Got neither"
            )

        self.base_branch = None
        if self.pr:
            self.base_branch = self.pr.base.ref
        self.production = self.is_production(self.base_branch)

        if self.production:
            self.devops_branch_id = "zilliqa-developer/production/{}".format(
                self.head_branch
            )
        else:
            self.devops_branch_id = "zilliqa-developer/preview/{}".format(
                self.head_branch
            )

        print(self.github)
        print(self.full_version)
        print(self.developer_repo)
        print(self.devops_repo)
        print(self.patches)
        print(self.type)
        print(self.pr)
        print(self.head_branch)
        print(self.base_branch)
        print(self.production)
        print(self.devops_branch_id)

    def create_pr(self):
        repo = self.devops_repo
        self.devops_pr = None
        pulls = repo.get_pulls(state="open", sort="created", base="main")
        for p in pulls:
            if self.devops_branch_id == p.head.ref:
                self.devops_pr = p
                break

        if self.devops_pr is None:
            type = "preview"
            if self.production:
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
                    self.head_branch
                )
                title = "Preview of zilliqa-developer:{}".format(self.head_branch)

            print("- Creating PR")
            self.devops_pr = repo.create_pull(
                title=title,
                body=body,
                head=self.devops_branch_id,
                base="main",
            )

            # Creating comment on original PR if not production
            if self.pr:
                self.pr.create_issue_comment(
                    "A {} PR was openened at {}".format(type, self.devops_pr.html_url)
                )

        # Adding preview label if this not a production PR
        has_preview = False
        for label in self.devops_pr.get_labels():
            if label.name == "preview":
                has_preview = True
                break

        if not has_preview:
            self.devops_pr.set_labels("preview")

    def create_messages(self):
        assert os.getcwd().endswith(".infra")

        # Getting list of changed files
        file_list_raw = subprocess.check_output(
            "git diff --name-only main", stderr=subprocess.STDOUT, shell=True
        ).decode("utf-8")
        file_list = file_list_raw.strip().split("\n")

        # Attempting to intepret updates and create messages from it
        messages = []
        if self.production:
            messages = ["Production version {}".format(self.full_version)]
        else:
            messages = ["Preview version {}".format(self.full_version)]

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

        if self.devops_pr:
            self.devops_pr.create_issue_comment("\n\n".join(messages))

        if not self.pr:
            print("Messages could not be sent to PR:\n\n" + "\n\n".join(messages))
            return

        # Sending messages
        self.pr.create_issue_comment("\n\n".join(messages))

    def __call__(self):

        self.get_devops_repo()

        self.apply_patches()

        if not self.is_git_dirty("."):
            print("No changes made - not pushing")
        else:
            assert os.getcwd().endswith(".infra")
            print("Pushing changes")
            os.system("git add . -A")
            os.system(
                'git commit -m "Preparing preview for commit: {}"'.format(
                    self.full_version
                )
            )
            os.system("git push --set-upstream origin {}".format(self.devops_branch_id))

            # Creating Devops PR
            self.create_pr()

            # Commenting updates to main PR
            self.create_messages()


def main():
    patches = sys.argv[1:-2]
    type = sys.argv[-2]
    source = sys.argv[-1]

    op = PrepareDevops(patches, type, source)
    op()


if __name__ == "__main__":
    main()
