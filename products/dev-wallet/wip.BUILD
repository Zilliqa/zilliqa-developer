load("@build_bazel_rules_nodejs//:index.bzl", "copy_to_bin")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")
load("@npm//webpack-cli:index.bzl", webpack = "webpack_cli")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load(":differential_loading.bzl", "differential_loading")

ts_project(
    name = "ts_source",
    srcs = glob(
        [
            "src/**/*",
            "src/*",
        ],
        exclude = [
            "src/**/*.spec.*",
            "src/*.spec.*",
            "src/**/*.test.*",
            "src/*.test.*",
        ],
    ),
    allow_js = True,
    composite = True,
    data = glob([
        "src/**/*.css",
        "src/*.css",
    ]),
    declaration = True,
    declaration_map = False,
    extends = "//:tsconfig.base.json",
    incremental = True,
    out_dir = "src",  # // TODO: This is not clean - outdir should be "dist"
    resolve_json_module = True,
    root_dir = "src",
    source_map = False,
    tsc = "@npm//typescript/bin:tsc",
    tsconfig = "tsconfig.json",
    deps = [
        "//zilliqa/js/zilliqa:lib",
        "@npm//@testing-library/jest-dom",
        "@npm//@testing-library/react",
        "@npm//@testing-library/user-event",
        "@npm//@types",
        "@npm//@types/jest",
        "@npm//@types/node",
        "@npm//@types/react",
        "@npm//@types/react-dom",
        "@npm//bootstrap",
        "@npm//csstype",
        "@npm//husky",
        "@npm//prettier",
        "@npm//rc-steps",
        "@npm//react",
        "@npm//react-app-rewired",
        "@npm//react-dom",
        "@npm//react-google-recaptcha",
        "@npm//react-hooks-worker",
        "@npm//react-icons",
        "@npm//react-jazzicon",
        "@npm//react-router-dom",
        "@npm//react-scripts",
        "@npm//reactstrap",
        "@npm//styled-components",
        "@npm//ts-jest",
        "@npm//typescript",
        "@npm//uuid",
        "@npm//web-vitals",
        "@npm//whatwg-fetch",
        "@npm//worker-plugin",
    ],
)

differential_loading(
    name = "app",
    srcs = glob(
        [
            "src/**/*",
            "src/*",
        ],
        exclude = [
            "src/**/*.spec.*",
            "src/*.spec.*",
            "src/**/*.test.*",
            "src/*.test.*",
        ],
    ),
    entry_point = "src/index.js",
    deps = [":ts_source"],
)

http_server(
    name = "server",
    data = [":app"],
    templated_args = ["app"],
)

copy_to_bin(
    name = "styles",
    srcs = glob([
        "src/**/*.css",
        "src/*.css",
    ]),
)

filegroup(
    name = "ts_source.js",
    srcs = [
        ":ts_source",
    ],
    output_group = "es6_sources",
    visibility = ["//visibility:public"],
)

webpack(
    name = "bundle",
    outs = ["app.bundle.js"],
    args = [
        "$(locations :ts_source)",  # "$(execpath index.js)",
        "--config",
        "$(execpath webpack.config.js)",
        "-o",
        "$@",
    ],
    data = [
        "webpack.config.js",
        ":src/index.js",
        ":styles",
        ":ts_source",
        "@npm//:node_modules",
    ],
)

pkg_tar(
    name = "wallet-content",
    srcs = [
        ":bundle",
        "index.html",
        ":index.js",
        #        ":styles.css",
        ":tailwind.css",
    ],
    mode = "0755",
    package_dir = "/usr/share/nginx/html/",
    deps = [],
)

pkg_tar(
    name = "wallet-config",
    srcs = ["nginx/default.conf"],
    mode = "0755",
    package_dir = "/etc/nginx/conf.d/",
    deps = [],
)

container_image(
    name = "image",
    base = "@nginx//image",
    ports = ["80"],
    tars = [
        ":wallet-config",
        ":wallet-content",
    ],
)

http_server(
    name = "serve",
    chdir = package_name(),
    data = [
        ":app.bundle.js",
        ":config.json",
        ":index.html",
    ],
    templated_args = [
        ".",
    ],
)
