load("@rules_python//python:defs.bzl", "py_binary")
load("@zilliqa_python_deps//:requirements.bzl", "requirement")

py_binary(
    name = "mkdocs_wrapper",
    srcs = ["mkdocs_wrapper.py"],
    visibility = ["//visibility:public"],
    deps = [
        requirement("click"),
        requirement("ghp-import"),
        requirement("Jinja2"),
        requirement("Markdown"),
        requirement("MarkupSafe"),
        requirement("mergedeep"),
        requirement("mkdocs"),
        requirement("packaging"),
        requirement("pyparsing"),
        requirement("python-dateutil"),
        requirement("PyYAML"),
        requirement("pyyaml_env_tag"),
        requirement("six"),
        requirement("watchdog"),
        requirement("pymdown-extensions"),
        requirement("mkdocs-material"),
    ],
)

exports_files(["mkdocs_wrapper.py"])
