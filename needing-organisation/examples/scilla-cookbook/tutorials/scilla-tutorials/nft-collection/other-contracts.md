---
sidebar_position: 6
---

# Interactivity with other contracts

The ZRC-6 NFT contract is scoped to only do so much, other functionality and interactivity can be easily achieved with other Scilla contracts. As discussed previously, it's important to not pollute the NFT contract with project specific information and have other contracts which can interact with the NFT to then include your project specific information.

## Proxy contracts

[Proxy contracts](../docs/recipes/scilla-recipes/pattern_proxy.md) are an easy way to delegate primary sales through. Firstly create a proxy contract which takes an exact payment of ZIL or tokens. Bestow the proxy with the minting ability and then ask it to call the `Mint` transition. To the public, the `Mint` transition will error because they are not minters, but the call can be routed through the proxy for the NFT contract to then send the token to the user calling of the proxy.

[Beanterra Proxy](https://viewblock.io/zilliqa/tx/0xadb58296ede89e5386239c8e6a5175d64dedf038a1336c9f42e5f1d0316e4765)

[Duck Proxy](https://viewblock.io/zilliqa/address/zil1vgz4y34ykxc9arjh0ezkacqv3xxtywlcvv0hx0)

## Migration contracts

A migration contract is typically used to upgrade an existing NFT collection. Developers may think of many different ways to perform this action. The easiest method to preserve `token_id` association with a user address is to directly Mint all of the tokens to a migration contract. When interacted with, burns a token_id from the old ZRC-1 or ZRC-6 contract, if this operation is successful, then the contract then moves the same token_id from the new nft contract to the user calling the migrator.

[Duck Migrator](https://viewblock.io/zilliqa/address/zil1m2hhu9reau5t57qckj9w2ejttxmn3hyhy77hr4)

## Exclusive contracts

An exclusive contract is a contract which only allows operations to be performed by particular users. The list of addresses that are allowed to call the function can easily be transposed by remote reading the NFT contract balances map at the position \_sender to check if they hold 1 or more NFTs.

[Duck Ownership](https://viewblock.io/zilliqa/address/zil1xhh2qwaca8w8u8ezcykg5yq8a2tguk7c298wjq)
