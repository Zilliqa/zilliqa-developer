# ccache

`ccache` works as a wrapper for C/C++ compiler and dramatically shorten the building time by caching the temporary files safely. It provides acceleration in the following scanerios:

1. reducing the time of an incremental build in the same build directory
2. reducing the time of a fresh build in the same build directory (after things like `make clean` or `git clean -dfx`)
3. reducing the time of a fresh build in a **different directory**.

The first two benefits are immediately available once `ccache` is enabled (see the top-level `CMakeLists.txt`), however, the third one with **caching across different directories** requires two new lines in the `ccache` config file `$HOME/.ccache/ccache.conf`.

> For example, the `PARENT_DIRECTORY_ABSOLUTE_PATH` should be `/home/user/workspace` if you have two build directories `/home/user/workspace/zilliqa1` and `/home/user/workspace/zilliqa2`.

```
hash_dir = false
base_dir = PARENT_DIRECTORY_ABSOLUTE_PATH
```
Enjoy! And if anything wrong is going with `ccache`, just check the clean command in `ccache` and make your choice.

# clang-tidy and clang-format

## Dependencies

The version (5.0.0+) is required:
- MacOS: `brew install llvm@5`
- Ubuntu 16.04: `sudo apt install clang-format-5.0 clang-tidy-5.0 clang-5.0`

## Usage

Four `make` targets defined in `cmake/LLVMExtraTools.cmake`

> **Note**
> 
> Two `clang-tidy` commands are pending, see this [pr](https://github.com/Zilliqa/Zilliqa/pull/148).
> As a result, the travis build will only enforce `clang-format` check presently.
- `make clang-format`: check the codebase against `.clang-format` and fail upon any violation.
- `make clang-format-fix`: apply the suggested changes directly
- ~~`make clang-tidy`: check the codebase following `.clang-tidy` and suggest the changes~~
- ~~`make clang-tidy-fix`: apply the suggested changes directly~~

