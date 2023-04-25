---
id: zilevm-introduction
title: ZILEVM Introduction
keywords:
  - EVM
description: Zilliqa EVM
---

---

_Please bear with us as we populate these pages; if you have feedback/requests
for documentation, please see the #ethereum-virtual-machine channel on
[discord](https://discord.gg/nKznfCaZxy)_

## Getting Started with Zilliqa EVM

The Zilliqa EVM implementation is written as an execution engine and state storage on top of the Zilliqa consensus protocol.

The token for EVM operations is ZIL - there is no separate token for EVM
APIs.

However, because Zilliqa and EVM APIs have different ways of
deriving an address from a private key, you will have _different addresses for EVM and Zilliqa APIs_.

This means that you need to be careful when sending contract-bound
tokens between them, because if you send ERC-20 tokens to your Zilliqa
API address, or ZRC-2 tokens to your EVM adress, you will not be able
to create contract calls from the "other" address to recover them. To
read more about topics like this, see the
[FAQ](../../faq/faq-introduction.md).

### Resources

- [Configuring Metamask for Zilliqa](../onboard/onboard-metamask.md)
- [Developing Solidity contracts for Zilliqa](../developer-onboarding/dev-onboarding-introduction.md)
- [API endpoints](../../api/introduction/api-endpoints.md)
- [Developing with hardhat on Zilliqa](../../developers/guides/developing-with-hardhat.md)

#### EVM testnet faucet

You can use the
[testnet faucet](../../developers/developer-toolings/dev-tools-faucet.md) to add
ZIL to your EVM account; just paste your ethereum address (given by your wallet)
into the faucet destination address.
