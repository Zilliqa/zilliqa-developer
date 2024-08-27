# ERC20ProxyForZRC2 Contract

Unless you want to build using the `zilliqa-developer` version of `zilliqa-js`, install with:

```shell
pnpm i --ignore-workspace
```


```shell
export PRIVATE_KEY=<...>
pnpm exec hardhat deployProxy 0x5DD38E64dA8f7d541d8aF45fe00bF37F6a2c6195 --network zq-testnet
```

To run tests:

```shell
export PRIVATE_KEY=<...>
pnpm exec hardhat test --network zq-testnet
```



This is the contract to deploy a ERC20Proxy for a ZRC2 contract living in the scilla environment. It leverages the precompiles available in Zilliqa to interoperate between the 2 environments.

Make sure to specify the `zrc2_address` on the deployment file for the ERC20Proxy to be correctly deployed. This allows EVM to execute all desired functions on the ZRC2 as if it were a ERC20. Implementing IERC20 means that all existing DApps and wallets should be compatible with this token.

Make sure to also copy `.env.example` into `.env` and fill in the necessarily variables. Also ensure that `pnpm install` to install any necessary dependencies

The following are the deployment commands:

- Zilliqa Mainnet

  ```shell
  pnpm exec hardhat run scripts/deploy.ts --network zq
  ```

- Zilliqa Testnet

  ```shell
  pnpm exec hardhat run scripts/deploy.ts --network zq-testnet
  ```
