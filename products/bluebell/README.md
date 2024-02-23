# Bluebell Scilla Compiler

This README provides examples of how to use the Bluebell command line tool.

## Running a Scilla File

To run a Scilla file, use the `Run` command followed by the `--entry_point` flag to specify the function to invoke, the `--args` flag to pass arguments to the function, and the `--backend` flag to specify the backend to use. Here is an example:

```bash
cargo run --bin cli -- examples/hello-world.scilla --runtime-enable debug run --backend evm  --entry-point "HelloWorld::setHello" --args "[\"Zilliqa ❤️ Rocks\"]"
```

This command will run the `main` function of the `hello-world.scilla` file with the argument `[\"Zilliqa ❤️ Rocks\"]` using the EVM backend. This should produce an output similar to

```bash
Zilliqa ❤️ Rocks


Exit reason: Succeed(
    Returned,
)Result: []%
```

This is a basic example of compiling a Scilla contract into EVM bytecode and running the code in a EVM instance. Note that this example makes use of external precompiles which provides the `print` command to print `Zilliqa ❤️ Rocks` to the terminal.

## Running the playground

To set up Rust to run the playground, first follow these steps:

1. Install Rust by following the instructions on the official Rust website.
2. Add WebAssembly (Wasm) support to your Rust setup by running `rustup target add wasm32-unknown-unknown`.
3. Install Trunk by running `cargo install trunk`.
4. Navigate to your project `playground/`.
5. Run `trunk serve` to start the development server.

A more detailed guide can be found in the playground's [README.md](playground/README.md) file.
