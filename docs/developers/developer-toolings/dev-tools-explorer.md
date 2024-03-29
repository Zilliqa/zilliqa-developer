---
id: dev-tools-explorer
title: Explorers
keywords:
  - explorer
  - viewblock
  - devex
  - api
  - zilliqa
  - token
description: Devex and ViewBlock Explorer
---

---

## Devex

!["Viewblock"](../../assets/img/dev-dapps/tools/devex.png)

Devex is a developer explorer that is maintained by Zilliqa Research. Devex is
integrated directly into developer tools like Ceres and Scilla IDE. Devex allows
you to look at blocks, address, transactions, status of the Zilliqa network as
well as contract code, variables and any transition events.

!["Viewblock"](../../assets/img/dev-dapps/tools/devexNetworkChange.png)

Devex is available at this [link](https://devex.zilliqa.com/), by default, devex
points to the Zilliqa mainnet but you can change the network by using the
dropdown button at the top right corner.

## ViewBlock

!["Viewblock"](../../assets/img/dev-dapps/tools/viewblock.png)

ViewBlock allows you to look at blocks, address, transactions, status of the
Zilliqa network, contract code and variables.

| Network | Link                                                                                         |
| ------- | -------------------------------------------------------------------------------------------- |
| Mainnet | [https://viewblock.io/zilliqa](https://viewblock.io/zilliqa)                                 |
| Testnet | [https://viewblock.io/zilliqa?network=testnet](https://viewblock.io/zilliqa?network=testnet) |

For example,
[this](https://viewblock.io/zilliqa/tx/c4030c73d6dae558ff0c9d98237101e342888115f13219a00bb14a8ee46fa3be?network=testnet)
is the link to a `getHello()` transition transaction.

If you create a legit token and have a logo, head over to the
[cryptometa](https://github.com/Ashlar/cryptometa) repository and follow the
Readme instructions.

### Viewblock products

[ViewBlock API](https://viewblock.io/api) - ViewBlock's API is a valuable
resource for some additional methods that a developer might require for his
application (e.g retrieving transactions sent by a particular address).

[ViewBlock Zilliqa Stats](https://viewblock.io/zilliqa/stats) - ViewBlock's
statistics provide statistical information about Zilliqa network such as
difficulty over time, number of blocks over time and address growth over time.

[ViewBlock Dashboard](https://dash.viewblock.io/d/zilliqa) - Is an advanced
dashboard showing more statistics which allows you to select any time range you
desire with a custom granularity
