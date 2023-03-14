load("@rules_python//python:defs.bzl", "py_binary", "py_library")
load("@zilliqa_python_deps//:requirements.bzl", "requirement")
load("//config:expand-workspace-status.bzl", "expand_workspace_status")
load(":applications.bzl", "APPLICATIONS")

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
        "$(location //products/devex:cd_preview_patch)",
    ],
    data = [
        "//products/developer-portal:cd_preview_patch",
        "//products/devex:cd_preview_patch",
    ],
    main = "update_devops_repository.py",
    visibility = ["//visibility:public"],
    deps = [
        ":version",
        requirement("PyGithub"),
    ],
)

py_binary(
    name = "update_staging",
    srcs = [
        "update_manifests_repository.py",
    ],
    args = ["$(location {}:cd_staging_patch)".format(s) for s in APPLICATIONS],
    data = ["{}:cd_staging_patch".format(s) for s in APPLICATIONS],
    main = "update_manifests_repository.py",
    visibility = ["//visibility:public"],
    deps = [
        ":version",
        requirement("PyGithub"),
    ],
)

py_binary(
    name = "update_production",
    srcs = [
        "update_manifests_repository.py",
    ],
    args = ["$(location {}:cd_production_patch)".format(s) for s in APPLICATIONS],
    data = ["{}:cd_production_patch".format(s) for s in APPLICATIONS],
    main = "update_manifests_repository.py",
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
        "$(location //products/devex:cd_preview_patch)",

        # Production updates
        "$(location //products/developer-portal:cd_production_patch)",
        "$(location //products/devex:cd_production_patch)",
    ],
    data = [
        # Preview updates
        "//products/developer-portal:cd_preview_patch",
        "//products/devex:cd_preview_patch",

        # Production updates
        "//products/developer-portal:cd_production_patch",
        "//products/devex:cd_production_patch",
    ],
    main = "update_devops_repository.py",
    visibility = ["//visibility:public"],
    deps = [
        ":version",
        requirement("PyGithub"),
    ],
)
