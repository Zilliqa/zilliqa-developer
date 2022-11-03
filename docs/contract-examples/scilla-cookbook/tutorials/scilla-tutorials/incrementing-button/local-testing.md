---
sidebar_position: 9
---

# Testing the button

This section discusses deploying, interacting and testing the IncrementingButton contract and seeing if it meets the objectives set out in the introduction. This section discusses some of the sights developers will expect to encounter when interacting with contracts.

## IDE

The [Neo-Savant IDE](https://ide.zilliqa.com/#/) can be used for writing, testing and deploying Scilla contracts to different environments. We will be deploying this contract out to the simulated environment and we will try and press the button.

We create a new contract on the IDE using the "add contract" symbol at the top left next to files, which opens a new untitled Scilla document. We can paste the contract defined so far. If we check the contract, it will be checked for syntax errors.

One warning is returned from our contract. `No transition in contract IncrementingButton contains an accept statement` This warning means that our contract does not have the `accept` keyword and currently will not accept zil. We expected our contract not to accept zil so this warning is acceptable.

![Docusaurus](/img/tutorials/incrementingbutton/IDE-accept.png)

At the top right, we can connect to a simulated environment where we have several accounts and test funds available. We can then select a random account that has been generated in our simulated environment session, on a successful account change the amount of zil should change to an amount.

![Docusaurus](/img/tutorials/incrementingbutton/IDE-deployparameters.png)

Pressing the deploy button we can open the contract constructors and see it's immutable parameters which need defining. `IncrementingButton` has one immutable parameter defined earlier called `contract_owner`. We need to provide this before we can deploy the contract.

`amount` is any ZIL that needs to be accepted by the contract, like a sale or purchase. `Gas Price` is the amount paid for a unit of Gas - know we need to pay some funds to have our transaction processed by the network, in our case we are deploying a contract to the network, this amount is in `QA` which is a small decimal unit of `ZIL`

### Environments

Smart contracts can be deployed to the following environments

- mainnet
- testnet
- simulated-env
- localhost

For our example we will use the simulated environment because it's the fastest for development without setting up local infrastructure.

### Deploy to simulated-environment

:::tip
Simulated environment creates burner wallets when used.

Request some funds from the [facuet](https://dev-wallet.zilliqa.com/faucet). Press the funds icon in the topnav to refresh your wallets holding of test Zil.
:::

Now our wallet is funded, we can deploy our contract out. Press the user icon in the topnav to copy your wallets current address. Copy your current address into the button contract and press "Deploy Contract".

:::tip
If your receipt is green(success=true) the contract is now deployed on the simulated network for everyone to interact with.
:::

View this transaction on the blockchain by clicking the transaction id. [Here's one I made earlier.](https://devex.zilliqa.com/tx/beb48267d1c61a72abe07eb0fafc4b1de635dee14d451bb2dbdc1e9c646f8115?network=https://zilliqa-isolated-server.zilliqa.com/) Notice for a contract deploy, the immutable parameters and contract code have been written.

In the transaction, if you click "to" with a paper icon - this is the contract address, where it lives on the network, and the "from" address is you, the user calling. If you examine the contract on the network, it looks different to a typical user wallet because it has extra state wrapped to it. `init parameters` are our immutable parameters we defined when calling, the `state` is our mutable parameters, and the `code` is the contract definition and logic written publicly.

![Docusaurus](/img/tutorials/incrementingbutton/explorer-state.png)

Our state reflects our initial starting value. `_balance` is an implicit field and holds the amount of ZIL held by the contract. Our two state fields have been initialised correctly. The current clicker is represented by the contract owners address initially, and our counter starts at zero.

Let's try and call the button with the `contract_owners` address. This should error since the contract has the deployer address already in the state. It will inspect the \_sender and match that correctly with the last button presser, meaning they are the last presser.

![Docusaurus](/img/tutorials/incrementingbutton/IDE-error.png)

In the error example above, it can be seen the -2 error is being thrown from the transition "PressTheButton".

```ocaml
...
let make_error =
  fun (result : Error) =>
    let result_code =
      match result with
      | NotContractOwner             => Int32 -1
      | NotUniqueClicker             => Int32 -2
      end
    in
    { _exception : "Error"; code : result_code }
...
```

So if we switch wallets to an address who isn't the contract owner, and attempt to call "PressTheButton" this will succeed.

![Docusaurus](/img/tutorials/incrementingbutton/IDE-success.png)

And the state of our button contract changes, the current_clicker mutable state changes, and the counter is incremented by one.

![Docusaurus](/img/tutorials/incrementingbutton/explorer-state-1.png)

[Events](https://devex.zilliqa.com/tx/a013d2add26b30106d0b035bb05b9ab1591904435c55bd19f5397f306e91c1ea?network=https://zilliqa-isolated-server.zilliqa.com/)

Similar errors or success would occur if we also pressed reset the button. Calling the AdminReset transition as the contract_owner, would successfully reset the counter to zero, whereas sending on any other address would result in an error.
