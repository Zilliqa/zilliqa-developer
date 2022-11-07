load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//http-server:index.bzl", "http_server")
load("@npm//sass:index.bzl", "sass")
load("@npm//webpack-cli:index.bzl", webpack = "webpack_cli")
load("@npm//tailwindcss:index.bzl", "tailwindcss")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")
load("@io_bazel_rules_docker//container:container.bzl",  "container_image")


ts_project(
    name = "ts_source",    
    srcs = glob(["src/**/*", "src/*"], exclude=["src/**/*.spec.*", "src/*.spec.*","src/**/*.test.*", "src/*.test.*"]),
    extends = "//:tsconfig.base.json",
    tsconfig = "tsconfig.json",
    out_dir = "dist",
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
    data = [],

 	allow_js=True,
 	declaration_map=True,
 	resolve_json_module=True,
 	source_map=True,
 	composite=True,
 	declaration=True,
 	incremental=True,
    # Experimental: Start a tsc daemon to watch for changes to make recompiles faster.
    # supports_workers = True,
    # If activated TSC stops working
#    source_map = True,  
#    preserve_jsx = True,
#    out_dir = "ts_source",
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
        ":dist/index.js",
#        ":dist/styles.css",
        "webpack.config.js",
        ":ts_source",

        "@npm//:node_modules",
        "//zilliqa/js/zilliqa:lib"        
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

# Add --platforms=@io_bazel_rules_go//go/toolchain:linux_amd64
# or --platforms=@//:linux-x86
container_image(
    name="image",
    base="@nginx//image",
    tars=[":wallet-config", ":wallet-content"],
    ports=["80"],
)