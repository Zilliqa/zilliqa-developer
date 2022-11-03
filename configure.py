#!/usr/bin/python3
# Configure is based on the TensorFlow configure file
import argparse
import errno
import os
import platform
import re
import subprocess
import sys

try:
    from shutil import which
except ImportError:
    from distutils.spawn import find_executable as which

_BAZELRC_FILENAME = ".bazelrc.configure"


def is_windows():
    return platform.system() == "Windows"


def is_linux():
    return platform.system() == "Linux"


def is_macos():
    return platform.system() == "Darwin"


def is_ppc64le():
    return platform.machine() == "ppc64le"


def is_cygwin():
    return platform.system().startswith("CYGWIN_NT")


def symlink_force(target, link_name):
    try:
        os.symlink(target, link_name)
    except OSError as e:
        if e.errno == errno.EEXIST:
            os.remove(link_name)
            os.symlink(target, link_name)
        else:
            raise e


def get_python_path(environ_cp, python_bin_path):
    """Get the python site package paths."""
    python_paths = []
    if environ_cp.get("PYTHONPATH"):
        python_paths = environ_cp.get("PYTHONPATH").split(":")
    try:
        stderr = open(os.devnull, "wb")
        library_paths = run_shell(
            [
                python_bin_path,
                "-c",
                'import site; print("\\n".join(site.getsitepackages()))',
            ],
            stderr=stderr,
        ).split("\n")
    except subprocess.CalledProcessError:
        library_paths = [
            run_shell(
                [
                    python_bin_path,
                    "-c",
                    "from distutils.sysconfig import get_python_lib;"
                    "print(get_python_lib())",
                ]
            )
        ]

    all_paths = set(python_paths + library_paths)

    paths = []
    for path in all_paths:
        if os.path.isdir(path):
            paths.append(path)
    return paths


def run_shell(cmd, allow_non_zero=False, stderr=None):
    if stderr is None:
        stderr = sys.stdout
    if allow_non_zero:
        try:
            output = subprocess.check_output(cmd, stderr=stderr)
        except subprocess.CalledProcessError as e:
            output = e.output
    else:
        output = subprocess.check_output(cmd, stderr=stderr)
    return output.decode("UTF-8").strip()


def write_to_bazelrc(line):
    with open(_BAZELRC_FILENAME, "a") as f:
        f.write(line + "\n")


def write_action_env_to_bazelrc(var_name, var):
    write_to_bazelrc('build --action_env {}="{}"'.format(var_name, str(var)))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--no-input",
        type=str,
        default=os.path.abspath(os.path.dirname(__file__)),
        help="Tells configure to not ask for any inputs.",
    )
    args = parser.parse_args()
