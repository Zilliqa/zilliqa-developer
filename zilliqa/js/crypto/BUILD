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
    "@npm//aes-js",
    "@npm//bsert",
    "@npm//elliptic",
    "@npm//hash.js",
    "@npm//hmac-drbg",
    "@npm//pbkdf2",
    "@npm//scrypt-js",
    "@npm//scryptsy",
    "@npm//sodium-randbytes",
    "@npm//tslib",
    "@npm//uuid",


	    "@npm//@types/bn.js",
	    "@npm//@types/camelcase",
	    "@npm//@types/fancy-log",
	    "@npm//@types/jest",
	    "@npm//@types/node",
	    "@npm//@types/webpack",
        "@npm//@types/elliptic",
        "@npm//@types/uuid",

	    "@npm//@typescript-eslint/eslint-plugin",
	    "@npm//@typescript-eslint/parser",

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
    package_name = "@zilliqa-js/crypto",
    deps = [
        ":src_js",
    ],
 	visibility = ["//visibility:public"],
    srcs = ["package.json"],
)

pkg_npm(
    name = "package",
    package_name = "@zilliqa-js/crypto",
    srcs = ["package.json"],
    deps = [":src_js"],
 	 visibility = ["//visibility:public"]    
)