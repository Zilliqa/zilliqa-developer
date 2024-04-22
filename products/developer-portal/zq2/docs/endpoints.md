---
id: endpoints
title: API endpoints
hide:
  - toc
keywords:
  - api
  - introduction
  - endpoints
description: Zilliqa API Endpoints
---

# Endpoints

Chain ids can be found at [chainlist](https://chainlist.org/?search=zilliqa&testnets=true).

Uptime can be found at [status.zilliqa.com](https://status.zilliqa.com) and you can get API examples from the [api](/api) pages.

We have several blockchains running.

## devnet

Devnet is a general purpose developer network, synchronised to the latest (or nearly the latest) commit from [https://github.com/zilliqa/zq2](https://github.com/zilliqa/zq2) . Its state and uptime are not guaranteed, but it is useful for checking your contracts against the latest and greatest software versions.

<div class="center-table" markdown>
|  Name  | Version | ChainId | API | Otterscan | Faucet  | Connect | Notes |
| ------ | ------- | ------- | --- | --------- | ------  | ------- | ----- |
| devnet | <span id="devnet_vsn" class="zq2_vsn">api.zq2-devnet.zilliqa.com</span> | [33469](https://chainlist.org/chain/33469) | https://api.zq2-devnet.zilliqa.com | https://explorer.zq2-devnet.zilliqa.com | https://faucet.zq2-devnet.zilliqa.com/ | <a href="javascript:connectZilliqaChain('https://api.zq2-devnet.zilliqa.com', '0x82BD', 'Zilliqa 2 EVM Devnet', 'https://explorer.zq2-devnet.zilliqa.com', 'Zilliqa 2 testnet', 'ZIL')">Metamask</a> | <span class="zq2_docs_devnet_vsn" kind="api">&nbsp;</span> |
</div>
