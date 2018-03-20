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

Enjoy!

