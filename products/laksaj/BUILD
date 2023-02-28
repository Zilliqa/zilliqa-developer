filegroup(
    name = "srcs",
    srcs = glob(["src/**/*.java"]),
    visibility = ["//visibility:public"])

java_plugin(
    name = "lombok-java",
    processor_class = "lombok.launch.AnnotationProcessorHider$AnnotationProcessor",
    generates_api = True,
    deps = [ "@maven//:org_projectlombok_lombok" ],
    visibility = ["//visibility:public"],
)

java_library(
    name = "laksaj.jar",
    plugins = [
        ":lombok-java"
    ],
    deps = [
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
    ],
    srcs = [
        ":srcs"
    ],
    data = [
    ])


