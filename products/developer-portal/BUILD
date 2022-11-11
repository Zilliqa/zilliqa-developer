load("//:config/docusaurus.bzl", "docusaurus_html")

docusaurus_html(
    name = "test_docs",
    srcs = glob(
        include = [
            "src/**/*",
        ],
    ),
    config = "docusaurus.config.js",
    docs = ["//docs:files"],
    sidebars = "sidebars.js",
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
