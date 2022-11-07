load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@build_bazel_rules_nodejs//:index.bzl", "pkg_npm")
load("@build_bazel_rules_nodejs//:index.bzl", "js_library")

ts_project(
   name = "src_js",
   srcs = [
   	"//zilliqa/js:typings"
   ] + glob(["src/*.ts", "src/**/*.ts"], exclude = ["src/**/*.spec.ts"]),
   extends = "//:tsconfig.base.json",
   tsconfig = "tsconfig.json",
   out_dir = "dist",
   root_dir = "src",
   tsc = "@npm//typescript/bin:tsc",
   deps = [
    "@npm//tslib",

	    "@npm//@types/bn.js",
	    "@npm//@types/camelcase",
	    "@npm//@types/fancy-log",
	    "@npm//@types/jest",
	    "@npm//@types/node",
	    "@npm//@types/webpack",
	    "@npm//@typescript-eslint/eslint-plugin",
	    "@npm//@typescript-eslint/parser",
        "@npm//hash.js", # TODO: Not needed - fix typings and remove

    "//zilliqa/js/account:lib",
    "//zilliqa/js/blockchain:lib",
    "//zilliqa/js/contract:lib",
    "//zilliqa/js/core:lib",
    "//zilliqa/js/proto:lib",    
    "//zilliqa/js/crypto:lib",
    "//zilliqa/js/subscriptions:lib",
    "//zilliqa/js/util:lib",   
    ],
   
	declaration_map = True,
	resolve_json_module = True,
	source_map = True,
	composite = True,
	declaration = True,
	incremental = True,
 	visibility = ["//visibility:public"]    
)

js_library(
    name = "lib",
    package_name = "@zilliqa-js/zilliqa",
    deps = [
        ":src_js",
    ],
 	visibility = ["//visibility:public"],
    srcs = ["package.json"],
)

pkg_npm(
    name = "package",
    package_name = "@zilliqa-js/zilliqa",
    srcs = ["package.json"],
    deps = [":src_js"],
 	 visibility = ["//visibility:public"]    
)