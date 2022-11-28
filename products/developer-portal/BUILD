load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@npm//:defs.bzl", "npm_link_all_packages")
load("defs.bzl", "collect_files")

npm_link_all_packages(name = "node_modules")

copy_to_bin(
    name = "package",
    srcs = ["package.json"],
)

collect_files(
    name = "collected_sources",
    srcs = [
        "//docs:files",
        "//assets:files",
        "docusaurus.config.js",
        "sidebars.js",
        "babel.config.js",
        "package.json",
        "tsconfig.json",
    ] + glob(
        include = [
            "src/**/*",
        ],
    ),
    strip_path = package_name(),
)

# TODO (issue US-157):
# Docusaurus cannot deal with symbolic links
# docusaurus_bin.docusaurus(
#     name = "build",
#     srcs = [
#         ":collected_sources",
#         ":node_modules",
#         ":node_modules/@docusaurus/core",
#         ":node_modules/@docusaurus/preset-classic",
#         ":node_modules/@docusaurus/theme-common",
#         ":node_modules/@mdx-js/react",
#         ":node_modules/clsx",
#         ":node_modules/prism-react-renderer",
#         ":node_modules/react",
#         ":node_modules/react-dom",
#         ":package",
#     ],
#     args = [
#         "build",
#         # Add .. to build dir as we have collected all in a sandbox
#         "--out-dir",
#         "../build",
#     ],
#     chdir = "../../../$(execpath :collected_sources)",
#     env = {
#         "JS_BINARY__PATCH_NODE_FS": "0",
#     },
#     log_level = "debug",
#     out_dirs = ["build"],
#     silent_on_success = False,
# )
#
# pkg_tar(
#     name = "html-folder",
#     srcs = [":build"],
#     mode = "0755",
#     package_dir = "/usr/share/nginx/html/",
#     strip_prefix = "build",
# )
#
# pkg_tar(
#     name = "nignx-conf",
#     srcs = ["nginx/default.conf"],
#     mode = "0755",
#     package_dir = "/etc/nginx/conf.d/",
#     deps = [],
# )
#
# container_image(
#     name = "image",
#     base = "@nginx//image",
#
#     # Disabling legacy run behaviour to allow run from the command line
#     legacy_run_behavior = False,
#     ports = ["80"],
#     tars = [
#         ":html-folder",
#         ":nignx-conf",
#     ],
# )
#
# # # TODO: Not working yet
# # docusaurus_bin.docusaurus_binary(
#     name = "start",
#     args = [
#         "start"
#     ],
#     data=[
#         ":package",
#         ":collected_sources",
#         "//:node_modules",
#         "//:node_modules/@docusaurus/preset-classic",
#         "//:node_modules/@docusaurus/theme-common",
#     ],
#     chdir = "../../../$(execpath :collected_sources)",
#     tags = [
#         # This tag instructs ibazel to pipe into stdin a event describing actions.
#         # ibazel send EOF to stdin by default and `react-scripts start` will stop when getting EOF in stdin.
#         # So use this to prevent EOF.
#         "ibazel_notify_changes",
#     ],
# )
