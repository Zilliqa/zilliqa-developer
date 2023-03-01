load(":defs.bzl", "run_tests")

java_toolchain(
    name = "toolchain",
    source_version = "11",
    target_version = "11",
    bootclasspath = [ "@bazel_tools//tools/jdk:bootclasspath" ],
    javabuilder = [ "@bazel_tools//tools/jdk:javabuilder" ],
    singlejar = [ "@bazel_tools//tools/jdk:singlejar" ],
    genclass = [ "@bazel_tools//tools/jdk:genclass" ],
    ijar = [ "@bazel_tools//tools/jdk:ijar" ],
    java_runtime =  "@bazel_tools//tools/jdk:java_runtime",
    forcibly_disable_header_compilation = True
)

filegroup(
    name = "srcs",
    srcs = glob(["src/main/**/*.java"]),
    visibility = ["//visibility:public"])

filegroup(
    name = "test_srcs",
    srcs = glob(["src/test/**/*.java"]),
    visibility = ["//visibility:public"])

java_plugin(
    name = "lombok-java",
    processor_class = "lombok.launch.AnnotationProcessorHider$AnnotationProcessor",
    generates_api = True,
    deps = [ "@maven//:org_projectlombok_lombok" ],
    visibility = ["//visibility:public"],
)

common_deps = [
        "@maven//:junit_junit",
        "@maven//:org_projectlombok_lombok",
        "@maven//:org_web3j_core",
        "@maven//:org_web3j_crypto",
        "@maven//:org_web3j_utils",
        "@maven//:com_google_guava_guava",
        "@maven//:com_squareup_okhttp3_okhttp",
        "@maven//:com_google_code_gson_gson",
        "@maven//:com_google_protobuf_protobuf_java",
        "@maven//:org_apache_commons_commons_lang3",
        "@maven//:org_bouncycastle_bcprov_jdk18on"
]

java_library(
    name = "laksaj",
    plugins = [
        ":lombok-java"
    ],
    deps = common_deps,
    srcs = [
        ":srcs"
    ],
    data = [
    ])

test_list = run_tests(
    name = "test",
    srcs = glob(["src/test/**/*.java"]),
    deps = [
        ":laksaj"
        ] + common_deps,
    src_prefix = "src/test/java/"
)

test_suite(
    name = "tests",
    tests = test_list
    )

