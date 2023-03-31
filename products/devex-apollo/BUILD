load("@io_bazel_rules_docker//container:container.bzl", "container_push")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")

# Not currently used as Dockerfile still part of the process
#copy_to_bin(
#    name = "static_files",
#    srcs = glob(
#        [
#            "*",
#            "config/*",
#            "datasource/*",
#            "gql/*",
#            "mongodb/*",
#        ],
#    ) + [
#        "package.json",
#        "yarn.lock",
#    ],
#)

#container_image(
#    name = "image",
#    base = "@node//image",
#
#    # Disabling legacy run behaviour to allow run from the command line
#    legacy_run_behavior = False,
#    ports = ["5000"],
#)

container_push(
    name = "push_image_staging",
    format = "Docker",
    image = "devex-apollo:local",
    registry = "816080630680.dkr.ecr.us-west-2.amazonaws.com",
    repository = "developer-explorer",

    # Tagging from workspace status - requires --stamp as build args
    tag = "{FULL_VERSION_TAG}",
)

container_push(
    name = "push_image_production",
    format = "Docker",
    image = "devex-apollo:local",
    registry = "298213327629.dkr.ecr.us-west-2.amazonaws.com",
    repository = "developer-explorer",

    # Tagging from workspace status - requires --stamp as build args
    tag = "{FULL_VERSION_TAG}",
)

###
# CD update

pkg_tar(
    name = "cd_base",
    srcs = glob(["products/devex/cd/base/*.yaml"]),
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "staging-kustomization",
    output = "products/devex/cd/overlays/staging/kustomization.yaml",
    template = "products/devex/cd/overlays/staging/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_staging_patch",
    srcs = ["products/devex/cd/overlays/staging/kustomization.yaml"],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "production-kustomization",
    output = "products/devex/cd/overlays/production/kustomization.yaml",
    template = "products/devex/cd/overlays/production/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_production_patch",
    srcs = ["products/devex/cd/overlays/production/kustomization.yaml"],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)
