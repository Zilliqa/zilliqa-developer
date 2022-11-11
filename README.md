# Zilliqa Developer Tools & Documnentation

`zilliqa-developer` is a Bazel based monorepo that contains SDKs, documentations
and products used to develop solutions based on the Zilliqa ecosystem.

This reposository is organised as follows:

- `docs/`: Pure documentation in `md` or `mdx` format.
- `examples/`: Reference material.
- `zilliqa/`: APIs and libraries (JS SDK, Python SDK, etc/)
- `products/`: Software products(`ceres`, `devex`, `neosavant`, `oil` etc.)

Detailed documentation of the Zilliqa ecosystem is found in [docs/](docs/).

Product targets contained in this repository are released as follows: TODO(tfr):
following table is a place holder:

| Target                        | Release page        | Notes             |
| ----------------------------- | ------------------- | ----------------- |
| `//products/developer-portal` | portal.zilliqa.com  | Contains `//docs` |
| `//products/dev-wallet`       | wallet.zilliqa.com  |                   |
| `//products/dev-ex`           | explore.zilliqa.com |                   |
| `//products/ceres`            | zilliqa.com/apps    |                   |

## Building

### Prerequisites

This repository is based on the [Bazel build tool](https://bazel.build/). Bazel
builds are mostly self-contained because Bazel downloads dependencies and
arrange them in you workspace. The only external tools we rely on is

- Bazelisk, ibazel or Bazel 5.2
- Pnpm / Npm / Yarn
- Python 3.6 or newer
- Trunk 1.0 or newer

While the repository can be built directly with Bazel, we recommend that you
either use Bazelisk or ibazel as these will manage the Bazel version used. The
following guide assumes you will be using Bazelisk.

On most platforms you can install Bazelisk using NPM:

```sh
npm install -g @bazel/bazelisk
```

On macOS, Bazelisk is also available using `brew`:

```sh
brew install bazelisk
```

Likewise on Windows, it can be installed using `choco`:

```sh
choco install bazelisk
```

Once installed, verify that Bazelisk is correctly installed by running

```sh
bazelisk --help
```

In case of issues, please refer to the
[official documentation](https://www.npmjs.com/package/@bazel/bazelisk).

### Building

To build a target, run

```sh
bazelisk build [target_name]
```

where `target_name` is the target you want to build. Targets can be found in the
respective `BUILD` files and we will also briefly cover how to find them from
the commandline in the [Listing targets](#listing-targets) section.

As a concrete example, you can build the all the Zilliqa Javascript SDK targets
as:

```sh
bazelisk build //zilliqa/js/...
```

or you can pick out a specific target as

```sh
bazelisk build //zilliqa/js/util:pkg
```

#### Building libraries

#### Building and running Docker images

For the purpose of building Docker images, you do not need Docker installed
whereas if you wish to run the generated image, you do need Docker.

To build the image execute:

```sh
bazelisk build //products/isolated-server:latest
```

To populate the image to the local Docker registry:

```sh
bazelisk run //products/isolated-server:latest
```

You can now verify that the image is in your local registry by running:

```sh
docker images | grep products/isolated-server:latest
```

To run the image, you need Docker installed

```sh
docker run -it products/isolated-server:latest
```

### Running executable targets

Bazel can run an executable target directly from the build tool. This is done as

```sh
bazelisk run [target_name]
```

This can be used while developing products.

### Testing

Testing follows the same pattern as described above, but using the `test`
command instead:

```sh
bazelisk test [target_name]
```

Similar to when building, you can request to run multiple tests:

```sh
bazelisk test //zilliqa/js/...
```

The above command would run all tests related to the Zilliqa Javascript SDK.
TODO(tfr): No tests enabled yet.

### Code formatting and style checking

To maintain a consistent style accross the repository we use
[trunk.io](https://trunk.io) to manage various linters and code formatters.

To check your code:

```sh
trunk check
```

To format your code where possible:

```sh
trunk fmt
```

This step is enforced by CI.

### Listing targets

Often it is useful to list targets and/or dependencies of targets. Here we
provide a few To list all targets:

```sh
bazelisk query "//..."
```

To list targets in a subfolder:

```sh
bazelisk query "//zilliqa/..."
```

To find the dependencies of at target

```sh
bazelisk query "deps(//zilliqa/js/util:pkg)"
```

## Useful notes on Bazel

To get verbose error messages add `--verbose_failures`.

If you experience issues with Bazel determining relevant toolchain for C++, try
adding the argument
`--toolchain_resolution_debug=@bazel_tools//tools/cpp:toolchain_type` to help
debug.

If you experience that it is difficult to understand what folder structure Bazel
builds, add `--sandbox_debug`

Sometimes while debugging it is helpful to clean all to aviod artefacts from
previous builds: `bazel clean --expunge`

To get information about your current Bazel setup run `bazelisk info`.

## Reasoning Behind Repository Organisation

This reposository is organised as follows:

- `docs/`: Pure documentation in `md` or `mdx` format.
- `exmaples/`: Reference material.
- `zilliqa/`: APIs and libraries (JS API, GO API, etc/)
- `products/`: Software products(`ceres`, `devex`, `neosavant`, `oil` etc.)

The idea with keeping it all in one repository is as follows:

1. When someone would make a breaking change to the JS Api which would break
   `ceres`, `devex` and two API examples this would be caught before the change
   makes it into `main`, and whoever responsbile for the change would also be
   responsible for updating all broken products as part of that update.

2. The documentation and examples are kept closer to the source code hence
   making it possible to request an update of docs during the review of a new
   code piece (as opposed to create a ticket on the backlog, which is under the
   danger of never being addressed)

3. For our different components, it would be substantially easier to make sure
   that all are using the same version of a given library. For instance, if the
   JS API is depending on `BN.js` version 4.11.8 but the `devex` product uses
   version 5.2.1, this possibly leads to incompatibility between `devex` and the
   API component. By keeping the two together, we can ensure that releases are
   done simultaneously and that they rely on the same library version to avoid
   tedious bugs that are hard to find.
