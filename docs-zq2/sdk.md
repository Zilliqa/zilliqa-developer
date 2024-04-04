---
id: sdk
title: Zilliqa SDKs
---
# Zilliqa SDKs, repositories, and tools

Note that a list of useful on-chain facilities (contracts, etc.) can be found via the [ecosystem](ecosystem/ecosystem.md) pages.

## Zilliqa SDKs

We provide native SDKs for:

 * [Rust](https://crates.io/crates/zilliqa-rs)
 * [Golang](https://github.com/Zilliqa/gozilliqa-sdk)
 * [Java](https://github.com/Zilliqa/zilliqa-developer/tree/main/products/laksaj)
 * [Javascript/typescript](https://www.npmjs.com/package/@zilliqa-js/zilliqa)
 * [Python](https://github.com/zilliqa/pyzil)

## Scilla tools

We provide various tools and information about Scilla:

 * [Scilla documentation](https://scilla.readthedocs.org/latest/) and a [tutorial](https://learnscilla.com).
 * [Neo-Savant](https://ide.zilliqa.com/) - a GUI for writing Scilla
 * You can deploy and test Scilla contracts via hardhat using the [Scilla hardhat plugin](https://github.com/Zilliqa/hardhat-scilla-plugin)

## EVM libraries

We provide local forks of:

 * [ethers.js](https://github.com/Zilliqa/ethers.js) - because ethers has recently started strictly enforcing canonical signatures and, of course, the signatures on native Zilliqa transactions (being Schnorr signatures) cannot be canonical.
 * [otterscan](https://github.com/Zilliqa/otterscan) - to add the ability to understand Zilliqa native transactions.

## Sample contracts

 * [ZRC](https://github.com/Zilliqa/zrc) contains reference contracts and standards for Zilliqa native contracts.
 * [zilliqa-developer/contracts](https://github.com/Zilliqa/zilliqa-developer/tree/main/contracts) contains more extensive/complex contracts.

In particular, `zilliqa-developer` contains contracts for vesting and burning tokens, and for generating contracts that expose native Zilliqa functionality to EVM (eg. for generating an ERC-20 compatible token contract that manipulates an underlying ZRC-2 asset).

## Zilliqa source

We provide a number of repositories as open source:

 * [zq2](https://github.com/Zilliqa/zq2) - the Zilliqa 2 source code itself.
 * [zilliqa-developer](https://github.com/Zilliqa/zilliqa-developer) - applications supported by Zilliqa; this site, the Zillion site, many of our SDKs, etc.


