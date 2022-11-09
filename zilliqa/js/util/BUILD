load("@aspect_rules_js//npm:defs.bzl", "npm_package", "stamped_package_json")
load("@aspect_rules_ts//ts:defs.bzl", "ts_project")

#    package_name = "@zilliqa-js/util",
stamped_package_json(
    name = "package",
    # This key is defined by /workspace_status.sh
    stamp_var = "STABLE_BUILD_VERSION",
)

ts_project(
    name = "src_js",
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
    out_dir = "dist",
    resolve_json_module = True,
    root_dir = "src",
    source_map = True,
    tsconfig = "tsconfig.json",
    visibility = ["//visibility:public"],
    deps = [
        "//:node_modules/@types/bn.js",
        "//:node_modules/@types/camelcase",
        "//:node_modules/@types/long",
        "//:node_modules/@types/node",
        "//:node_modules/@typescript-eslint/eslint-plugin",
        "//:node_modules/@typescript-eslint/parser",
        "//:node_modules/bn.js",
        "//:node_modules/hash.js",  # TODO: Not needed - fix typings and remove
        "//:node_modules/long",
        "//:node_modules/tslib",
    ],
)

npm_package(
    name = "pkg",
    srcs = [":src_js"],
    package = "@zilliqa-js/util",
    # The package name does not need to be specified in this target since the name
    # that this package is linked as is set by the pnpm lockfile. If it is
    # specified here then it won't be load bearing since the value in the pnpm
    # lockfile will take precendence.
    visibility = ["//visibility:public"],
)
