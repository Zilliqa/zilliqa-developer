load("@io_bazel_rules_docker//container:container.bzl", "container_image", "container_push")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
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

# TODO: Consider using genfile to use the same configuration for both deployments

pkg_tar(
    name = "nignx-conf",
    srcs = ["nginx/default.conf"],
    mode = "0755",
    package_dir = "/etc/nginx/conf.d/",
    deps = [],
)

pkg_tar(
    name = "nignx-dev-conf",
    srcs = ["nginx-dev/default.conf"],
    mode = "0755",
    package_dir = "/etc/nginx/conf.d/",
    deps = [],
)

container_image(
    name = "image",
    base = "@nginx//image",
    # Due to bazel's OS X docker support: https://github.com/bazelbuild/rules_docker/issues/768
    docker_run_flags = "--publish=80:80",
    # Disabling legacy run behaviour to allow run from the command line
    legacy_run_behavior = False,
    ports = ["80"],
    tars = [
        ":html-folder",
        ":nignx-conf",
    ],
)

container_image(
    name = "dev-image",
    base = "@nginx//image",
    # Due to bazel's OS X docker support: https://github.com/bazelbuild/rules_docker/issues/768
    docker_run_flags = "--publish=8000:8000",
    # Disabling legacy run behaviour to allow run from the command line
    legacy_run_behavior = False,
    ports = ["8000"],
    tars = [
        ":html-folder",
        ":nignx-dev-conf",
    ],
)

container_push(
    name = "push_image_dev",
    format = "Docker",
    image = ":image",
    registry = "localhost:5001",
    repository = "developer-portal",
    tag = "latest",
)

container_push(
    name = "push_image_stg",
    format = "Docker",
    image = ":image",
    registry = "asia-docker.pkg.dev/prj-d-devops-services-4dgwlsse/zilliqa-public",
    repository = "developer-portal",
    tag = "$${IMAGE_TAG#*:}",
)

container_push(
    name = "push_image_prd",
    format = "Docker",
    image = ":image",
    registry = "asia-docker.pkg.dev/prj-p-devops-services-tvwmrf63/zilliqa-public",
    repository = "developer-portal",
    tag = "$${IMAGE_TAG#*:}",
)
