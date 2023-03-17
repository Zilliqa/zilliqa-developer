---
sidebar_position: 1
---

# Introduction

We are going to use zilliqa-js to interact with our deployed "IncrementingButton" contract.

You don't need any previous experience with zilliqa-js to follow along with this tutorial, but experience with Javascript, Node.js and npm will be useful

## IncrementingButtonInteraction

IncrementingButtonInteraction is a walkthrough tutorial of the basic concepts of using zilliqa-js SDK to interact with deployed contracts with the following objectives.

- Use zilliqa-js to deploy a contract.
- Use zilliqa-js to interact with and press the button.
- Use zilliqa-js to get the mutable state of our contract.
- Use zilliqa-js to listen for events in our contract.

Effectively we are creating an automation layer where we can dynamically deploy new contracts and test the contract implementation will emit certain events and leave the mutable variables in a particular state. The whole file is defined in the summary.

## Setup of project

Open a new Visual Studio Code (VSC) window and create a new project.

Open a terminal window and initialise npm with `npm init` and either press enter to skip and insert default values, or type the values, a file called `package.json` is created which will store our dependencies.

Now npm is setup, we can install the latest zilliqa-js, in the terminal type `npm install @zilliqa-js/zilliqa --save`.

:::tip
This is the recommended way for frontend and backend Javascript frameworks to interact with the blockchain.
:::

Now we have all the dependencies required, we can start to write some Javascript. Create a new file called `index.js`.

The require statements are pulling in the logic from zilliqa-js to use later in our code.

For example the `util` class has methods dealing with BigNumbers and Zilliqa units. Similarly, the `crypto` class has methods dealing with converting an Base16 to Bech32 and resolving a Bech32 address from a private key string.

The notable mention of this snippet is the initialisation of a new Zilliqa Object with a network address as a parameter, the value is the network we want to connect to, in this example its `dev-api` aka testnet. To determine what network parameters you need for a specific environment, consult the [developer network documentation](https://dev.zilliqa.com/docs/apis/api-introduction).

```js
// Dependency Implementation
const { BN, Long, bytes, units } = require("@zilliqa-js/util");
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const { StatusType, MessageType } = require("@zilliqa-js/subscriptions");
const {
  fromBech32Address,
  toBech32Address,
  getAddressFromPrivateKey,
} = require("@zilliqa-js/crypto");
```

In the below snippet, we begin to create some of the configuration needed later to send a transaction. `VERSION` will be used later in the transaction call. We then add a private key to the Zilliqa object so later we can execute against a specific address we own.

For our example, we ensure chain id and msgVersion are correct for testnet

```js
// these values can change depending on what network you are connecting on
const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");
const chainId = 333;
const msgVersion = 1;
const VERSION = bytes.pack(chainId, msgVersion);

// use a dotenv file for storing private keys, NEVER commit this value directly to your repo
const privateKey = ""; //process.env.PRIVATE_KEY
```

:::danger
Never commit a private key to a repo, use a dotenv file instead, use .gitignore to ignore .env and create a .env.template with what the .env should contain.
:::
