load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@build_bazel_rules_nodejs//:index.bzl", "pkg_npm")

ts_project(
   name = "util_ts",
   srcs = [
   	"//zilliqa/typescript:typings"
   ] + glob(["src/*.ts", "src/**/*.ts"], exclude = ["src/**/*.spec.ts"]),
   extends = "//:tsconfig.base.json",
   tsconfig = "tsconfig.json",
   out_dir = "build",
   root_dir = "src",
   tsc = "@npm//typescript/bin:tsc",
   deps = [
	    "@npm//@types/long",
	    "@npm//bn.js",
	    "@npm//long",
	    "@npm//tslib",

	    "@npm//@types/bn.js",
	    "@npm//@types/camelcase",
	    "@npm//@types/fancy-log",
	    "@npm//@types/jest",
	    "@npm//@types/node",
	    "@npm//@types/webpack",
	    "@npm//@typescript-eslint/eslint-plugin",
	    "@npm//@typescript-eslint/parser",
   ],

	declaration_map=1,
	resolve_json_module=1,
	source_map=1,
	composite=1,
	declaration=1,
	incremental=1,
)

# TODO:
# pkg_npm(
#    name = "core",
#    package_name = "@zilliqa-js/core",
#    srcs = ["package.json"],
#    deps = [":core_ts"],
# )