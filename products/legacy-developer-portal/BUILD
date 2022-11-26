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
        "//:node_modules/@docusaurus/core",
        "//:node_modules/@docusaurus/preset-classic",
        "//:node_modules/@docusaurus/theme-common",
        "//:node_modules/@mdx-js/react",
        "//:node_modules/react",
        "//:node_modules/clsx",
        "//:node_modules/prism-react-renderer",
        "//:node_modules/react-dom",
    ],    
    chdir = "../../../$(execpath :collected_sources)",
    silent_on_success=False,
    log_level = "info",
    out_dirs=["build"],
    env = {
        "JS_BINARY__PATCH_NODE_FS": "0"

    }    
)