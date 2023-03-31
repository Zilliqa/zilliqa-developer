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
