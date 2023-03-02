"""
Tiny function to collect our JUnit tests
"""

load("@rules_java//java:defs.bzl", "java_test")

def run_tests(name, srcs, deps, src_prefix):
    """
    The single function in this module.

    Args:
      name: Unused
      srcs: List of test source files
      deps: List of dependencies for each test
      src_prefix: Prefix to remove from test filenames before turning them into Java class names
    Returns:
      The list of tests we generated.
    """
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
