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
    out_dir = "dist",
    resolve_json_module = True,
    root_dir = "src",
    source_map = True,
    tsc = "@npm//typescript/bin:tsc",
    tsconfig = "tsconfig.json",
    visibility = ["//visibility:public"],
    deps = [
        "//zilliqa/js/util:lib",
        "@npm//@types/bn.js",
        "@npm//@types/camelcase",
        "@npm//@types/elliptic",
        "@npm//@types/jest",
        "@npm//@types/node",
        "@npm//@types/uuid",
        "@npm//@typescript-eslint/eslint-plugin",
        "@npm//@typescript-eslint/parser",
        "@npm//aes-js",
        "@npm//elliptic",
        "@npm//hash.js",
        "@npm//hmac-drbg",
        "@npm//pbkdf2",
        "@npm//scrypt-js",
        "@npm//sodium-randbytes",
        "@npm//uuid",
    ],
)

js_library(
    name = "lib",
    package_name = "@zilliqa-js/crypto",
    srcs = ["package.json"],
    visibility = ["//visibility:public"],
    deps = [
        ":src_js",
    ],
)

pkg_npm(
    name = "package",
    package_name = "@zilliqa-js/crypto",
    srcs = ["package.json"],
    visibility = ["//visibility:public"],
    deps = [":src_js"],
)
