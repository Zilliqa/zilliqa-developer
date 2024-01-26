# Testing information

1.Run two anvil chains:

```sh
anvil -p 8545 --chain-id 1
```

```sh
anvil -p 8546 --chain-id 2
```

2.Start:

```sh
# First account as leader
cargo run 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 -l
```

```sh
# Second account
cargo run 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
```

2.Run `smart-contract` forge commands to test if bridge works

## Bootstrap with docker-compose

Automated bootstrap of a 4 nodes Zilliqa 2.0 aka zq2 network.

Build the images first:

```bash
docker build . -t bridge-validator-node
```

Then run:

```bash
docker-compose up
```
