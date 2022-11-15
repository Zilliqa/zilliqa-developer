load("//:config/docusaurus.bzl", "docusaurus_pkg")

# load("@npm//:react-app-rewired/package_json.bzl", cra_bin = "bin")
docusaurus_pkg(
    name = "test_docs",
    srcs = glob(
        include = [
            "src/**/*",
        ],
    ),
    config = "docusaurus.config.js",
    docs = ["//docs:files"],
    sidebars = "sidebars.js",
    srcs_strip = package_name(),
    #    binary = docusaurus_bin,
)

# docusaurus(
#     name = "start",
#     outs = ["index.html"],
#     args = [
#         "build",
#         "--config",
#         "$(execpath docusaurus.config.js)",
#         "--no-minify",
#     ],
#     chdir = package_name(),
#     data = [
#         # Actually documentation source
#         ":docs-src",
#         ":docusaurus.config.js",
#         # Loadable packages
#         "@npm//@docusaurus/preset-classic",
#         "@npm//clsx",
#         #    "@npm//markdown-link-check",
#         "@npm//react",
#         "@npm//react-dom",
#         #    "@npm//react-loadable",
#     ] + glob(
#         include = [
#             "src/**/*",
#         ],
#     ),
# )
