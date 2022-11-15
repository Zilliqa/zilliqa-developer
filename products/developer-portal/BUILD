load("//:config/docusaurus.bzl", "docusaurus_pkg")

# load("@npm//:react-app-rewired/package_json.bzl", cra_bin = "bin")
docusaurus_pkg(
    name = "test_docs",
    srcs = ["//docs:files"] + glob(
        include = [
            "src/**/*",
        ],
    ),
    config = "docusaurus.config.js",
    sidebars = "sidebars.js",
    strip_path = package_name(),
)
