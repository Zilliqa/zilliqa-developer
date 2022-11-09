load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")

filegroup(
    name = "typings",
    srcs = glob(
        [
            "typings/**/*.d.ts",
            "typings/*.d.ts",
        ],
    ),
    visibility = [
        "//visibility:public",
    ],
)

copy_to_bin(
    name = "aes-js.d",
    srcs = ["typings/aes-js.d.ts"],
    visibility = ["//zilliqa/js/util:__subpackages__"],
)
