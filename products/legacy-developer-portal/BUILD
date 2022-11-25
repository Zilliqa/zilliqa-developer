load("@npm//:@docusaurus/core/package_json.bzl", docusaurus_bin = "bin")
load("//:config/docusaurus.bzl", "docusaurus_pkg")
load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")

copy_to_bin(
    name = "package",
    srcs = ["package.json"],
)


docusaurus_pkg(
    name = "collected_sources",
    srcs = ["//docs:files","package.json"] + glob(
        include = [
            "src/**/*",
        ],
    ),
    config = "docusaurus.config.js",
    sidebars = "sidebars.js",
    strip_path = package_name(),
)


docusaurus_bin.docusaurus(
    name = "test",
    args = [
        "build",        
    ],
    srcs=[
        ":package",
        ":collected_sources",
        "//:node_modules",
        "//:node_modules/@docusaurus/preset-classic",
        "//:node_modules/@docusaurus/theme-common",
    ],    
    chdir = "../../../$(execpath :collected_sources)",
    silent_on_success=False,
    log_level = "info",
    out_dirs=["build"]
)