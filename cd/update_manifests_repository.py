import os
import shutil
import subprocess
import sys

from cd import version


class DeploymentUpdate:
    def __init__(self, files):
        self.files = files
        self.infra_repo = "zilliqa-internal-manifests"
        self.full_version = version.full_version

    def run(self, command, *args, **kwargs):
        print(">", command)
        sys.stdout.flush()
        sys.stderr.flush()
        os.system(command, *args, **kwargs)

    def is_git_dirty(self, path):
        p = subprocess.Popen(["git", "status", "-s"], cwd=path, stdout=subprocess.PIPE)
        (out, err) = p.communicate()
        if p.returncode != 0:
            sys.exit(p.returncode)
        return out.decode("ascii").strip() != ""

    def checkout_infa_repo(self):
        print("* Checkout infra structure repository")
        sys.stdout.flush()
        sys.stderr.flush()

        if os.path.exists(".infra"):
            shutil.rmtree(".infra")

        # Creating fresh checkout
        self.run("git clone git@github.com:Zilliqa/{} .infra".format(self.infra_repo))

        # Updating to lates
        os.chdir(".infra")

    def patch_repo(self):
        print("* Patching repository")
        sys.stdout.flush()
        sys.stderr.flush()

        assert os.getcwd().endswith(".infra")

        for patch in self.files:
            self.run("tar xf ../{}".format(patch))

    def commit_and_push(self):
        print("* Committing and pushing")
        sys.stdout.flush()
        sys.stderr.flush()

        if not self.is_git_dirty("."):
            print("No changes made - not pushing")
        else:
            assert os.getcwd().endswith(".infra")
            self.run("git add . -A")
            self.run(
                'git commit -m "Preparing staging for commit: {}"'.format(
                    self.full_version
                )
            )
            self.run("git push --set-upstream origin main")

        # Returning
        os.chdir("..")


def main():
    deployment = DeploymentUpdate(sys.argv[1:])
    deployment.checkout_infa_repo()
    deployment.patch_repo()
    deployment.commit_and_push()


if __name__ == "__main__":
    main()
