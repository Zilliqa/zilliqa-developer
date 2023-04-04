load("@io_bazel_rules_docker//container:container.bzl", "container_image", "container_layer")
load("@io_bazel_rules_docker//docker/util:run.bzl", "container_run_and_commit", "container_run_and_commit_layer")
load("@rules_pkg//:pkg.bzl", "pkg_tar")

_RUNTIME_DEPS = [
]

pkg_tar(
    name = "src",
    srcs = glob(["**"]),
    
    package_dir = "/app",
)

container_image(
    name = "installer",
    base = "@node12//image",

    tars = [
        ":src",
    ],

    legacy_run_behavior = False,
    ports = ["5000"],
    workdir = "/app",
)

container_run_and_commit_layer(
    name = "install",
    commands = ["apt-get update", "apt-get install -y python", "yarn install"],
    docker_run_flags = [
        "-u",
        "root",
    ],
    image = ":installer.tar",
)

container_image(
    name = "installed",
    base = "@node12//image",
    entrypoint = [
        "bash",
        "run.sh",
    ],
    layers = [":install-layer.tar"],
    legacy_run_behavior = False,
    ports = ["5000"],
    workdir = "/app",
)

# container_run_and_commit(
#     name = "install",
#     commands = [
#         # "apt-get update",
#         # "apt-get install -y python",
#         # "yarn install",
#         "ls",
#     ],
#     image = ":installer.tar",
# )

# container_run_and_commit_layer(
#     name = "test_container_commit_layer",
#     commands = ["touch /foo.txt"],
#     docker_run_flags = [
#         "-u",
#         "root",
#     ],
#     image = "@debian_base//image",
# )

# container_image(
#     name = "installed",
#     base = ":install_commit.tar",
#     cmd = [
#         "bash",
#         "run.sh",
#     ],
#     layers = [":src_in_layer"],
#     legacy_run_behavior = False,
#     ports = ["5000"],
#     workdir = "/app",
# )

# pkg_tar(
#     name = "src",
#     srcs = glob(["**"]),
#     # mode = "0755",
#     # package_dir = "/usr/share/nginx/html/",
#     # strip_prefix = "savant-artifact",
# )

# container_layer(
#     name = "src_in_layer",
#     directory = "/app",
#     # files = ["package.json", "yarn.lock"],
#     tars = [":src"],
#     # mode = "755",
# )

# container_image(
#     name = "installer",
#     base = "@node12//image",
#     # cmd = "",
#     # entrypoint = "",
#     legacy_run_behavior = False,
#     layers = [":files_in_layer"],
# )

# container_run_and_commit(
#     name = "install",
#     commands = [
#         "apt-get update",
#         "apt-get install -y python",
#         "yarn install",
#     ],
#     image = ":installer.tar",
# )

# # Wrapper targets for the tarred images are required in order to be able to run
# # commandTests in container_test targets.

# container_image(
#     name = "image",
#     base = ":install_commit.tar",
#     cmd = ["bash", "run.sh"],

#     legacy_run_behavior = False,
#     ports = ["5000"],
#     workdir = "/app"
# )

# pkg_tar(
#     name = "devex-apollo",
#     srcs = glob(["devex-apollo-artifact/**"]),
#     mode = "0755",
#     package_dir = "/tmp/",
#     strip_prefix = "devex-apollo-artifact",
# )

# container_image(
#     name = "image",
#     base = "@node12//image",
#     # cmd = [
#     #     "bash",
#     #     "ls",
#     # ],

#     # Disabling legacy run behaviour to allow run from the command line
#     legacy_run_behavior = False,
#     ports = ["5000"],
#     tars = [
#         ":devex-apollo",
#     ],
# )

# container_push(
#     name = "push_image_staging",
#     format = "Docker",
#     image = ":image",
#     registry = "816080630680.dkr.ecr.us-west-2.amazonaws.com",
#     repository = "devex-apollo",

#     # Tagging from workspace status - requires --stamp as build args
#     tag = "{FULL_VERSION_TAG}",
# )

# container_push(
#     name = "push_image_production",
#     format = "Docker",
#     image = ":image",
#     registry = "298213327629.dkr.ecr.us-west-2.amazonaws.com",
#     repository = "devex-apollo",

#     # Tagging from workspace status - requires --stamp as build args
#     tag = "{FULL_VERSION_TAG}",
# )

# ###
# # CD update

# pkg_tar(
#     name = "cd_base",
#     srcs = glob(["products/devex-apollo/cd/base/*.yaml"]),
#     mode = "0755",
#     package_dir = "",
#     strip_prefix = ".",
#     visibility = ["//visibility:public"],
# )

# expand_workspace_status(
#     name = "staging-kustomization",
#     output = "products/devex-apollo/cd/overlays/staging/kustomization.yaml",
#     template = "products/devex-apollo/cd/overlays/staging/kustomization.tpl.yaml",
# )

# pkg_tar(
#     name = "cd_staging_patch",
#     srcs = glob(
#         ["products/devex-apollo/cd/overlays/staging/*.yaml"],
#         exclude = ["products/devex-apollo/cd/overlays/staging/*.tpl.yaml"],
#     ) + [
#         "products/devex-apollo/cd/overlays/staging/kustomization.yaml",
#     ],
#     mode = "0755",
#     package_dir = "",
#     strip_prefix = ".",
#     visibility = ["//visibility:public"],
# )

# expand_workspace_status(
#     name = "production-kustomization",
#     output = "products/devex-apollo/cd/overlays/production/kustomization.yaml",
#     template = "products/devex-apollo/cd/overlays/production/kustomization.tpl.yaml",
# )

# pkg_tar(
#     name = "cd_production_patch",
#     srcs = glob(
#         ["products/devex-apollo/cd/overlays/production/*.yaml"],
#         exclude = ["products/devex-apollo/cd/overlays/production/*.tpl.yaml"],
#     ) + [
#         "products/devex-apollo/cd/overlays/production/kustomization.yaml",
#     ],
#     mode = "0755",
#     package_dir = "",
#     strip_prefix = ".",
#     visibility = ["//visibility:public"],
# )
