load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@build_bazel_rules_nodejs//:index.bzl", "pkg_npm")

ts_project(
   name = "core_ts",
   srcs = [
   	"//:tsconfig.base.json",
   	"//zilliqa/typescript:typings"
   ] + glob(["src/*.ts", "src/**/*.ts"], exclude = ["src/**/*.spec.ts"]),
   out_dir = "build",
   root_dir = "src",
   tsc = "@npm//typescript/bin:tsc",
   deps = [
        "@npm//cross-fetch",
        "@npm//mitt",
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