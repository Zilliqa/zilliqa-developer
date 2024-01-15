---
id: dev-started-env
title: Development Environments
keywords:
  - development environments
  - isolated server
  - mainnet
  - testnet
  - api
  - chain id
  - websocket endpoint
  - zilliqa
description: Zilliqa Development Environments - Testnet, Mainnet & Isolated Server
---

---

## Zilliqa Mainnet

|                                        | URL(s)                                                                                                                                                      |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **API URL**                            | [https://api.zilliqa.com/](https://api.zilliqa.com/)                                                                                                        |
| **EVM API**                            | [https://api.zilliqa.com/](https://api.zilliqa.com/) or [http://evm-api-filters.zilliqa.com/](http://evm-api-filters.zilliqa.com/) (if using eth_subscribe) |
| **Block Explorer**                     | [**Link**](https://viewblock.io/zilliqa)                                                                                                                    |
| **Devex explorer**                     | [**Link**](https://devex.zilliqa.com/)                                                                                                                      |
| **EVM Block Explorer**                 | [Ethereal EVM explorer](https://evmx.zilliqa.com/overview)                                                                                                  |
| **WebSocket endpoint**                 | `wss://api-ws.zilliqa.com`                                                                                                                                  |
| **Chain ID**                           | 1                                                                                                                                                           |
| **EVM Chain ID**                       | 32769                                                                                                                                                       |
| **VERSION**                            | v9.3.0                                                                                                                                                      |
| **REWARD CONTROL CONTRACT ADDRESS**    | 0xbce4ee32539760622a2fe2b7cc67b089aa11b63a                                                                                                                  |
| **PERSISTENCE BUCKET (S3 compatible)** | `s3://zq1-mainnet-persistence` (use: `AWS_ENDPOINT_URL=https://storage.googleapis.com`)                                                                     |
| **PERSISTENCE FOLDER NAME**            | mainnet-v930                                                                                                                                                |

## Developer Testnet

|                                        | URL(s)                                                                                  |
| :------------------------------------- | :-------------------------------------------------------------------------------------- |
| **API URL**                            | [https://dev-api.zilliqa.com/](https://dev-api.zilliqa.com/)                            |
| **EVM URL**                            | [https://evm-api-dev.zilliqa.com/](https://evm-api-dev.zilliqa.com/)                    |
| **Faucet**                             | [**Link**](https://dev-wallet.zilliqa.com/home?network=testnet)                         |
| **Block Explorer**                     | [**Link**](https://viewblock.io/zilliqa?network=testnet)                                |
| **EVM Block Explorer**                 | [**Link**](https://otterscan.testnet.zilliqa.com)                                       |
| **WebSocket endpoint**                 | `wss://dev-ws.zilliqa.com`                                                              |
| **Chain ID**                           | 333                                                                                     |
| **EVM Chain ID**                       | 33101                                                                                   |
| **VERSION**                            | v9.3.0rc19                                                                              |
| **REWARD CONTROL CONTRACT ADDRESS**    | 0x489F0Ec426DF9343A5F6D7B170B0Bca08e6a81CE                                              |
| **PERSISTENCE BUCKET (S3 compatible)** | `s3://zq1-testnet-persistence` (use: `AWS_ENDPOINT_URL=https://storage.googleapis.com`) |
| **PERSISTENCE HTTPS ENDPOINT**         | `https://persistence.testnet.zq1.dev`                                                   |
| **PERSISTENCE FOLDER NAME**            | testnet-v930rc19                                                                        |

## Developer Devnet

|                                        | URL(s)                                                                                 |
| :------------------------------------- | :------------------------------------------------------------------------------------- |
| **API URL**                            | [https://api.devnet.zilliqa.com/](https://api.devnet.zilliqa.com/)                     |
| **EVM URL**                            | [https://api.devnet.zilliqa.com/](https://api.devnet.zilliqa.com/)                     |
| **Faucet**                             | [**Link**](https://faucet.devnet.zilliqa.com)                                          |
| **Block Explorer**                     | [**Link**](https://devex.devnet.zilliqa.com)                                           |
| **EVM Block Explorer**                 | [**Link**](https://otterscan.devnet.zilliqa.com)                                       |
| **WebSocket endpoint**                 | `wss://wss.devnet.zilliqa.com`                                                         |
| **Chain ID**                           | 617                                                                                    |
| **VERSION**                            | v9.3.0rc19                                                                             |
| **EVM Chain ID**                       | 33385                                                                                  |
| **REWARD CONTROL CONTRACT ADDRESS**    | 0xE2d79664c088Aec94209F0E657642f8569FC12D8                                             |
| **PERSISTENCE BUCKET (S3 compatible)** | `s3://zq1-devnet-persistence` (use: `AWS_ENDPOINT_URL=https://storage.googleapis.com`) |
| **PERSISTENCE FOLDER NAME**            | devnet-pub-v930                                                                        |

## Isolated Server

Zilliqa Isolated Server is a test server for dApp developers to quickly test
their applications. Transactions are validated immediately, hence improving the
productivity for dApp developers.

|             | URL(s)                                                                                       |
| :---------- | :------------------------------------------------------------------------------------------- |
| **API URL** | [https://zilliqa-isolated-server.zilliqa.com/](https://zilliqa-isolated-server.zilliqa.com/) |
