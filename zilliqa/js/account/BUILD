load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@aspect_rules_js//npm:defs.bzl", "npm_package", "stamped_package_json")
load("@aspect_rules_ts//ts:defs.bzl", "ts_project")
load("@jest//:defs.bzl", "jest_test")

stamped_package_json(
    name = "package",
    stamp_var = "STABLE_JS_API_BUILD_VERSION",
)

_SRCS = [
    ":package",
    "//zilliqa/js:aes-js.d",
] + glob(
    [
        "src/*.ts",
        "src/**/*.ts",
    ],
    exclude = ["src/**/*.spec.ts"],
)

copy_to_bin(
    name = "sources",
    srcs = _SRCS,
)

ts_project(
    name = "lib",
    srcs = _SRCS,
    composite = True,
    data = [
        ":sources",
        "//:node_modules/@typescript-eslint/eslint-plugin",
        "//:node_modules/@typescript-eslint/parser",
        "//:node_modules/@zilliqa-js/core",
        "//:node_modules/@zilliqa-js/crypto",
        "//:node_modules/@zilliqa-js/proto",
        "//:node_modules/@zilliqa-js/util",
        "//:node_modules/bip39",
        "//:node_modules/hash.js",
        "//:node_modules/hdkey",
        "//:node_modules/tslib",
    ],
    declaration = True,
    declaration_map = True,
    extends = "//:tsconfig.base",
    incremental = True,
    resolve_json_module = True,
    root_dir = "src",
    source_map = True,
    tsconfig = "tsconfig.json",
    visibility = ["//visibility:public"],
    deps = [
        "//:node_modules/@types/bip39",
        "//:node_modules/@types/bn.js",
        "//:node_modules/@types/camelcase",
        "//:node_modules/@types/hdkey",
        "//:node_modules/@types/long",
        "//:node_modules/@types/node",
        "//:node_modules/@typescript-eslint/eslint-plugin",
        "//:node_modules/@typescript-eslint/parser",
        "//:node_modules/@zilliqa-js/core",
        "//:node_modules/@zilliqa-js/crypto",
        "//:node_modules/@zilliqa-js/proto",
        "//:node_modules/@zilliqa-js/util",
        "//:node_modules/bip39",
        "//:node_modules/hash.js",
        "//:node_modules/hdkey",
        "//:node_modules/protobufjs",  # TODO: This one should come automatically from proto
        "//:node_modules/tslib",
    ],
)

npm_package(
    name = "pkg",
    srcs = [":lib"],
    package = "@zilliqa-js/account",
    visibility = ["//visibility:public"],
)

ts_project(
    name = "test_src",
    srcs = [
        "//zilliqa/js:aes-js.d",
    ] + glob(
        [
            "src/*.ts",
            "src/**/*.ts",
            "test/*.ts",
            "test/**/*.ts",
        ],
    ),
    composite = True,
    data = [],
    declaration = True,
    declaration_map = True,
    extends = "//:tsconfig.test",
    incremental = True,
    resolve_json_module = True,
    source_map = True,
    tsconfig = "tsconfig.test.json",
    validate = False,
    deps = [
        "//:node_modules/@jest/expect",
        "//:node_modules/@jest/expect-utils",
        "//:node_modules/@testing-library/jest-dom",
        "//:node_modules/@testing-library/react",
        "//:node_modules/@types/bip39",
        "//:node_modules/@types/bn.js",
        "//:node_modules/@types/camelcase",
        "//:node_modules/@types/hdkey",
        "//:node_modules/@types/jest",
        "//:node_modules/@types/long",
        "//:node_modules/@types/node",
        "//:node_modules/@typescript-eslint/eslint-plugin",
        "//:node_modules/@typescript-eslint/parser",
        "//:node_modules/@zilliqa-js/core",
        "//:node_modules/@zilliqa-js/crypto",
        "//:node_modules/@zilliqa-js/proto",
        "//:node_modules/@zilliqa-js/util",
        "//:node_modules/ansi-styles",
        "//:node_modules/bip39",
        "//:node_modules/bn.js",
        "//:node_modules/buffer-from",
        "//:node_modules/cross-fetch",
        "//:node_modules/expect",
        "//:node_modules/hash.js",
        "//:node_modules/hdkey",
        "//:node_modules/jest-fetch-mock",
        "//:node_modules/tslib",
    ],
)

jest_test(
    name = "test",
    config = "//:jest_config",
    data = [
        ":test_src",
        "//:jest-setup",
        "//:node_modules/@types/long",
        "//:node_modules/jest-environment-jsdom",
        "//:node_modules/jest-fetch-mock",
    ],
)
