load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")

###
# CD update

pkg_tar(
    name = "cd_base",
    srcs = glob(["products/devex-apollo/cd/base/*.yaml"]),
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "staging-kustomization",
    output = "products/devex-apollo/cd/overlays/staging/kustomization.yaml",
    template = "products/devex-apollo/cd/overlays/staging/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_staging_patch",
    srcs = glob(
        ["products/devex-apollo/cd/overlays/staging/*.yaml"],
        exclude = ["products/devex-apollo/cd/overlays/staging/*.tpl.yaml"],
    ) + [
        "products/devex-apollo/cd/overlays/staging/kustomization.yaml",
    ],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "production-kustomization",
    output = "products/devex-apollo/cd/overlays/production/kustomization.yaml",
    template = "products/devex-apollo/cd/overlays/production/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_production_patch",
    srcs = glob(
        ["products/devex-apollo/cd/overlays/production/*.yaml"],
        exclude = ["products/devex-apollo/cd/overlays/production/*.tpl.yaml"],
    ) + [
        "products/devex-apollo/cd/overlays/production/kustomization.yaml",
    ],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)
