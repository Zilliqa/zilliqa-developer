load("@npm//@bazel/typescript:index.bzl", "ts_project")
load("@npm//@docusaurus/core:index.bzl", "docusaurus")
load("@npm//http-server:index.bzl", "http_server")
load("@npm//sass:index.bzl", "sass")
load("@npm//webpack-cli:index.bzl", webpack = "webpack_cli")
load("@npm//tailwindcss:index.bzl", "tailwindcss")

load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")
load("@io_bazel_rules_docker//container:container.bzl",  "container_image")


tailwindcss(
    name = "tailwindcss",
    args = [
        "-i",
        "$(execpath tailwind.in.css)",
        "-c",
        "$(execpath tailwind.config.js)",        
        "-o",
        "$(execpath tailwind.css)",
    ],
    data = [
        "tailwind.config.js",
        "tailwind.in.css",
    ] + glob(["*.tsx", "**/*.tsx"]),
    outs = ["tailwind.css"]
)

docusaurus(
	name="start",
	args = [
		"build",
		"--config",
		"$(execpath docusaurus.config.js)",
		"--no-minify" 
	],
	data = [
		# Actually documentation source
		"//docs:files",

		# Loadable packages
		"@npm//@docusaurus/preset-classic",
		"@npm//clsx",
		"@npm//markdown-link-check",
		"@npm//react",
		"@npm//react-dom",
		"@npm//react-loadable",
	] + glob(
		["**/*.js",
		 "*.js",		  
		 "**/*.css", 
		 "*.css"], 
		 exclude_directories = 0
	),
	outs = ["index.html"],
)