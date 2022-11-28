filegroup(
    name = "files",
    srcs = glob(
        [
            "**/*",
        ],
        exclude_directories = 1,
    ),
    visibility = [
        "//visibility:public",
    ],
)

# pkg_tar(
# 	name = "raw-tar",
# 	strip_prefix=".",
# 	srcs=glob(
# 	    ["**/*.md","**/*.mdx", "**/*.png", "**/*.jpg"],
# 	    exclude_directories=0
# 	),
#     mode="0755",
#     deps=[],
#     package_dir="docs",
# 	visibility = [
# 		"//visibility:public"
# 	]
# )
