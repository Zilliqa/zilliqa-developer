load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")

###
# CD update

pkg_tar(
    name = "cd_base",
    srcs = glob(["cd/base/*.yaml"]),
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "production-kustomization",
    output = "cd/overlays/production/kustomization.yaml",
    template = "cd/overlays/production/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_production_patch",
    srcs = glob(
        ["cd/overlays/production/*.yaml"],
        exclude = ["cd/overlays/production/*.tpl.yaml"],
    ) + [
        "cd/overlays/production/kustomization.yaml",
    ],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)
