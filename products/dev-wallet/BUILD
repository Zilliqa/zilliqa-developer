load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")
load("@npm//sass:index.bzl", "sass")
load("@npm//webpack-cli:index.bzl", webpack = "webpack_cli")
load("@npm//tailwindcss:index.bzl", "tailwindcss")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")
load("@io_bazel_rules_docker//container:container.bzl",  "container_image")
load("@build_bazel_rules_nodejs//:index.bzl", "copy_to_bin")
load(":differential_loading.bzl", "differential_loading")

ts_project(
    name = "ts_source",    
    srcs = glob(["src/**/*", "src/*"], exclude=["src/**/*.spec.*", "src/*.spec.*","src/**/*.test.*", "src/*.test.*"]),
    extends = "//:tsconfig.base.json",
    tsconfig = "tsconfig.json",
    out_dir = "src", # // TODO: This is not clean - outdir should be "dist"
    root_dir = "src",
    tsc = "@npm//typescript/bin:tsc",
    deps = [
	    "@npm//@testing-library/jest-dom",
	    "@npm//@testing-library/react",
	    "@npm//@testing-library/user-event",
	    "@npm//@types/jest",
	    "@npm//@types/node",
	    "@npm//@types/react",
	    "@npm//@types/react-dom",
	    "@npm//bootstrap",
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

        "@npm//@types",
        "@npm//csstype",

        "//zilliqa/js/zilliqa:lib"
    ],
    data = glob(["src/**/*.css", "src/*.css"]),

    declaration_map=False,
    source_map=False,    

 	allow_js=True,
 	resolve_json_module=True,
 	composite=True,
 	declaration=True,
 	incremental=True,
)


differential_loading(
    name = "app",
    srcs =glob(["src/**/*", "src/*"], exclude=["src/**/*.spec.*", "src/*.spec.*","src/**/*.test.*", "src/*.test.*"]),
    entry_point = "src/index.js",
    deps =[":ts_source"],
)

http_server(
    name = "server",
    data = [":app"],
    templated_args = ["app"],
)


copy_to_bin(
 name = "styles",
 srcs = glob(["src/**/*.css", "src/*.css"])
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
        "$(locations :ts_source)", # "$(execpath index.js)",
        "--config",
        "$(execpath webpack.config.js)",
        "-o",
        "$@",
    ],
    data = [
        ":src/index.js",
        "webpack.config.js",
        ":ts_source",
        ":styles",
        "@npm//:node_modules",
    ]
)


pkg_tar(
    name="wallet-content",
    package_dir="/usr/share/nginx/html/",
    srcs=[":bundle",
        "index.html",
        ":index.js",
#        ":styles.css",
        ":tailwind.css",],
    mode="0755",
    deps=[]
)

pkg_tar(
    name="wallet-config",
    package_dir="/etc/nginx/conf.d/",
    srcs=glob(["nginx/default.conf"]),
    mode="0755",
    deps=[]
)


container_image(
    name="image",
    base="@nginx//image",
    tars=[":wallet-config", ":wallet-content"],
    ports=["80"],
)

http_server(
    name = "serve",
    chdir = package_name(),
    data = [
        ":app.bundle.js",
        ":index.html",
        ":config.json",

    ],
    templated_args = [
        "."
    ],
)