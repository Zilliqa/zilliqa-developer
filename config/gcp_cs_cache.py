#!/usr/bin/python

import argparse
import os

parser = argparse.ArgumentParser(description="Configure Bazel remote cache.")
parser.add_argument("--endpoint", type=str, help="Remote cache endpoint URL")
parser.add_argument("--credentials", type=str, help="Path to JSON secret key file")
args = parser.parse_args()

# Get values from command-line arguments or environment variables
ENDPOINT = args.endpoint or os.environ.get("BAZEL_REMOTE_CACHE_ENDPOINT")

CREDENTIALS_JSON = args.credentials or os.environ.get(
    "BAZEL_REMOTE_CACHE_CREDENTIALS_JSON"
)

with open(".bazelrc.configure", "w") as fb:
    fb.write("build --remote_cache={}\n".format(ENDPOINT))
    fb.write("build --google_credentials={}\n".format(CREDENTIALS_JSON))
