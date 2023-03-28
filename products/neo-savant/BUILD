load("@io_bazel_rules_docker//container:container.bzl", "container_image", "container_push")
load("@npm//:react-app-rewired/package_json.bzl", cra_bin = "bin")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")

_RUNTIME_DEPS = [
]

pkg_tar(
    name = "html-folder",
    srcs = glob(["savant-artifact/**"]),
    mode = "0755",
    package_dir = "/usr/share/nginx/html/",
    strip_prefix = "savant-artifact",
)

container_image(
    name = "image",
    base = "@nginx//image",

    # Disabling legacy run behaviour to allow run from the command line
    legacy_run_behavior = False,
    ports = ["80"],
    tars = [
        ":html-folder",
    ],
)

container_push(
    name = "push_image_staging",
    format = "Docker",
    image = ":image",
    registry = "816080630680.dkr.ecr.us-west-2.amazonaws.com",
    repository = "scilla-savant-ide",

    # Tagging from workspace status - requires --stamp as build args
    tag = "{FULL_VERSION_TAG}",
)

container_push(
    name = "push_image_production",
    format = "Docker",
    image = ":image",
    registry = "298213327629.dkr.ecr.us-west-2.amazonaws.com",
    repository = "scilla-savant-ide",

    # Tagging from workspace status - requires --stamp as build args
    tag = "{FULL_VERSION_TAG}",
)

###
# CD update

pkg_tar(
    name = "cd_base",
    srcs = glob(["products/neo-savant/cd/base/*.yaml"]),
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "staging-kustomization",
    output = "products/neo-savant/cd/overlays/staging/kustomization.yaml",
    template = "products/neo-savant/cd/overlays/staging/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_staging_patch",
    srcs = ["products/neo-savant/cd/overlays/staging/kustomization.yaml"],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

expand_workspace_status(
    name = "production-kustomization",
    output = "products/neo-savant/cd/overlays/production/kustomization.yaml",
    template = "products/neo-savant/cd/overlays/production/kustomization.tpl.yaml",
)

pkg_tar(
    name = "cd_production_patch",
    srcs = ["products/neo-savant/cd/overlays/production/kustomization.yaml"],
    mode = "0755",
    package_dir = "",
    strip_prefix = ".",
    visibility = ["//visibility:public"],
)

cra_bin.react_app_rewired_binary(
    name = "start",
    args = [
        "start",
    ],
    chdir = package_name(),
    data = _RUNTIME_DEPS + [
        "//:node_modules/react-app-rewired",
        "//:node_modules/react-scripts",
        "//:node_modules/typescript",
        "//:node_modules/web-vitals",
        "//:tsconfig.base",
        "//:node_modules/webpack",
    ],
    tags = [
        # This tag instructs ibazel to pipe into stdin a event describing actions.
        # ibazel send EOF to stdin by default and `react-scripts start` will stop when getting EOF in stdin.
        # So use this to prevent EOF.
        "ibazel_notify_changes",
    ],
)

# TODO: Many snapshot based tests with heuristic class names.
# cra_bin.react_app_rewired_test(
#     name = "test",
#     args = [
#         "test",
#         # ibazel is the watch mode for Bazel when running tests
#         # Because Bazel is really a CI system that runs locally
#         "--watchAll=false",
#         # https://jestjs.io/docs/cli#--cache. Whether to use the cache. Defaults to true. Disable
#         # the cache using --no-cache. Caching is Bazel's job, we don't want non-hermeticity
#         "--no-cache",
#         # https://jestjs.io/docs/cli#--watchman. Whether to use watchman for file crawling. Defaults
#         # to true. Disable using --no-watchman. Watching is ibazel's job
#         "--no-watchman",
#         # https://jestjs.io/docs/cli#--ci. When this option is provided, Jest will assume it is
#         # running in a CI environment. This changes the behavior when a new snapshot is encountered.
#         # Instead of the regular behavior of storing a new snapshot automatically, it will fail the
#         # test and require Jest to be run with --updateSnapshot.
#         "--ci",
#         # Needed for Jest to walk the filesystem to find inputs.
#         # See https://github.com/facebook/jest/pull/9351
#         "--haste",
#         "{\\\"enableSymlinks\\\":\\ true}",
#     ],
#     chdir = package_name(),
#     data = _RUNTIME_DEPS + [
#         "copy_test_files",
#         "//:node_modules/@testing-library/jest-dom",
#         "//:node_modules/@testing-library/react",
#         "//:node_modules/@testing-library/user-event",
#         "//:node_modules/typescript",
#         "//:node_modules/jest-watch-typeahead",
#         "//:node_modules/webpack",
#     ],
# )
