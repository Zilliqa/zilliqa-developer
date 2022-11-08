exports_files(["package.json", "tsconfig.base.json"])
exports_files(["docs"])

# Compiler settings
config_setting(
    name = "clang_compiler",
    flag_values = {"@bazel_tools//tools/cpp:compiler": "clang"},
)

config_setting(
    name = "gcc_compiler",
    flag_values = {"@bazel_tools//tools/cpp:compiler": "gcc"},
)


# Platform constraints
platform(
    name = "apple-silicon",
    constraint_values = [
        "@platforms//cpu:aarch64",
        "@platforms//os:osx",
    ],
)

platform(
    name = "apple-x86",
    constraint_values = [
        "@platforms//cpu:x86_64",
        "@platforms//os:osx",
    ],
)

platform(
    name = "linux-x86",
    constraint_values = [
        "@platforms//cpu:x86_64",
        "@platforms//os:linux",
    ],
)