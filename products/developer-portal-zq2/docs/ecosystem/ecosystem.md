---
id: ecosystem
title: Zilliqa ecosystem
---

# Zilliqa ecosystem

This page collects information about parts of the Zilliqa ecosystem useful for developers.

You should look to the [main Zilliqa website](https://zilliqa.com/) to provide information on the ecosystem more widely. Our [coingecko](https://www.coingecko.com/en/coins/zilliqa) page has general information about our token, and developer tools and SDKs can be found on the [sdk](/sdk/) page.

Unless specified otherwise, all addresses given here are Zilliqa mainnet addresses. Note that unless specified otherwise, Zilliqa _DOES NOT MAKE ANY REPRESENTATIONS ABOUT THESE SITES OR CONTRACTS_. You use them entirely at your own risk and you are encouraged to do your own research to satisfy yourself that they meet your needs and risk profile.

Entries below marked with a `[Z]` are operated by Zilliqa.

## On-chain facilities

- We have some [price oracles](./chainlink.md).

## Useful contract addresses

- Tokens: [ZilStream](https://zilstream.com/) can direct you to native token addresses.

- Oracles
  **ZIL / USD (Testnet)**: `0x845f152725A2FF60cb2F5a0E024B5A749f7551C0`
  **USDT / USD (Testnet)**: `0xcb893BC5741672Ffc7A7992876253BE83f2c550D`
  **ZIL / USD (Mainnet)**: `0x8245E42c7eBF756E7A18A59Fd828159f29587a23`
  **USDT / USD (Mainnet)**: `0x54d10Ee86cd2C3258b23FDb78782F70e84966683`

- The reward contract - this tells you what the current staking reward amounts are.

  - Source: [https://github.com/Zilliqa/zilliqa-developer/tree/main/contracts/reward_control](https://github.com/Zilliqa/zilliqa-developer/tree/main/contracts/reward_control)
  - **Mainnet**: `zil1hnjwuvjnjasxy230u2muceas3x4prd36d9dkev`

- wZIL (a wrapped ZIL)
  - [Instructions](wzil.md)
  - **Mainnet**: [`zil1gvr0jgwfsfmxsyx0xsnhtlte4gks6r3yk8x5fn`](https://viewblock.io/zilliqa/address/zil1gvr0jgwfsfmxsyx0xsnhtlte4gks6r3yk8x5fn)
  - **Testnet**: [`zil1nzn3k336xwal7egdzgalqnclxtgu3dggxed85m`](https://viewblock.io/zilliqa/address/zil1nzn3k336xwal7egdzgalqnclxtgu3dggxed85m?network=testnet)

## Useful websites

- [ZilStream](https://zilstream.com/) provides Zilliqa token prices.
- [Viewblock](https://viewblock.io/zilliqa) provides a block explorer for Zilliqa main and testnet
- [ZilSwap](https://zilswap.io/swap) - provides a native Zilliqa DEX and ZilBridge, for bridging assets between chains.
  - [ZilSwap developer info](https://docs.zilswap.org/#/smart-contract)
- [Xbridge](https://xbridge.zilliqa.com) - a token bridge based on our general contract bridge (see [github](https://github.com/Zilliqa/zilliqa-developer/tree/main/products/bridge))
- [Plunderswap](https://plunderswap.com/) - provides an EVM DEX for Zilliqa
  - [transfer from Zil to EVM wallet or back](https://plunderswap.com/transfer)
- [Avely](https://avely.fi/) provides a liquid staking token for Zilliqa.
  - [Contracts](https://github.com/avely-finance)
- `[Z]` [Zillion](https://stake.zilliqa.com) is a Zilliqa staking viewer.

## Wallets

- [Magic](https://docs.magic.link/) is a developer SDK that allows you to enable passwordless authentication using magic links.
- [ZilPay](https://zilpay.io/) is a Zilliqa native wallet.

## CEXs

See our [coingecko](https://www.coingecko.com/en/coins/zilliqa) page for a list of Zilliqa markets.
