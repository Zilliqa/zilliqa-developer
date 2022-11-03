# Zilliqa Developer Tools & Documnentation

## Change log

- All document repositories combined in `docs/`
- Framework for building static webpage separated into `products/developer-portal` to allow clean `docs/` layout that is usable from Github.

## Encountering Bazel Issues

To get verbose error messages add `--verbose_failures`.

If you experience issues with Bazel determining relevant toolchain for C++, try adding the argument `--toolchain_resolution_debug=@bazel_tools//tools/cpp:toolchain_type` to help debug.

If you experience that it is difficult to understand what folder structure Bazel builds, add `--sandbox_debug`

Sometimes while debugging it is helpful to clean all to aviod artefacts from previous builds: `bazel clean --expunge`
