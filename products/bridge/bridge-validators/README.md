# Testing information

1.Run two anvil chains:

```sh
anvil -p 8545 --chain-id 1
```

```sh
anvil -p 8546 --chain-id 2
```

2.Export `.env`:

```sh
./load_dotenv.sh
```

3.Start:

```sh
cargo run
```

4.Run `smart-contract` forge commands to test if bridge works
