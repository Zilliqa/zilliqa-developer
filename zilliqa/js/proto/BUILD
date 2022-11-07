load("@rules_proto//proto:defs.bzl", "proto_library")
load("@build_bazel_rules_nodejs//:index.bzl", "pkg_npm")
load("@build_bazel_rules_nodejs//:index.bzl", "js_library")
load(":defs.bzl", "protobufjs_library")

proto_library(
    name = "src_js",
    srcs = [
        "src/messages.proto",
    ],
)

protobufjs_library(
    name = "lib",
    package_name = "@zilliqa-js/proto",    
    proto = "src_js",
    srcs = ["package.json"],
     visibility = ["//visibility:public"]    

)


pkg_npm(
    name = "package",
    package_name = "@zilliqa-js/proto",
    srcs = ["package.json"],
    deps = [":src_js"],
 	 visibility = ["//visibility:public"]    
)