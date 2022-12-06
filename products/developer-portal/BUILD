load("@io_bazel_rules_docker//container:container.bzl", "container_image", "container_push")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")
load(":defs.bzl", "mkdocs_html")

# TODO:
# mkdocs_collect(
#     name = "collect",
#     srcs = [
#         "mkdocs.yml",
#         "//docs:files",
#     ],
#     config = "mkdocs.yml",
#     strip_path = package_name(),
#     out = "source"
# )
#
# py_binary(
#     name = "dev",
#     main= "mkdocs_wrapper.py",
#     srcs = ["//products/developer-portal/tools:mkdocs_wrapper.py"],
#     visibility = ["//visibility:public"],
#     args= ["serve", "-f", "$(location :collect)", "-a","0.0.0.0:8000" ],
#     data = [":collect",],
#     deps = [
#         requirement("click"),
#         requirement("ghp-import"),
#         requirement("Jinja2"),
#         requirement("Markdown"),
#         requirement("MarkupSafe"),
#         requirement("mergedeep"),
#         requirement("mkdocs"),
#         requirement("packaging"),
#         requirement("pyparsing"),
#         requirement("python-dateutil"),
#         requirement("PyYAML"),
#         requirement("pyyaml_env_tag"),
#         requirement("six"),
#         requirement("watchdog"),
#         requirement("pymdown-extensions"),
#         requirement("mkdocs-material"),
#     ],
# )

filegroup(
    name = "extra-files",
    srcs = glob([
        "stylesheets/*",
        "overrides/*",
    ]),
)

mkdocs_html(
    name = "build",
    srcs = [
        "mkdocs.yml",
        ":extra-files",
        "//docs:files",
    ],
    config = "mkdocs.yml",
    remap_paths = {
        "stylesheets/extra.css": "docs/stylesheets/extra.css",
    },
    strip_path = package_name(),
)

##
# Docker

pkg_tar(
    name = "html-folder",
    srcs = [":build"],
    mode = "0755",
    package_dir = "/usr/share/nginx/html/",
    #    strip_prefix = package_name(),
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

container_push(
    name = "push_image",
    format = "Docker",
    image = ":image",
    registry = "298213327629.dkr.ecr.us-west-2.amazonaws.com",
    repository = "zilliqa-devportal",

    # Tagging from workspace status - requires --stamp as build args
    tag = "{FULL_VERSION_TAG}",
)

###
# CD update
expand_workspace_status(
    name = "preview-kustomization",
    output = "cd/applications/devportal/overlays/preview/kustomization.yaml",
    template = "cd/applications/devportal/overlays/preview/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_preview_patch",
    srcs = ["cd/applications/devportal/overlays/preview/kustomization.yaml"],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "production-kustomization",
    output = "cd/applications/devportal/overlays/production/kustomization.yaml",
    template = "cd/applications/devportal/overlays/production/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_production_patch",
    srcs = ["cd/applications/devportal/overlays/production/kustomization.yaml"],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)
