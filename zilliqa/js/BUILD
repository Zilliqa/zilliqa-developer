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
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "bsert.d",
    srcs = ["typings/bsert.d.ts"],
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "hmac-drbg.d",
    srcs = ["typings/hmac-drbg.d.ts"],
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "json.d",
    srcs = ["typings/json.d.ts"],
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "pbkdf2.d",
    srcs = ["typings/pbkdf2.d.ts"],
    visibility = ["//visibility:public"],
)

copy_to_bin(
    name = "window.d",
    srcs = ["typings/window.d.ts"],
    visibility = ["//visibility:public"],
)
