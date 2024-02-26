---
id: dev-quirks
title: Zilliqa EVM quirks
keywords:
  - Developers
  - Dapps
description: Zilliqa EVM architecture and quirks
---

---

Supporting EVM on Zilliqa involves:

- Building an execution engine based on the Ethereum Virtual Machine. This allows us to run EVM programs and, thus, compiled Solidity contracts.
- Supporting enough common precompiles to make common-case contracts work.
- Supporting a "sufficient" subset of the Ethereum API calls. This allows users to create EVM contracts, use common Ethereum tools like hardhat and truffle to write contracts, and interact with Zilliqa using Ethereum-standard software like Metamask.
- Arranging to execute EVM transactions, and store EVM-style state.

There are, inevitably, some differences from other EVM chains, which will be listed here and in the [FAQ](../../faq/faq-introduction.md).

### Execution engine

Our execution engine is based on continuation passing and the [Sputnik VM](https://github.com/rust-blockchain/evm).

This allows us to call Scilla from EVM, via [precompiles](../protocol/protocol-precompiles.md), and, one day, EVM from Scilla via message passing.

### Gas use

Because ETH and ZIL differ in the number of decimals they support, we
scale EVM balances by 6 decimal places.

If we maintained a 1-1 relationship between ethereum gas and ZIL gas
(see [Gas](../protocol/protocol-gas.md)], EVM would be dramatically more expensive
than Scilla; in order to ameliorate this, we scale ethereum gas when
charging and for technical reasons, the scaled value (ie. the
scilla-equivalent gas price is what we track under the hood).

As a result, rounding occurs, and the amount of ethereum gas you are
charged for a transaction will be slightly less than the amount
estimated, in order that the `effectiveGasPrice * gasUsed` is equal to
the deduction made for gas.
