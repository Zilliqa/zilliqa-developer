load("@npm//:@docusaurus/core/package_json.bzl", docusaurus_bin = "bin")
# load("@npm//:react-app-rewired/package_json.bzl", cra_bin = "bin")

load("//:config/docusaurus.bzl", "docusaurus_pkg")

docusaurus_bin.docusaurus_binary(
    name = "docusaurus_exe",
    include_npm = True,
    include_npm_linked_packages=True,    
    data=[
        "//:package",
        "//:node_modules",
        "//:node_modules/@docusaurus/preset-classic",

    ],
    include_transitive_sources = True
)

docusaurus_pkg(
    name = "docs_pkg",
    srcs = ["//docs:files"] + glob(
        include = [
            "src/**/*",
        ],
    ),
    package = "//:package",
#    node_modules = "//:node_modules",
    binary = ":docusaurus_exe",
    config = "docusaurus.config.js",
    sidebars = "sidebars.js",
    strip_path = package_name(),
)


docusaurus_bin.docusaurus(
    name = "test",
    args = [
        "build",
        "$(execpath :docs_pkg)"
    ],
    chdir="../../..",
    srcs=[
        ":docs_pkg",
        "//:package",
        "//:node_modules",
        "//:node_modules/@docusaurus/preset-classic",
        "//:node_modules/@docusaurus/theme-common",
    ],    
    out_dirs=["build"]
)