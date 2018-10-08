
- [ccache](#ccache)
- [clang-format and clang-tidy](#clang-format-and-clang-tidy)
- [git-extras](#git-extras)

# ccache

`ccache` works as a wrapper for C/C++ compiler and dramatically shorten the building time by caching the temporary files safely. It works even when you clean up your entire build (e.g. `git clean -dfx` or `make clean`, if apply).

## Installation

- MacOS: `brew install ccache`
- Ubuntu 16.04: `sudo apt install ccache`

## Usage

### **Speeding up different builds in the same directory**

No configuring needed as CMake will find it and adopt it. It even accelerates build

### **Speeding up different builds in different directories**

Two options needed to be appended to `ccache` config file `$HOME/.ccache/ccache.conf`.

```
hash_dir = false
base_dir = COMMON_ABSOLUTE_PATH
```

> **COMMON_ABSOLUTE_PATH** takes the common absolute path of two or more build directories. (e.g. It will be `/home/user/workspace` if you have two build directories `/home/user/workspace/zilliqa1` and `/home/user/workspace/zilliqa2`)

### **Cleaning up the cache if something went wrong**

See `man ccache` to select either `ccache --clear` or `ccache --cleanup`.

# clang-format and clang-tidy

## Installation

The version (7.0.0+) is required:
- MacOS: 
    ```
    brew install llvm@7
    ```
- Ubuntu 16.04: 

    ```bash
    # from http://apt.llvm.org/ 
    sudo cat <<EOF > /etc/apt/sources.list.d/llvm-7.list
    deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-7 main
    deb-src http://apt.llvm.org/xenial/ llvm-toolchain-xenial-7 main
    EOF
    curl https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
    sudo apt-get update && sudo apt-get install clang-format-7 clang-tidy-7 -y
    ```


Also, `pyyaml` is required: `pip install pyyaml`
 
## Usage

Use `./build.sh style` or add `-DLLVM_EXTRA_TOOLS=ON` flag to cmake

### Check (or fix) coding style violations

- **`make clang-format`**: check the codebase against `.clang-format` and fail upon any violation.
- **`make clang-format-fix`**: apply the suggested changes directly

### Check (or fix) wrong coding practices

- **`make clang-tidy`**: check the codebase following `.clang-tidy` and suggest the changes
- **`make clang-tidy-fix`**: apply the suggested changes directly

## Source

see `cmake/LLVMExtraTools.cmake`.

# git-extras

## Installation

- MacOS: `brew install git-extras`
- Ubuntu 16.04: `sudo apt-get install git-extras`

## Usage

### Checking out a specific pull-request locally

If you want to check someone's pull-request, you can just do

```
git pr PR_NUMBER
```