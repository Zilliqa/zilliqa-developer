load("@rules_java//java:defs.bzl", "java_plugin")
load("@rules_jvm_external//:defs.bzl", "java_export")
load(":defs.bzl", "run_tests")

filegroup(
    name = "srcs",
    srcs = glob(["src/main/**/*.java"]),
    visibility = ["//visibility:public"],
)

filegroup(
    name = "test_srcs",
    srcs = glob(["src/test/**/*.java"]),
    visibility = ["//visibility:public"],
)

java_plugin(
    name = "lombok-java",
    generates_api = True,
    processor_class = "lombok.launch.AnnotationProcessorHider$AnnotationProcessor",
    visibility = ["//visibility:public"],
    deps = ["@maven//:org_projectlombok_lombok"],
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
    "@maven//:org_bouncycastle_bcprov_jdk18on",
]

LAKSAJ_VERSION = "2.0.0rc3-SNAPSHOT"

java_export(
    name = "laksaj",
    srcs = [
        ":srcs",
    ],
    data = [
    ],
    # @TODO make this more dynamic.  There are a few issues here: the
    # release process is, basically, manual, and so moving it to CICD
    # isn't all that useful.
    # The version number is pretty sensitive - needs to be increasing, and
    # not overlap the previously issued numbers.
    # Almost all variables are forbidden to macros (and java_export is quite
    # a complicated macro) :-(
    maven_coordinates = "com.zilliqa:laksaj:%s" % LAKSAJ_VERSION,
    plugins = [
        ":lombok-java",
    ],
    pom_template = "pom.tmpl",
    visibility = ["//:__subpackages__"],
    deps = common_deps,
)

test_list = run_tests(
    name = "test",
    srcs = glob(["src/test/**/*.java"]),
    src_prefix = "src/test/java/",
    deps = [
        ":laksaj",
    ] + common_deps,
)

test_suite(
    name = "tests",
    tests = test_list,
)
