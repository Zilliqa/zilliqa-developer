load("@npm//:@docusaurus/core/package_json.bzl", docusaurus_bin = "bin")
load("//:config/docusaurus.bzl", "docusaurus_pkg")
load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@rules_pkg//:pkg.bzl", "pkg_tar")


copy_to_bin(
    name = "package",
    srcs = ["package.json"],
)


docusaurus_pkg(
    name = "collected_sources",
    srcs = ["//docs:files","package.json","tsconfig.json"] + glob(
        include = [
            "src/**/*",
        ],
    ),
    config = "docusaurus.config.js",
    sidebars = "sidebars.js",
    strip_path = package_name(),
)


docusaurus_bin.docusaurus(
    name = "build",
    args = [
        "build",
        "fail",
        # Add .. to build dir as we have collected all in a sandbox
        "--out-dir",
        "../build"
    ],
    srcs=[
        ":package",
        ":collected_sources",
        "//:node_modules",
        "//:node_modules/@docusaurus/preset-classic",
        "//:node_modules/@docusaurus/theme-common",
        "//:node_modules/@mdx-js/react"
    ],    
    chdir = "../../../$(execpath :collected_sources)",
    log_level ="debug",
    silent_on_success = False,
    out_dirs=["build"],
    env = {
		"JS_BINARY__PATCH_NODE_FS": "0"

    }
)


pkg_tar(
    name = "html-folder",
    srcs = [":build"],
    mode = "0755",
    package_dir = "/usr/share/nginx/html/",
    strip_prefix = "build",
)

pkg_tar(
    name = "nignx-conf",
    srcs = ["nginx/default.conf"],
    mode = "0755",
    package_dir = "/etc/nginx/conf.d/",
    deps = [],
)

container_image(
    name = "image",
    base = "@nginx//image",

    # Disabling legacy run behaviour to allow run from the command line
    legacy_run_behavior = False,
    ports = ["80"],
    tars = [
        ":html-folder",
        ":nignx-conf",
    ],
)

# # TODO: Not working yet
# docusaurus_bin.docusaurus_binary(
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
