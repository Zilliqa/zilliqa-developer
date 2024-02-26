# Getting started

## Prerequisites

Before you proceed, ensure you have the following prerequisites installed on
your system:

- For CLI: Install Rust by following the instructions from
  [Rust's official documentation](https://rust-lang.org/).

- For Playground:
  - Install Rust nightly: `rustup install nightly`
  - Install `trunk`: `cargo install trunk`
  - Install WebAssembly (wasm): `rustup target add wasm32-unknown-unknown`
  - Install `tailwindcss`: `npm install tailwindcss`

### Installation

To compile the project, run:

```
cargo build
```

### Run CLI

To run the CLI, use the following command:

```
cargo run --bin cli -- examples/hello-world.scilla --runtime-enable debug run --backend evm  --entry-point "HelloDebugModule::setHello" --args "[\"Zilliqa ❤️  Rocks\"]"
```

### Run Playground

Navigate to the playground directory and set up the environment:

```
cd playground
NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
trunk serve
```

This will serve the playground for you to interact with.

## Hello world with Scilla

Here's a simple Hello World example in Scilla:

```scilla
scilla_version 0

library HelloDebugModule

contract HelloDebugModule()

transition setHello (msg : Uint64)
  x = builtin print msg
end
```

You can run the above code using:

```
cargo run --bin cli -- examples/hello-world.scilla --runtime-enable debug run --backend evm  --entry-point "HelloDebugModule::setHello" --args "[\"Zilliqa ❤️  Rocks\"]"
```

## Working with State

The following contract demonstrates how to work with state in Scilla:

```scilla
scilla_version 0

library HelloState

contract HelloState()

field welcome_msg : Uint64 = Uint64 0

transition setHello (msg : Uint64)
  welcome_msg := msg;
  msg <- welcome_msg;
  x = builtin print msg
end
```

## Default values

Demonstrating default values in Scilla:

```scilla
scilla_version 0

library DefaultValuesExample

contract DefaultValuesContract()

field data : Uint128 = Uint128 100

transition UpdateData(newData : Option Uint128)
  match newData with
  | Some val =>
    data := val
  | None =>
    (* Do nothing, keep the default value *)
  end
end
```

This completes the basic introductory tutorial for Scilla. Make sure to consult
the official documentation for more detailed insights.
