---
tags:
  - metatransaction
  - reference
  - contract
  - zrc-3
---

# Metatransactions

Meta Transactions are a broad concept borrowed from Ethereum where it is typically used to load authenticated data onto a smart contract. This pattern is needed for many Ethereum projects including the ‘gasless’ relay system whereby one or more nodes accept transactions signed with a hidden private key and then wrap that transaction in a Ethereum transaction.

[ZRC-3](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-3.md) defines the technical specification that another contract with the ability to perform transitions with metatransactions should consume.

## How does it work?

It’s equivalent to adding a checkbook to your bank account (contract). Through the metatransaction which is defined by all the variables to show proof that the owner of the funds did authorize this exact spend in the form of a cryptographic signature of the entire message hash.

Metatransactions provide an alternative flow for users just getting started with their cryptocurrency and wallets. It allows for gasless transactions, cutting out the barrier of getting ZIL from an exchange or via mining before a Dapp user can interact with the smart contract.

It can also be seen as a way to defer the processing of transactions or even to guard a transition that requires multiparty authorization, although those applications are left to implementations and future standards.

[The ZRC-3 contract](https://github.com/Zilliqa/ZRC/blob/main/reference-contracts/MetaFungibleToken.scilla) adds a metatransfer transition to the existing ZRC-2 interface.

## Docker interaction

[relay.js](https://github.com/starling-foundries/relay.js) is a repository containing the cheque relay to forward metatransactions to Zilliqa network (sometimes called a bouncer proxy).

This contract is based largely on the recommendations of EIP-965. It accomplishes similar goals, but also adds a functionality for gasless metatransactions. In this implementation the cheque is a message containing these parameters:

```JS
{
         "from" : "0xa0000000...",
           "to" : "0xb0000000...",
       "amount" : 100,
     "contract" : "0x12395345...",
          "fee" : 2,
        "nonce" : 12,
    "signature" : "0xasdlj3io2j...",
    "pubkey"    : "asdfg123..."
}
```

Note that the recipient, amount, contract, fee and nonce values are hashed and signed client-side before submitting the metatransaction to an arbitrarily relay which can use this metatransaction as a spend-by-proxy. Both the relay and sender can be confident that parameters are properly validated on-chain, only a valid metatransaction can be spent and it can only be spent once.

This spend function does not replace the normal Transfer transition or the OperatorSend functionality already possible with ZRC2, instead it offers a third method for authorizing these transfers. One in which the holder of a token with spending authority is not presumed to have the Zil onhand to pay for transactions. It also optionally allows for paying transaction fees in tokens.

Possession of a metatransaction authorizes a relayer to submit this cheque on their behalf. This does not guarantee the funds, as any other transactions spending funds will be ordered like any other within the blockchain. This is by design as it removes the main way relays can abuse power - by censoring transactions.

If a relayer does not submit the cheque in a timely manner, the token holder has the opportunity to send the same metatransaction to the voidcheque transition to ensure the stalled transaction is not processed later.

### Relayer responsibilities

The relayer provides a public endpoint for receiving metatransactions, chequeing it's target contract and can validate any parameters to avoid wasting its own gas with a transaction that might be refused due to balance or previous spending of the metatransaction.

It may optionally report a minimum fee, but it cannot charge a fee that the token owner did not approve in the signed metatransaction. The anticipated malevolent behaviour - a relayer censoring transactions - is mitigated by the possibility for multiple relayers.

## Further reading

[Medium - ZRC-3 Grand Rewrite](https://medium.com/builders-of-zilliqa/zrc3s-grand-rewrite-22558797ea0)

[ZRC-3 Specification](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-3.md)

[ZRC-3 - Contract](https://github.com/Zilliqa/ZRC/blob/main/reference-contracts/MetaFungibleToken.scilla)

[ZRC3 - Interaction](https://github.com/Zilliqa/ZRC/tree/master/example/zrc3)

[relay.js "bouncer proxy" - Docker Interaction](https://github.com/starling-foundries/relay.js)
