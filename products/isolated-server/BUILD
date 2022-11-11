load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@rules_pkg//:pkg.bzl", "pkg_tar")

pkg_tar(
    name = "logs-dir",
    srcs = [],
    package_dir = "/zilliqa/logs",
    strip_prefix = ".",
)

pkg_tar(
    name = "image-content",
    srcs = [
        "boot.json",
        "constants.xml",
        "dsnodes.xml",
        "run.sh",
        "validate.sh",
    ],
    mode = "0755",
    package_dir = "/zilliqa",
    strip_prefix = ".",
)

container_image(
    name = "latest",
    base = "@zilliqa-docker-x86//image",
    compression = "gzip",
    compression_options = ["--fast"],
    entrypoint = [
        "bash",
        "run.sh",
    ],
    env = {
        "MODE": "",
    },
    experimental_tarball_format = "compressed",
    ports = [
        "5555",
    ],
    tars = [
        ":image-content",
        ":logs-dir",
    ],
    workdir = "/zilliqa",
)
