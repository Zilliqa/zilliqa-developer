load("@npm//:vuepress/package_json.bzl", vuepress_bin = "bin")
load(":defs.bzl", "collect")
load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("@aspect_rules_js//npm:defs.bzl", "npm_package")
load("@npm//:defs.bzl", "npm_link_all_packages")

npm_link_all_packages(name = "node_modules")

copy_to_bin(
    name = "package",
    srcs = ["package.json"],
)

collect(
    name = "collected_sources",
    srcs = ["//docs:files"] + glob(
        include = [
            ".vuepress/*",
            ".vuepress/**/*",
        ],
    ),
    strip_path = package_name(),
)


vuepress_bin.vuepress(
    name = "build",
    args = [
        "build",
    ],
    srcs=[
        ":package",
        ":collected_sources",
        "//:node_modules",        
        ":node_modules",
        ":node_modules/@vuepress/core",
        ":node_modules/@vuepress/shared-utils",
    ],    
   chdir = "../../../$(execpath :collected_sources)",
    log_level ="debug",
    silent_on_success = False,
    include_npm = True,
    include_npm_linked_packages = True,
    include_transitive_sources = True,
    out_dirs=["build"],
    patch_node_fs = True,
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

