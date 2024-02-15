# eth-spout - A super simple faucet for Ethereum testnets

## Features

A static site which lets users request funds from a pre-configured account.

## Configuration

All configuration is passed via environment variables.

| Variable | Description |
| -------- | ----------- |
| `HTTP_PORT` | The port to serve the site at. Defaults to `80`. |
| `RPC_URL` | The HTTP URL of an Ethereum RPC endpoint. Required. |
| `NATIVE_TOKEN_SYMBOL` | The symbol of the native token for the chain. Defaults to `ETH`. |
| `PRIVATE_KEY` | The private key of the account to send funds from. Required. |
| `ETH_AMOUNT` | The amount to send for each request in Eth. Defaults to `1`. |
| `EXPLORER_URL` | The URL to a block explorer for this chain, used to provide a link to the transaction after a request succeeds. Optional. |
| `MINIMUM_SECONDS_BETWEEN_REQUESTS` | The minimum time between each request to send funds to the same address. Set to `0` to disable rate limiting. Defaults to `300`. |
| `BECH32_HRP` | The expected human-readable part (HRP) of provided bech32 addresses. Optional. |
