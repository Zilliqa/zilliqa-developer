load("@rules_java//java:defs.bzl", "java_test")

def run_tests(name, srcs, deps, src_prefix):
    test_list = []
    for src in srcs:
        src_name = src[:-5]

        # replace `/` with `.`
        pkg_name = src_name[len(src_prefix):].replace("/", ".")

        #print(pkg_name)
        test_list.append(src_name)
        java_test(
            name = src_name,
            test_class = pkg_name,
            srcs = srcs,
            deps = deps,
            size = "small",
            timeout = "eternal",
        )
    return test_list
