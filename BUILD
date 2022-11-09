load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@npm//:defs.bzl", "npm_link_all_packages")

package(default_visibility = ["//visibility:public"])

# This macro expands to a link_npm_package for each third-party package in package.json
npm_link_all_packages(name = "node_modules")

exports_files([
    "package.json",
    "tsconfig.base.json",
])

exports_files(["docs"])

copy_to_bin(
    name = "tsconfig",
    srcs = ["tsconfig.json"],
)

copy_to_bin(
    name = "tsconfig.base",
    srcs = ["tsconfig.base.json"],
    visibility = ["//zilliqa/js/util:__subpackages__"],
)
