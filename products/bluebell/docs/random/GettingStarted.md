## Playground

```sh
cd playground
trunk serve
```

## CLI

```sh
cargo run --bin cli -- examples/target4.scilla --runtime-enable debug run --backend evm  --entry-point "HelloWorld::setHello" --args "[\"Zilliqa ❤️  Rocks\"]"
```
