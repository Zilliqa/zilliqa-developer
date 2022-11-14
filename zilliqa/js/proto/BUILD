load("@aspect_rules_js//npm:defs.bzl", "npm_package", "stamped_package_json")
load("@rules_proto//proto:defs.bzl", "proto_library")
load(":defs.bzl", "ts_proto_library")

stamped_package_json(
    name = "package",
    stamp_var = "STABLE_JS_API_BUILD_VERSION",
)

proto_library(
    name = "proto_lib",
    srcs = ["src/messages.proto"],
)

ts_proto_library(
    name = "lib",
    srcs = [":package"],
    proto = ":proto_lib",
    visibility = ["//visibility:public"],
)

npm_package(
    name = "pkg",
    srcs = [":lib"],
    package = "@zilliqa-js/proto",
    visibility = ["//visibility:public"],
)
