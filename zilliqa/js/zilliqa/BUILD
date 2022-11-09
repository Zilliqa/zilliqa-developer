load("@aspect_rules_js//npm:defs.bzl", "npm_package", "stamped_package_json")
load("@aspect_rules_ts//ts:defs.bzl", "ts_project")

stamped_package_json(
    name = "package",
    stamp_var = "STABLE_JS_API_BUILD_VERSION",
)

ts_project(
    name = "lib",
    srcs = [
        "//zilliqa/js:aes-js.d",
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
    extends = "//:tsconfig.base",
    incremental = True,
    resolve_json_module = True,
    root_dir = "src",
    source_map = True,
    tsconfig = "tsconfig.json",
    visibility = ["//visibility:public"],
    deps = [
        "//:node_modules/@types/bn.js",
        "//:node_modules/@types/camelcase",
        "//:node_modules/@types/node",
        "//:node_modules/@typescript-eslint/eslint-plugin",
        "//:node_modules/@typescript-eslint/parser",
        "//:node_modules/@zilliqa-js/account",
        "//:node_modules/@zilliqa-js/blockchain",
        "//:node_modules/@zilliqa-js/contract",
        "//:node_modules/@zilliqa-js/core",
        "//:node_modules/@zilliqa-js/crypto",
        "//:node_modules/@zilliqa-js/proto",
        "//:node_modules/@zilliqa-js/subscriptions",
        "//:node_modules/@zilliqa-js/util",
        "//:node_modules/hash.js",  # TODO: Not needed - fix typings and remove
    ],
)

npm_package(
    name = "pkg",
    srcs = [":lib"],
    package = "@zilliqa-js/zilliqa",
    visibility = ["//visibility:public"],
)
