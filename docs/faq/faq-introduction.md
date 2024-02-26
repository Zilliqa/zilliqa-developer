---
id: faq-introduction
title: Frequently Asked Questions
keywords:
  - FAQ
  - Questions
  - EVM
description: Frequently asked questions
---

## Frequently asked questions

<!-- markdownlint-disable MD001 -->

#### What is the relationship between EVM ZIL and Zilliqa ZIL?

They are the same, though EVM ZIL are scaled to accomodate the differing precisions of EVM and Zilliqa ZIL (18 vs 12 decimal places).

#### Does this mean that transfers will produce dust?

No. The internal representation of value is in ZIL, not EVM units, but dust may not transfer as you were expecting it to.

#### Can I use the same address for EVM and ZIL API (eg. for Metamask and ZilPay/Torch)?

In practice, no. The way your address is chosen is that:

1.  You pick a private key
2.  You derive a public key from that.
3.  Derive an address from the public key, usually by hashing.

Zilliqa and Ethereum have different ways of performing the second
step, so the Zilliqa address derived from a given private key is
different from the Ethereum address for that key.

If you were to want a single address for both EVM and ZIL, you would
need to know eg. the ethereum private key for the ZIL address you had
just worked out; this would involve deriving the private key from a
(public) ZIL address, which can't practically be done, and so you
can't have the same address for EVM and ZIL APIs.

#### Can you work around this?

If there is enough interest, it might be possible to work around this
by allowing EVM transactions to have Schnorr signatures (so that you
could submit them via the ZIL API) and vice versa, but this would need
explicit support in DApps, and we judged the extra complexity surface
probably wasn't worth it for now.

#### Will I be able to restore a Zilliqa account in Metamask using my private key or seed phrase?

No. You can only restore Zilliqa accounts in a Zilliqa wallet (eg. ZilPay or Torch) or EVM accounts in an EVM wallet (like Metamask).

#### Will I be able to store ZRC fungible tokens in Metamask?

Not directly; you will be able to store them via our ERC-20-to-ZRC-2 gateway contract, which will let you see your ZRC-2 tokens as though they were ERC-20 tokens.

The source code will be available in [zilliqa-developer](https://github.com/zilliqa/zilliqa-developer) shortly after launch and we will publish a directory of deployed contracts.

#### Will I be able to store ZRC non-fungible tokens in Metamask?

Not initially; we hope to be able to provide this in a future release, though you could also write a gateway contract (similar to our ZRC-2/ERC-20 gateway) yourself.

#### Will I be able to use my Zil on Dex like Uniwap, Sushiswap to trade?

If and when those DEXes deploy to Zilliqa, yes.

#### Will Devs be able to deploy Uniswap/Sushiswap/1inch on Zilliqa?

Developers should be able to deploy your contracts to EVM on Zilliqa just like any other EVM-compatible chain.

#### Will I be able to sell my NFTs on NFT marketplace like Opensea/Blur?

Not until they add support for the Zilliqa chain (and even then, you will either need an ERC-721 gateway, or to have EVM NFTs on Zilliqa)

#### Will I be able to buy a NFT using Zil on Opensea/Blur?

Not until they add support for Zilliqa.

#### Will I be able to use stake.zilliqa.com with Metamask & stake my Zil?

Not directly; please let us know if this is functionality you'd like (or you can write a gateway contract yourself, of course). In the meantime, you'll need to transfer your ZIL to a Zilliqa wallet and stake them from there.

#### Will I be able to connect Metamask with Zilswap & buy tokens listed on Zilswap with native Zil?

Not until Zilswap supports EVM wallets.

#### Which Dex can I use to connect with Metamask & use my Native Zil to trade?

This is a new release, so there aren't currently any EVM dexes that we know of. We'll update this answer when some have tested and deployed.

#### Will NFTs created under ZRC1 have EVM interoperability or this apply only to ZRC6?

This is not yet decided; we'd hope to support both standards. Please
get in touch if you have a particular need for ZRC1 support (or,
again, it should be possible to write this yourself using our interop
facilities if you really need it).

#### What happens if I send ZIL via Torch or ZilPay to my EVM address, or Metamask to my ZIL address?

These transfers should execute normally, and your ZIL will arrive safely in the "other" wallet.

#### What happens if I send ZIL to a random address via Metamask?

They will be lost, just as they are today - in fact, they'll turn up just fine at the address you sent them to, but since no-one has the private key for that address, it won't be possible to do anything with them once they get there.

#### How about ZRC-2, ERC-20 and other contract-wrapped tokens?

This is trickier. Suppose you send some ZRC-2 tokens (such as `XCAD` or `ZWAP`) to your EVM address. They'll arrive just fine, but you will now want to send them elsewhere.

In order to do so, you will need to call the ZRC-2 contract with `_sender` equal to your EVM address. But, in order to make that call you need to submit a Zilliqa API transaction from your EVM address, which we've just agreed you can't do. So your funds will be stuck.

This is not optimal, and you can get out of it using [interwork](../zilevm/protocol/protocol-interwork.md) ; create a solidity contract which calls the Scilla contract using the `call scilla contract with _sender unchanged` precompile. You can now send a Scilla call from an EVM transaction, and there is a contract available in the
[zilliqa-developer](https://github.com/zilliqa/zilliqa-developer) repository which does this by building an ERC-20 facade for ZRC-2 assets; we'll deploy this against the common ZRC-2s and publish a list once EVM is live on mainnet.

This will recover your funds, but might be quite tricky to operate for arbitrary contracts; our roadmap contains a more generic mechanism for arbitrary contracts (though you will still need to know what transition/calldata you need to call).

If you didn't understand the above, please contact your the dApp maintainer, or your developers, who will hopefully be able to help you.

#### What about NFTs?

Please don't transfer your Zilliqa NFTs to EVM addresses for now! They are (probably) rescuable using the interwork protocol right now, and we will address this in our next release, but it's best not to test that.

#### How about transferring ERC-20s and other tokens to ZIL API address?

Please don't do that either! Rescuing trapped tokens in EVM contracts is significantly harder than for Scilla contracts, because Scilla contracts have self-describing storage encoding and source is always available.

Whilst the interop mechanism can be used to transfer these back to EVM addresses, it is substantially harder to write the code to do so, and probably impossible unless you have the source code (or at least the interface) of the contract in question. Again, the maintainers of the dApp may be able to help, or if you are sophisticated, you may be able to do this yourself.

We will try to provide assistance with the most common cases as they arise, but Zilliqa doesn't have the resources to support every use case.

#### How do I get started with development using hardhat?

See our [handy guide](../developers/guides/developing-with-hardhat.md) .
