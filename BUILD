load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@aspect_rules_js//js:defs.bzl", "js_library")
load("@aspect_rules_js//npm:defs.bzl", "npm_link_package")
load("@npm//:defs.bzl", "npm_link_all_packages")

package(default_visibility = ["//visibility:public"])

# This macro expands to a link_npm_package for each third-party package in package.json
npm_link_package(
    name = "node_modules/@zilliqa-js/account",
    src = "//zilliqa/js/account:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/blockchain",
    src = "//zilliqa/js/blockchain:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/contract",
    src = "//zilliqa/js/contract:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/core",
    src = "//zilliqa/js/core:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/crypto",
    src = "//zilliqa/js/crypto:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/proto",
    src = "//zilliqa/js/proto:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/subscriptions",
    src = "//zilliqa/js/subscriptions:pkg",
    visibility = ["//visibility:public"],
)

npm_link_package(
    name = "node_modules/@zilliqa-js/util",
    src = "//zilliqa/js/util:pkg",
    visibility = ["//visibility:public"],
)

npm_link_all_packages(name = "node_modules")

exports_files([
    "package.json",
    "tsconfig.base.json",
    "tsconfig.test.json",
])

exports_files(["docs"])

copy_to_bin(
    name = "package",
    srcs = ["package.json"],
)

copy_to_bin(
    name = "tsconfig",
    srcs = ["tsconfig.json"],
)

copy_to_bin(
    name = "tsconfig.base",
    srcs = ["tsconfig.base.json"],
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "tsconfig.test",
    srcs = [
        "tsconfig.base.json",
        "tsconfig.test.json",
    ],
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "jest-setup",
    srcs = ["jest-setup.js"],
    visibility = ["//visibility:public"],
)

js_library(
    name = "jest_config",
    srcs = ["jest.config.js"],
    visibility = ["//visibility:public"],
)
