---
tags:
  - dex
  - decentralised
  - exchange
  - swap
---

# Decentralised exchange

Decentralized exchanges (DEX) typically do not use order books to facilitate trades or set prices. Instead, these platforms employ liquidity pool protocols to determine asset pricing.

Peer-to-peer in nature, these exchanges execute trades between users’ wallets instantly — a process some refer to as a swap. The DEX in this category are ranked in total value locked (TVL), or the value of assets held in the protocol's smart contracts.

## What is Zilswap DEX

Zilswap is the most popular DEX on Zilliqa with the most TVL.

The [Zilswap DEX Contract](https://viewblock.io/zilliqa/address/zil1gkwt95a67lnpe774lcmz72y6ay4jh2asmmjw6u?tab=code) is deployed at the mainnet address ```zil1gkwt95a67lnpe774lcmz72y6ay4jh2asmmjw6u```

## How does it work

Pairs (ZIL/Token) act as automated market makers, standing ready to accept one token for the other as long as the “constant product” formula is preserved.

This formula, most simply expressed as ```k = x * y```, states that trades must not change the product (k) of a pair’s reserve balances (x and y).

This formula particularly desirable feature where it can always provide liquidity, no matter how large the order size nor how tiny the liquidity pool. The formula increases the price of the coin as the desired swap quantity increases. While larger orders tend to suffer, the DEX never has to worry about running out of liquidity.

## What is Liquidity

Anyone can become a liquidity provider (LP) for a pool by depositing an equivalent value of each underlying token in return for having an entry in the pool state. The deposit is a 50%/50% weighting with a 1000 ZIL minimum (2000 ZIL total value). The state tracks the pro-rata shares of the total reserves, and can be redeemed for the underlying assets at any time.

## Zilswap DEX Contract

The Zilswap DEX has the following public transitions to facilitate in the swapping of tokens. It allows the swapping of ZIL or tokens with either an equal amount of one side of the trade and a dynamic calculated amount on the other. The DEX allows for adding and removing of liquidity. The DEX has some admin transitions for setting the fee and transferring the ownership.

These transitions are able to be called across contracts or interacted with through a Zilliqa SDK.

```ocaml
transition SetFee

transition TransferOwnership

transition AcceptPendingOwnership

transition AddLiquidity

transition RemoveLiquidity

transition SwapExactZILForTokens

transition SwapExactTokensForZIL

transition SwapZILForExactTokens

transition SwapTokensForExactZIL

transition SwapExactTokensForTokens

transition SwapTokensForExactTokens
```

## Further reading

[Scilla Documentation - Create a token exchange](https://scilla.readthedocs.io/en/latest/scilla-by-example.html?highlight=list#exchange-specification)
