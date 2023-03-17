---
sidebar_position: 1
---

# Introduction

We are going to utilise the [ZRC-6 non-fungible token Scilla contract](../../../recipes/scilla-recipes/nonfungible.md) to create and manage an art collection. We'll use the metadata definition described in [ZRC-7 NFT Metadata Standard](https://github.com/Zilliqa/ZRC/blob/main/zrcs/zrc-7.md) to describe our tokens with richer data than just an image.

You don't need any previous blockchain experience, but general Javascript and web hosting experience will be useful.

## NFT collection

NFT collection is a walkthrough tutorial of the steps required to setup an NFT project on Zilliqa.

- Deploy a ZRC-6 contract to the network
- Make the art/resource assets public
  - Asset storage options
- Create and make the metadata for the asset public
  - base_uri vs token_uri
- Mint a token to an address
- Other typical NFT contract interactions

Once we have setup our contract and initially tested the creation (minting) process, we can begin to move the calling of NFT contract logic to zilliqa-js to help automate this process or move this into a web interface.

:::tip
If you found this guide useful, please consider contributing your knowledge!
:::
