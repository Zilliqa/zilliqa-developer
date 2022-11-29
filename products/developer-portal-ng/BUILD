load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@rules_pkg//:pkg.bzl", "pkg_tar")
load(":defs.bzl", "mkdocs_html")

mkdocs_html(
    name = "build",
    srcs = [
        "mkdocs.yml",
        "//docs:files",
        "//assets:files",
    ] + glob([
        "tailwind_theme/*",
        "tailwind_theme/**",
    ]),
    config = "mkdocs.yml",
    strip_path = package_name(),
)

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
