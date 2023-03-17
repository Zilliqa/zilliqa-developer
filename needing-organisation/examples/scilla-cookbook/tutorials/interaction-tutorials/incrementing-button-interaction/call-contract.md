---
sidebar_position: 3
---

# Calling a Contracts Transition

Review the Javascript that defines [callContract.js](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/callContract.js)

## Calling a Transition

In the below snippet, we begin to define an async function that will call our public transition for pushing the button. We now define a call to the previously defined contract transition with any associated vnames parameters. The below example shows a call to the `HelloWorld` contract's transition `setHello`.

Our example transition name is `PressTheButton` that takes no additional vnames so we can omit the collection of vnames.

```js {11,12,13}
// Calls the previously deployed contract transition
async function PressTheButton(bech_32_bystr) {
  try {
    const balance = await zilliqa.blockchain.getBalance(address_from_pk);
    const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();
    const myGasPrice = units.toQa("2000", units.Units.Li);
    const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result));
    const deployedContract = zilliqa.contracts.at(bech_32_bystr);

    console.log(`Pressing the button...`);
    const callTx = await deployedContract.callWithoutConfirm(
      "PressTheButton", // transition name
      [], // no params required
      {
        version: VERSION,
        amount: new BN(0),
        gasPrice: myGasPrice,
        gasLimit: Long.fromNumber(8000),
      },
      false
    );

    console.log(`Waiting transaction be confirmed`);
    const confirmedTxn = await callTx.confirm(callTx.id);

    //console.log(`The transaction status is:`);
    //console.log(confirmedTxn.receipt);
    if (confirmedTxn.receipt.success === true) {
      console.log(`Button pressed by : ${address_from_pk}`);
    }
  } catch (err) {
    console.log(err);
  }
}
```

We then add this call method to our start point to then call the transition from the newly deployed contract. Using `Promise.all` we can ensure that `PressTheButton` processes after `DeployButtonContract` so we have the `deployed_contract_base_16` variable defined.

```js
// Application Definition
// DEPLOY a contract,
// SEND a transaction
// LISTEN to events emitted
// MONITOR mutable state
async function DeploySendListenMonitor() {
  deployed_contract_base_16 = await DeployButtonContract();
  bech_32_bystr = toBech32Address(deployed_contract_base_16);
  console.log(`got ${bech_32_bystr} from ${deployed_contract_base_16}`);

  await Promise.all([PressTheButton(bech_32_bystr)]);
}
```

The console, after waiting for the async call to be returned should reply with. If we examined the contract state, our counter would have increased since we had a successful transaction with no errors thrown.

```js
...
Pressing the button...
Waiting transaction be confirmed
Button pressed by : 0x428A2aA43456FE7fd2De66E48C1fBf372eC10eAE
```
