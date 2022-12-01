load("@rules_python//python:defs.bzl", "py_binary", "py_library")
load("@zilliqa_python_deps//:requirements.bzl", "requirement")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")

expand_workspace_status(
    name = "git-version-py",
    false_value = "False",
    output = "version/__init__.py",
    template = "version/version.tpl.py",
    true_value = "True",
)

py_library(
    name = "version",
    srcs = [":git-version-py"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":git-version-py",
    ],
)

py_binary(
    name = "submit_patch",
    srcs = [
        "submit_patch.py",
    ],
    args = [
        "$(location //products/developer-portal:cd_patch)",
    ],
    data = [
        "//products/developer-portal:cd_patch",
    ],
    visibility = ["//visibility:public"],
    deps = [
        ":version",
        requirement("PyGithub"),
    ],
)
