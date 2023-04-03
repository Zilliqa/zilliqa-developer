load("@io_bazel_rules_docker//container:container.bzl", "container_image", "container_push")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")

_RUNTIME_DEPS = [
]

pkg_tar(
    name = "devex-apollo",
    srcs = glob(["devex-apollo-artifact/**"]),
    mode = "0755",
    package_dir = "/app/",
    strip_prefix = "devex-apollo-artifact",
)

container_image(
    name = "image",
    base = "@node12//image",
    cmd = [
        "bash",
        "run.sh",
    ],

    # Disabling legacy run behaviour to allow run from the command line
    legacy_run_behavior = False,
    ports = ["5000"],
    tars = [
        ":devex-apollo",
    ],
)

container_push(
    name = "push_image_staging",
    format = "Docker",
    image = ":image",
    registry = "816080630680.dkr.ecr.us-west-2.amazonaws.com",
    repository = "devex-apollo",

    # Tagging from workspace status - requires --stamp as build args
    tag = "{FULL_VERSION_TAG}",
)

container_push(
    name = "push_image_production",
    format = "Docker",
    image = ":image",
    registry = "298213327629.dkr.ecr.us-west-2.amazonaws.com",
    repository = "devex-apollo",

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
