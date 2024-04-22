---
id: chainlink
title: Zilliqa Chainlink Oracles
keywords:
  - zilliqa
  - development
  - oracles
  - chainlink
  - solidity
description: Learn how to use Chainlink oracles on Zilliqa.
---

---

## How to use Chainlink Community Deployment price feeds with smart contracts on Zilliqa

In this tutorial we will show you how to get the current ZIL/ USDT price from one of the price feed oracles operating as part of the [Chainlink Community Deployment](https://docs.chain.link/data-feeds/selecting-data-feeds/#chainlink-community-deployments) on the Zilliqa network.

> **Note**: This tutorial is aimed towards developers as you will need to have some basic knowledge of smart contracts and programming to complete the steps below.

This guide will explain how to accomplish the following:

1. Configure Metamask to the Zilliqa testnet
2. Deploy a smart contract to the testnet.
3. Have our new smart contract get the current price of ZIL from the oracle contract

> **Note**: Chainlink Community Deployment price feeds are not official integrations. Developers should understand risks associated with community deployments. [Learn More](https://docs.chain.link/data-feeds/selecting-data-feeds/#chainlink-community-deployments)

## Connecting to Zilliqa Testnet

First we need to add the Zilliqa testnet to our metamask:

**Testnet network information:**

- **Network Name**: Zilliqa Testnet
- **Chain ID**: 33101
- **RPC URL**: [https://dev-api.zilliqa.com](https://dev-api.zilliqa.com)
- **Currency Symbol**: ZIL

Using the information above, follow this guide: [How to add a custom network to Metamask](https://support.metamask.io/hc/en-us/articles/360043227612-How-to-add-a-custom-network-RPC).

## Deploy Smart Contract via Remix

Remix is great as it is a zero-setup tool for working with smart contracts, allowing us to easily demonstrate the oracle contracts. But first - let’s inspect the contracts we’ll be working with!

## FeedConsumer

This is the contract you will deploy. This is just a minimal example to demonstrate how to consume these data feeds on Zilliqa. You can extend this to be a decentralised borrowing and lending protocol, a DEX, game, NFT minting contract - anything you can think of!

If you look at the last function, it only returns the current answer, or price. You can see that it also is capable of returning other pieces of information such as timestamp data. Chainlink documentation encourages dAapp developers to verify the timestamp together with the price information to protect against stale data. While the oracles should be publishing fresh data every other minute, there can be unexpected downtime.

This is a public view function, meaning that getting the price data has zero cost. You can add pricing data to your dApp with no extra charge to your team or your users.

The constructor takes in an address as an argument.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PriceFeedInterface} from "./PriceFeedInterface.sol";

contract FeedConsumer {
  PriceFeedInterface public immutable dataFeed;

  constructor(address feedAddress) {
    dataFeed = PriceFeedInterface(feedAddress);
  }

  function decimals() external view returns (uint8) {
    return dataFeed.decimals();
  }

  function description() external view returns (string memory) {
    return dataFeed.description();
  }

  function getLatestAnswer() public view returns (int) {
    // prettier-ignore
    (
    /* uint80 roundID */,
    int answer,
    /*uint startedAt*/,
    /*uint timeStamp*/,
    /*uint80 answeredInRound*/
    ) = dataFeed.latestRoundData();
    return answer;
  }
}
```

Below is the interface of a contract that has already been deployed by the Zilliqa team. The oracle nodes publish data directly to the PriceFeed contract every other minute. There is one contract per asset.

## Contract Addresses

- **ZIL / USD (Testnet)**: `0x845f152725A2FF60cb2F5a0E024B5A749f7551C0`
- **USDT / USD (Testnet)**: `0xcb893BC5741672Ffc7A7992876253BE83f2c550D`
- **ZIL / USD (Mainnet)**: `0x8245E42c7eBF756E7A18A59Fd828159f29587a23`
- **USDT / USD (Mainnet)**: `0x54d10Ee86cd2C3258b23FDb78782F70e84966683`

## PriceFeedInterface

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

interface PriceFeedInterface {
  function latestRoundData() external view returns (
    uint80 roundId,
    int256 answer,
    uint256 startedAt,
    uint256 updatedAt,
    uint80 answeredInRound
  );

  function decimals() external view returns (uint8);

  function description() external view returns (string memory);
}

```

## Deploying FeedConsumer

Now that we have inspected the smart contracts, let’s deploy FeedConsumer.

Go to [Remix IDE](https://remix.ethereum.org/). In the File Explorer tab, add both of the contracts under the contracts folder like this:

<img alt="Remix File Explorer" width="1600" src="../../../../assets/img/chainlink/remix-file-explorer.png" />

The next thing to do is to compile our contracts with the built-in compiler. Do this for both contracts:

<img alt="Remix File Explorer" width="800" src="../../../../assets/img/chainlink/remix-compiler.png" />

Now we are ready to deploy the contract! Go to the Remix Deployer. Make sure you have selected Zilliqa Testnet in Metamask from the earlier step.

There are three actions we need to take here:

1. For “Environment”, select “Injected Provider”. If you are using Metamask, you should get a popup asking if you want to allow metamask to connect to Remix. Accept this. If successful, you should see “Custom (33101) network” right under the dropdown list.
2. Copy the EVM address from above that belongs to “ZIL / USD (Testnet)” and paste it in the input field that says “address feedAddress”
3. Click deploy and accept the transaction in your Metamask wallet. Normally it takes around one minute for deployment to complete.

<img alt="Remix File Explorer" width="1600" src="../../../../assets/img/chainlink/remix-transactions-0.png" />

If all went well you should now see your FeedConsumer contract appear under “Deployed Contracts”:
<img alt="Remix File Explorer" width="800" src="../../../../assets/img/chainlink/remix-transactions-1.png" />

If so, great! Let’s open this up and call a few of its functions:

Let’s call the description function: after a couple of seconds you should see “ZIL / USD”. Do the same with decimals and getLatestAnswer() and you should see a similar result as below:
<img alt="Remix File Explorer" width="800" src="../../../../assets/img/chainlink/remix-deployed-contracts.png" />

Remember since this is on-chain data, there are no floats / decimal numbers. So to get the nominal value in a more human readable format, you divide the answer with the decimal points as follows:

`1791000 / 1e8 = 0.01791 USD for 1 ZIL.`

There you have it! While this a simple demonstration, it should illuminate the wide range of possibilities that can be materialised on Zilliqa now that price feed oracles are active on the network.

## Resources

- [Chainlink Data Feeds](https://docs.chain.link/data-feeds)
- [Using Data Feeds](https://docs.chain.link/data-feeds/using-data-feeds)
- [Chainlink Whitepaper](https://chain.link/whitepaper)
- [Remix](https://remix.ethereum.org/)
- [Zilliqa public RPCs](https://chainlist.org/?search=zilliqa&testnets=true)
- [DefiLlama](https://defillama.com/)
- [Chainlink Community Deployments](https://docs.chain.link/data-feeds/selecting-data-feeds/#chainlink-community-deployments)
