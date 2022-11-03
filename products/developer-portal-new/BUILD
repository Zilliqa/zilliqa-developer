load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//@docusaurus/core:index.bzl", "docusaurus")
load("@npm//http-server:index.bzl", "http_server")
load("@npm//sass:index.bzl", "sass")
load("@npm//webpack-cli:index.bzl", webpack = "webpack_cli")
load("@npm//tailwindcss:index.bzl", "tailwindcss")

load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")
load("@io_bazel_rules_docker//container:container.bzl",  "container_image")

filegroup(
	name = "src",
	srcs = glob(
		["**/*.ts",
		"**/*.tsx",
		 "**/*.css", 
		 "*.css"], 
		 exclude_directories = 0
	)
)

filegroup(
	name = "docs",
	srcs = glob(
		["**/*.md",
		"**/*.mdx",
], 
		 exclude_directories = 0
	)
)

docusaurus(
	name="start",
	chdir = "products/developer-portal-new",
	args = [
		"build",
	],
	data = [
#		"babel.config.js",
#		"docusaurus.config.js",
#		"sidebars.js",
#		"tsconfig.json",
#		":src",
		":docs",
	    "//:package.json",
	    "@npm//:node_modules",
	],
	outs = ["index.html"],
)