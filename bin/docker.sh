#!/bin/sh
# the bazel container image rules end up running docker in a deleted
# temporary directory. Sadly, podman objects to this, so change directory
# before we re-exec docker and set DOCKER to the location of this file.
# ugh!
$(pwd) >/dev/null 2>&1
if [ $? -ne 0 ]; then
	cd /tmp
fi
/usr/bin/docker $*
