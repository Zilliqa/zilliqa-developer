load("@npm//:@docusaurus/core/package_json.bzl", docusaurus_bin = "bin")
# load("@npm//:react-app-rewired/package_json.bzl", cra_bin = "bin")

load("//:config/docusaurus.bzl", "docusaurus_pkg")

docusaurus_bin.docusaurus_binary(name = "docusaurus_exe")

docusaurus_pkg(
    name = "test_docs",
    srcs = ["//docs:files"] + glob(
        include = [
            "src/**/*",
        ],
    ),
    binary = ":docusaurus_exe",
    config = "docusaurus.config.js",
    sidebars = "sidebars.js",
    strip_path = package_name(),
)
