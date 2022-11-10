load("@bazel_skylib//rules:copy_directory.bzl", "copy_directory")
load("@npm//@docusaurus/core:index.bzl", "docusaurus")

# To keep the docs cleanly separated from the developer portal
# code, we copy them into the build directory
copy_directory(
    name = "docs-src",
    src = "//:docs",
    out = "docs",
)

docusaurus(
    name = "start",
    outs = ["index.html"],
    args = [
        "build",
        "--config",
        "$(execpath docusaurus.config.js)",
        "--no-minify",
    ],
    chdir = package_name(),
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
        ],
    ),
)
