---
sidebar_position: 2
---

# Deploying a ZRC-6

Taking the example for [HelloWorld Scilla contract deployment](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/deployContract.js) we can extend this to deploy an NFT contract.

As we stated previously, to first do any NFT operations. We firstly need to create a contract to house the data association.

## Amendments made

Firstly, review the sample Javascript for [deploying a contract](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/deployContract.js).
The first change was to copy and paste the [ZRC-6 contract code](https://github.com/Zilliqa/ZRC/blob/main/zrcs/zrc-6.md) into this example program (highlighted).

Next, we need to change the immutable parameters being passed, this is on L106 of the [sample code for deploying a contract](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/deployContract.js#L106). As usual we always pass ```_scilla_version``` first, followed by our defined immutable parameters in the order they are expected.

## Example deployment code

Change ```YOUR_BECH32_ADDRESS_HERE``` to your bech32 address. Get some testnet zil funded to a private key you control and run the [ZRC-6 deployment code.](https://pastebin.com/raw/GLaSDWpx)
