load("@build_bazel_rules_nodejs//:index.bzl", "js_library", "pkg_npm")
load("@npm//@bazel/typescript:index.bzl", "ts_project")

ts_project(
    name = "src_js",
    srcs = [
        "//zilliqa/js:typings",
    ] + glob(
        [
            "src/*.ts",
            "src/**/*.ts",
        ],
        exclude = ["src/**/*.spec.ts"],
    ),
    composite = True,
    declaration = True,
    declaration_map = True,
    extends = "//:tsconfig.base.json",
    incremental = True,
    resolve_json_module = True,
    root_dir = "src",
    source_map = True,
    tsc = "@npm//typescript/bin:tsc",
    tsconfig = "tsconfig.json",
    visibility = ["//visibility:public"],
    deps = [
        "@npm//@types/bn.js",
        "@npm//@types/camelcase",
        "@npm//@types/jest",
        "@npm//@types/node",
        "@npm//@typescript-eslint/eslint-plugin",
        "@npm//@typescript-eslint/parser",
        "@npm//cross-fetch",
        "@npm//hash.js",  # TODO: Not needed - fix typings and remove
        "@npm//mitt",
        "@npm//tslib",
    ],
)

js_library(
    name = "lib",
    package_name = "@zilliqa-js/core",
    srcs = ["package.json"],
    visibility = ["//visibility:public"],
    deps = [
        ":src_js",
    ],
)

pkg_npm(
    name = "package",
    package_name = "@zilliqa-js/core",
    srcs = ["package.json"],
    visibility = ["//visibility:public"],
    deps = [":src_js"],
)
