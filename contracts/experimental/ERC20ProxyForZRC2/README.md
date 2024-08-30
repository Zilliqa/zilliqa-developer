# ERC20ProxyForZRC2 Contract

These contracts allow ZRC-2 tokens to look like ERC-20 tokens.

Unless you want to build using the `zilliqa-developer` version of `zilliqa-js`, or you encounter missing modules issues while running tests, install our dependencies with:

```shell
pnpm --ignore-workspace i
```

## Deploying a proxy

You can deploy a proxy with:

```shell
export PRIVATE_KEY=<...>
pnpm exec hardhat deployProxy 0x5DD38E64dA8f7d541d8aF45fe00bF37F6a2c6195 --network zq-testnet
```

If your ZRC-2 is burnable (ie. supports the `Burn()` transition), you can use:

```shell
export PRIVATE_KEY=<...>
pnpm exec hardhat deployProxyBurnable 0x5DD38E64dA8f7d541d8aF45fe00bF37F6a2c6195 --network zq-testnet
```

The task should automatically verify these contracts to sourcify.

## Networks

Various networks are available in the `hardhat.conf.ts`:

- `zq-testnet` - the Zilliqa 1 testnet
- `zq` - the Zilliqa 1 mainnet
- `local-proxy` - a local proxy.

You can use the `local-proxy` network and run:

```sh
mitmweb --mode reverse:https://dev-api.zilliqa.com --no-web-open-browser --listen-port 5556 --web-port 5557
```

To monitor requests.

## Testing

To run the tests:

```shell
export PRIVATE_KEY=<...>
export TEST_KEY_1=<...>
export TEST_KEY_2=<...>
pnpm exec hardhat test --network zq-testnet
```

Each test has a number prefix so you can select them individually.

If you set the `CACHED` environment variable, we will use a built-in
cached contract deployment whose addresses appear in the source -
please update it if you change the contracts.

This allows you to run tests quickly, without waiting for contract
deployment.
