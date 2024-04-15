---
id: changes
title: Changelog and transition plan
---

# Zilliqa 2.0 changes and transition plan

## Changes

There are a number of differences between Zilliqa 1.0 and Zilliqa 2.0 that you should be aware of:

- Zilliqa 2.0 uses proof of stake based Fast-Hotstuff as a consensus algorithm. Mining is no longer necessary.
- Zilliqa 2.0 has many fewer nodes, and is thus far cheaper to run, than Zilliqa 1.0 - a typical Zilliqa 2.0 mainnet can run comfortably in 32 nodes.
- Zilliqa 2.0 has a much faster block time (there is typically a hardwired minimum of 1s/block); dApp operators will need to make sure that where they use block number as a proxy for a timestamp, they allow sufficient blocks for users to react.
- Zilliqa 2 upgrades are seamless and relatively quick; you don't need to redownload persistence and there is an auto-upgrader you can run if you wish which will run the newer version of zq2 and cut over when ready. We hope this will enable us to eliminate upgrade downtime and to make more frequent bug fixes.

## Continuity

There are also a number of things that have not changed:

- Zilliqa 2.0 is (or should be!) compatible with all the same dApps, tokens and sites as Zilliqa 1.

## Transition plan

Zilliqa 2.0 will being by running a prototype devnet. This is an empty developer test network that you can use to try out your code.

There will then be:

- A `prototestnet` network - this periodically (once every few days) imports existing Zilliqa 1 testnet persistence and starts a network on it; this allows you to test against existing testnet persistence.
- A `protomainnet` network - which does the same with mainnet persistence.
- Existing SSNs will be invited to become validators on the Zilliqa 2 network (bringing their delegated stake with them). We'll contact you individually about this.
- We will then cut `testnet` over to Zilliqa 2
- Then `mainnet`

Note that the `proto` networks will have different chain IDs to the networks they import their state from; this is necessary to avoid replay attacks, but means that you will find the chain ids of old transactions are not the same as the chain ids for new ones.
