load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library", "cc_test")


def zilliqa_cc_library(
        name,
        srcs = [],
        hdrs = [],
        copts = None,
        visibility = None,
        external_deps = [],
        tcmalloc_dep = None,
        repository = "",
        linkopts = None,
        linkstamp = None,
        tags = [],
        linkshared = 1,
        deps = [],
        strip_include_prefix = None,
        textual_hdrs = None):
    if not visibility:
        visibility = ["//visibility:private"]

    coptions = [
        "-Wall",
        "-Wextra",
        "-Wconversion",
        "-Wpedantic",
        "-Werror",
        "-mavx2",
        "-DWONOP_COMPILE_LOGGING_LEVEL=4",
        "-DWONOP_ENABLE_BACKTRACE",
    ]

    if copts:
        coptions.extend(copts)
    cc_library(
        name = name,
        srcs = srcs,
        hdrs = hdrs,
        copts = coptions,
        visibility = visibility,
        tags = tags,
        textual_hdrs = textual_hdrs,
        deps = deps,
        linkopts = linkopts,
        include_prefix = None,
        alwayslink = 1,
        linkstatic = True,
        strip_include_prefix = strip_include_prefix,
    )

def zilliqa_cc_binary(
        name,
        srcs = [],
        data = [],
        args = [],
        visibility = None,
        copts = None,
        deps = [],
        linkopts = [],
        linkstatic = True,
        linkshared = False):
    if not visibility:
        visibility = ["//visibility:private"]

    coptions = [
        "-Wall",
        "-Wextra",
        "-Wconversion",
        "-Wpedantic",
        "-Werror",
        "-mavx2",
        "-DWONOP_COMPILE_LOGGING_LEVEL=4",
        "-DWONOP_ENABLE_BACKTRACE",
    ]

    if copts:
        coptions.extend(copts)
    cc_binary(
        name = name,
        srcs = srcs,
        copts = coptions,
        deps = deps,
        visibility = visibility,
        data = data,
        args = args,
        linkopts = linkopts,
        linkstatic = linkstatic,
        linkshared = linkshared,
    )

def zilliqa_cc_test(
        name,
        srcs = [],
        data = [],
        args = [],
        copts = None,
        deps = [],
        tags = [],
        timeout = "moderate"):
    coptions = [
        "-Wall",
        "-Wextra",
        "-Wconversion",
        "-Wpedantic",
        "-Werror",
        "-mavx2",
        "-DWONOP_COMPILE_LOGGING_LEVEL=4",
        "-DWONOP_ENABLE_BACKTRACE",
    ]

    if copts:
        coptions.extend(copts)
    cc_test(
        name = name,
        srcs = srcs,
        copts = coptions,
        deps = deps,
        data = data,
        args = args,
        tags = tags,
        timeout = timeout,
    )
