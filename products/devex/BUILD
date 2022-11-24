load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@npm//:react-app-rewired/package_json.bzl", cra_bin = "bin")
load("@rules_pkg//:pkg.bzl", "pkg_tar")

# Filename conventions described at
# https://create-react-app.dev/docs/running-tests#filename-conventions
_TESTS = [
    "src/**/*.test.js*",
    "src/**/*.test.ts*",
    "src/**/*.spec.js*",
    "src/**/*.spec.ts*",
    "src/**/__tests__/**/*.js*",
    "src/**/__tests__/**/*.ts*",
]

copy_to_bin(
    name = "static_files",
    srcs = glob(
        [
            "public/*",
            "src/**/*",
        ],
        exclude = _TESTS,
    ) + [
        "package.json",
        "tsconfig.json",
        "config-overrides.js",
    ],
)

_RUNTIME_DEPS = [
    ":static_files",
    "//:node_modules/@zilliqa-js/account",
    "//:node_modules/@zilliqa-js/blockchain",
    "//:node_modules/@zilliqa-js/contract",
    "//:node_modules/@zilliqa-js/core",
    "//:node_modules/@zilliqa-js/crypto",
    "//:node_modules/@zilliqa-js/proto",
    "//:node_modules/@zilliqa-js/subscriptions",
    "//:node_modules/@zilliqa-js/util",
    "//:node_modules/@zilliqa-js/zilliqa",
    "//:node_modules/@apollo/client",
    "//:node_modules/@fortawesome/fontawesome-svg-core",
    "//:node_modules/@fortawesome/free-brands-svg-icons",
    "//:node_modules/@fortawesome/free-regular-svg-icons",
    "//:node_modules/@fortawesome/free-solid-svg-icons",
    "//:node_modules/@fortawesome/react-fontawesome",
    "//:node_modules/@types/d3",
    "//:node_modules/@types/sanitize-html",
    "//:node_modules/ace-builds",
    "//:node_modules/apollo-boost",
    "//:node_modules/bootstrap",
    "//:node_modules/d3",
    "//:node_modules/dayjs",
    "//:node_modules/graphql",
    "//:node_modules/leader-line",
    "//:node_modules/leader-line-types",
    "//:node_modules/moment",
    "//:node_modules/numbro",
    "//:node_modules/rc-switch",
    "//:node_modules/react",
    "//:node_modules/react-ace",
    "//:node_modules/react-apollo",
    "//:node_modules/react-beautiful-dnd",
    "//:node_modules/react-bootstrap",
    "//:node_modules/react-contenteditable",
    "//:node_modules/react-custom-scroll",
    "//:node_modules/react-dom",
    "//:node_modules/react-dropzone",
    "//:node_modules/react-fontawesome",
    "//:node_modules/react-router-dom",
    "//:node_modules/react-table",
    "//:node_modules/sanitize-html",
    "//:node_modules/@types/enzyme",
    "//:node_modules/@types/jest",
    "//:node_modules/@types/node",
    "//:node_modules/@types/react",
    "//:node_modules/@types/react-beautiful-dnd",
    "//:node_modules/@types/react-custom-scroll",
    "//:node_modules/@types/react-dom",
    "//:node_modules/@types/react-router-dom",
    "//:node_modules/@types/react-table",
    "//:node_modules/@types/react-test-renderer",
    "//:node_modules/@typescript-eslint/eslint-plugin",
    "//:node_modules/@typescript-eslint/parser",
    "//:node_modules/cz-conventional-changelog",
    "//:node_modules/enzyme",
    "//:node_modules/enzyme-to-json",
    "//:node_modules/eslint-plugin-react",
    "//:node_modules/react-test-renderer",
    "//:node_modules/process",
    "//:node_modules/url",
    "//:node_modules/assert",
    "//:node_modules/crypto-browserify",
    "//:node_modules/stream-http",
    "//:node_modules/https-browserify",
    "//:node_modules/os-browserify",
    "//:node_modules/buffer",
    "//:node_modules/stream-browserify",
    "//:node_modules/string_decoder",
    "//:node_modules/events",
    "//:node_modules/classnames",
    "//:node_modules/tslib",
    "//:node_modules/bip39",
    "//:node_modules/hash.js",
    "//:node_modules/hdkey",
]

cra_bin.react_app_rewired(
    # Note: If you want to change the name make sure you update BUILD_PATH below accordingly
    # https://create-react-app.dev/docs/advanced-configuration/
    name = "build",
    srcs = _RUNTIME_DEPS + [
        "//:node_modules/react-app-rewired",
        "//:node_modules/react-scripts",
        "//:node_modules/typescript",
        "//:node_modules/web-vitals",
        "//:node_modules/webpack",
    ],
    args = ["build"],
    chdir = package_name(),
    env = {
        "BUILD_PATH": "./build",
    },
    out_dirs = ["build"],
)

copy_to_bin(
    name = "copy_test_files",
    srcs = glob(_TESTS),
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
