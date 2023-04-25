#!/usr/bin/env python3
# the bazel container image rules end up running docker in a deleted
# temporary directory. Sadly, podman objects to this, so change directory
# before we re-exec docker and set DOCKER to the location of this file.
# ugh!
# Sadly, this has to be a python script, because trunk invokes shellcheck,
# which is so fascist that it's nearly impossible to satisfy it.

import os
import sys

try:
    exists = os.getcwd()
except Exception:
    # nope!
    os.chdir("/tmp")
os.execvp("/usr/bin/docker", sys.argv)
