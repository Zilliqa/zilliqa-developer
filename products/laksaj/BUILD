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
    srcs = [
        ":srcs"
    ],
    data = [
    ])


