---
id: architecture
title: Zilliqa EVM architecture
keywords:
  - Developers
  - Dapps
description: Zilliqa EVM architecture
---

---

## Zilliqa EVM architecture

Supporting EVM on Zilliqa involves:

- Building an execution engine based on the Ethereum Virtual Machine. This allows us to run EVM programs and, thus, compiled Solidity contracts.
- Supporting enough common precompiles to make common-case contracts work.
- Supporting a "sufficient" subset of the Ethereum API calls. This allows users to create EVM contracts, use common Ethereum tools like hardhat and truffle to write contracts, and interact with Zilliqa using Ethereum-standard software like Metamask.
- Arranging to execute EVM transactions, and store EVM-style state.

### Execution engine

Our execution engine is based on continuation passing and the [Sputnik VM](https://github.com/rust-blockchain/evm).

This allows us to call Scilla from EVM, via [precompiles](../protocol/protocol-precompiles.md), and EVM from Scilla via message passing.

###
