workspace(
    name = "zilliqa",
)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "16e9fca53ed6bd4ff4ad76facc9b7b651a89db1689a2877d6fd7b82aa824e366",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.34.0/rules_go-v0.34.0.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.34.0/rules_go-v0.34.0.zip",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.18.3")

http_archive(
    name = "bazel_skylib",
    sha256 = "74d544d96f4a5bb630d465ca8bbcfe231e3594e5aae57e1edbf17a6eb3ca2506",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.3.0/bazel-skylib-1.3.0.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.3.0/bazel-skylib-1.3.0.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "bazel_gazelle",
    sha256 = "5982e5463f171da99e3bdaeff8c0f48283a7a5f396ec5282910b9e8a49c0dd7e",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.25.0/bazel-gazelle-v0.25.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.25.0/bazel-gazelle-v0.25.0.tar.gz",
    ],
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")

gazelle_dependencies()

# ================================================================
# Python and Pip
# ================================================================

http_archive(
    name = "rules_python",
    sha256 = "8c8fe44ef0a9afc256d1e75ad5f448bb59b81aba149b8958f02f7b3a98f5d9b4",
    strip_prefix = "rules_python-0.13.0",
    url = "https://github.com/bazelbuild/rules_python/archive/refs/tags/0.13.0.tar.gz",
)

load("@rules_python//python:pip.bzl", "pip_parse")

# Create a central repo that knows about the dependencies needed from
# requirements_lock.txt.
pip_parse(
    name = "zilliqa_python_deps",
    requirements = "//:requirements.txt",
)

# Load the starlark macro which will define your dependencies.
load("@zilliqa_python_deps//:requirements.bzl", "install_deps")

# Call it to define repos for your requirements.
install_deps()

# ================================================================
# Rules JS
# ================================================================
http_archive(
    name = "aspect_rules_js",
    sha256 = "c3b5fd40ec19f3260094321380169abe86dd89e3506c4e44a515a50c1626629b",
    strip_prefix = "rules_js-1.6.6",
    url = "https://github.com/aspect-build/rules_js/archive/refs/tags/v1.6.6.tar.gz",
)

http_archive(
    name = "aspect_rules_jest",
    sha256 = "6d6303372879579cff3c615d0f53ec1cea8a919ed457ffcd375ef5ac2aaaa0b4",
    strip_prefix = "rules_jest-0.11.1",
    url = "https://github.com/aspect-build/rules_jest/archive/refs/tags/v0.11.1.tar.gz",
)

load("@aspect_rules_js//js:repositories.bzl", "rules_js_dependencies")

rules_js_dependencies()

load("@rules_nodejs//nodejs:repositories.bzl", "DEFAULT_NODE_VERSION", "nodejs_register_toolchains")

nodejs_register_toolchains(
    name = "nodejs",
    node_version = DEFAULT_NODE_VERSION,
)

load("@aspect_rules_jest//jest:dependencies.bzl", "rules_jest_dependencies")

rules_jest_dependencies()

# Fetches the npm packages for jest-cli.
load("@aspect_rules_jest//jest:repositories.bzl", "jest_repositories")

jest_repositories(name = "jest")

load("@jest//:npm_repositories.bzl", jest_npm_repositories = "npm_repositories")

jest_npm_repositories()

load("@aspect_rules_js//npm:npm_import.bzl", "npm_translate_lock")

npm_translate_lock(
    name = "npm",
    bins = {
        # derived from "bin" attribute in node_modules/next/package.json
        "next": {
            "next": "./dist/bin/next",
        },
    },
    pnpm_lock = "//:pnpm-lock.yaml",
    verify_node_modules_ignored = "//:.bazelignore",
)

load("@npm//:repositories.bzl", "npm_repositories")

npm_repositories()

# JQ toolchain
load("@aspect_bazel_lib//lib:repositories.bzl", "aspect_bazel_lib_dependencies", "register_jq_toolchains")

aspect_bazel_lib_dependencies(override_local_config_platform = True)

register_jq_toolchains()

# ================================================================
# Protobuf
# ================================================================
http_archive(
    name = "rules_proto",
    sha256 = "e017528fd1c91c5a33f15493e3a398181a9e821a804eb7ff5acdd1d2d6c2b18d",
    strip_prefix = "rules_proto-4.0.0-3.20.0",
    urls = [
        "https://github.com/bazelbuild/rules_proto/archive/refs/tags/4.0.0-3.20.0.tar.gz",
    ],
)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

# ================================================================
# Rules TS
# ================================================================
http_archive(
    name = "aspect_rules_ts",
    sha256 = "1149d4cf7f210de67e0fc5cd3e8f624de3ee976ac05af4f1484e57a74c12f2dc",
    strip_prefix = "rules_ts-1.0.0-rc5",
    url = "https://github.com/aspect-build/rules_ts/archive/refs/tags/v1.0.0-rc5.tar.gz",
)

load("@aspect_rules_ts//ts:repositories.bzl", "LATEST_VERSION", "rules_ts_dependencies")

rules_ts_dependencies(ts_version = LATEST_VERSION)

# ================================================================
# Pkg Tar
# ================================================================

http_archive(
    name = "rules_pkg",
    sha256 = "451e08a4d78988c06fa3f9306ec813b836b1d076d0f055595444ba4ff22b867f",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_pkg/releases/download/0.7.1/rules_pkg-0.7.1.tar.gz",
        "https://github.com/bazelbuild/rules_pkg/releases/download/0.7.1/rules_pkg-0.7.1.tar.gz",
    ],
)

load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

rules_pkg_dependencies()

#    strip_prefix = "rules_pkg-0.8.0",

# ================================================================
# Docker
# ================================================================

# Seee https://github.com/bazelbuild/rules_docker/blob/master/testing/examples/WORKSPACE
# It contains an example on including custom Dockerfiles which may be handy for the isolated server

http_archive(
    name = "io_bazel_rules_docker",
    sha256 = "b1e80761a8a8243d03ebca8845e9cc1ba6c82ce7c5179ce2b295cd36f7e394bf",
    urls = ["https://github.com/bazelbuild/rules_docker/releases/download/v0.25.0/rules_docker-v0.25.0.tar.gz"],
)

load(
    "@io_bazel_rules_docker//repositories:repositories.bzl",
    container_repositories = "repositories",
)

container_repositories()

load(
    "@io_bazel_rules_docker//repositories:go_repositories.bzl",
    container_go_deps = "go_deps",
)

container_go_deps()

load("@io_bazel_rules_docker//container:pull.bzl", "container_pull")
load("@io_bazel_rules_docker//java:image.bzl", _java_image_repos = "repositories")

_java_image_repos()

# https://hub.docker.com/layers/zilliqa/zilliqa/v8.3.0-deps/images/sha256-35725f3b6799a359416fd7228815753b54a604b34f7bd3147807934dda49c2e5?context=explore
# https://hub.docker.com/layers/library/mediawiki/latest/images/sha256-b2fede20876f681b6b32dfe1ba49c93ba2e3507d8fe104bb41286e31e3a25861?context=explore
container_pull(
    name = "zilliqa-docker-x86",
    registry = "index.docker.io",
    repository = "zilliqa/zilliqa",
    tag = "v8.2.0rc2",
)

container_pull(
    name = "nginx",
    digest = "sha256:186c79dc14ab93e43d315143ee4b0774506dc4fd952388c20e35d3d37058ab8d",
    registry = "index.docker.io",
    repository = "library/nginx",
    tag = "1.23.1",
)

container_pull(
    name = "node12",
    digest = "sha256:c1a87a687d7635e668ab37eb66e113cad004f69ec488f9da19179d0328f7959c",
    registry = "index.docker.io",
    repository = "library/node",
    tag = "12.18.3",
)

# ================================================================
# Kubernetes
# ================================================================

http_archive(
    name = "io_bazel_rules_k8s",
    sha256 = "ce5b9bc0926681e2e7f2147b49096f143e6cbc783e71bc1d4f36ca76b00e6f4a",
    strip_prefix = "rules_k8s-0.7",
    urls = ["https://github.com/bazelbuild/rules_k8s/archive/refs/tags/v0.7.tar.gz"],
)

load("@io_bazel_rules_k8s//k8s:k8s.bzl", "k8s_repositories")

k8s_repositories()

# TODO: THis causes issues
# load("@io_bazel_rules_k8s//k8s:k8s_go_deps.bzl", k8s_go_deps = "deps")
# k8s_go_deps()

# ================================================================
# Java
# ================================================================

RULES_JVM_EXTERNAL_TAG = "5.1"

RULES_JVM_EXTERNAL_SHA = "8c3b207722e5f97f1c83311582a6c11df99226e65e2471086e296561e57cc954"

http_archive(
    name = "rules_jvm_external",
    sha256 = RULES_JVM_EXTERNAL_SHA,
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    url = "https://github.com/bazelbuild/rules_jvm_external/releases/download/%s/rules_jvm_external-%s.tar.gz" % (RULES_JVM_EXTERNAL_TAG, RULES_JVM_EXTERNAL_TAG),
)

load("@rules_jvm_external//:repositories.bzl", "rules_jvm_external_deps")

rules_jvm_external_deps()

load("@rules_jvm_external//:setup.bzl", "rules_jvm_external_setup")

rules_jvm_external_setup()

load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    artifacts = [
        "junit:junit:4.12",
        "org.web3j:core:4.9.7",
        "org.web3j:crypto:4.9.7",
        "org.web3j:utils:4.9.7",
        "org.projectlombok:lombok:1.18.26",
        "com.google.guava:guava:31.1-jre",
        "com.squareup.okhttp3:okhttp:4.9.3",
        "com.google.code.gson:gson:2.10.1",
        "com.google.protobuf:protobuf-java:3.22.2",
        "org.apache.commons:commons-lang3:3.12.0",
        "org.bouncycastle:bcprov-jdk18on:1.72",
    ],
    repositories = [
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)

# ================================================================
# AWS
# ================================================================

# http_file(
#     name = "gcloud_archive",
#     downloaded_file_path = "google-cloud-sdk.tar.gz",
#     sha256 = "a2205e35b11136004d52d47774762fbec9145bf0bda74ca506f52b71452c570e",
#     urls = [
#         "https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-220.0.0-linux-x86_64.tar.gz",
#     ],
# )
