load("@npm//@docusaurus/core:index.bzl", "docusaurus")
load("@io_bazel_rules_docker//container:container.bzl",  "container_image")
load("@build_bazel_rules_nodejs//:index.bzl", "copy_to_bin")
load("@bazel_skylib//rules:copy_directory.bzl", "copy_directory")

# To keep the docs cleanly separated from the developer portal
# code, we copy them into the build directory
copy_directory(
    name = "docs-src",
    src = "//:docs",
    out = "docs",
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
    ":docs-src",
    ":docusaurus.config.js",
    # Loadable packages
    "@npm//@docusaurus/preset-classic",
    "@npm//clsx",
#    "@npm//markdown-link-check",
    "@npm//react",
    "@npm//react-dom",
#    "@npm//react-loadable",
  ] + glob(
        include = [
            "src/**/*",
        ]),
  chdir=package_name(),
  outs = ["index.html"],
)