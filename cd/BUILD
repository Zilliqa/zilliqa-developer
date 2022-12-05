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
    name = "create_preview",
    srcs = [
        "update_devops_repository.py",
    ],
    args = [
        # Preview updates
        "$(location //products/developer-portal:cd_preview_patch)",
    ],
    data = [
        "//products/developer-portal:cd_preview_patch",
    ],
    main = "update_devops_repository.py",
    visibility = ["//visibility:public"],
    deps = [
        ":version",
        requirement("PyGithub"),
    ],
)

py_binary(
    name = "create_production",
    srcs = [
        "update_devops_repository.py",
    ],
    args = [
        # Preview updates
        "$(location //products/developer-portal:cd_preview_patch)",

        # Production updates
        "$(location //products/developer-portal:cd_production_patch)",

        # Production build
        "!production",
    ],
    data = [
        "//products/developer-portal:cd_preview_patch",
        "//products/developer-portal:cd_production_patch",
    ],
    main = "update_devops_repository.py",
    visibility = ["//visibility:public"],
    deps = [
        ":version",
        requirement("PyGithub"),
    ],
)
