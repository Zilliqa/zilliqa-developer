---
sidebar_position: 3
---

# Batch minting ZRC-6

Taking the example for [HelloWorld Scilla transition interaction](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/helloWorld.js) we can extend this to call our NFT contract to perform a Mint or BatchMint operation.

As we stated previously, to first do any NFT operations. We firstly need to create a contract to house the data association.

## Amendments made

Instead of calling once, we take advantage of using the BatchMint operation whereby we can perform multiple mints in one transaction.

`multi_call` is a pair of ByStr20 String which is the relationship of user address to token_uri - it's the parameters required for a singular mint. If base_uri is being used, then token_uri should be empty string. An array is filled up N times with this data.

`mintMsg` is an object which is the accumulation of these `multi_call` of the type `List (Pair ByStr20 String)`. This type is required for the BatchMint operation.

`BatchMint` is a function which wraps the batch minting in a loop. The user specifies an amount of tokens to mint, and the function read's the current token_id present on the contract to determine how many more tokens should be minted, if any. We need to manually increase the nonce by once each time for the blockchain to understand the commands in order.

At this point it's a typical blockchain transaction. We post the data to the network, specifying a contract address, transition and parameter arguments. The chain will respond with a success or error response and that result is captured.

```js
const { BN, Long, bytes, units } = require("@zilliqa-js/util");
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const {
  toBech32Address,
  getAddressFromPrivateKey,
} = require("@zilliqa-js/crypto");

const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");

const chainId = 333; // chainId of the developer testnet
const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(chainId, msgVersion);

// Populate the wallet with an account
const privateKey = "";

zilliqa.wallet.addByPrivateKey(privateKey);

const zrc6contract = "18d1f737c1a1102cca966bf82dfe459e35fbd524";

async function IncrementingNonceBatchMint(nonce, n) {
  try {
    // Get Minimum Gas Price from blockchain
    const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();

    console.log(`Current Minimum Gas Price: ${minGasPrice.result}`);

    const myGasPrice = units.toQa("6100", units.Units.Li); // Gas Price that will be used by all transactions

    console.log(`My Gas Price ${myGasPrice.toString()}`);
    const isGasSufficient = myGasPrice.gte(new BN(minGasPrice.result)); // Checks if your gas price is less than the minimum gas price
    console.log(`Is the gas price sufficient? ${isGasSufficient}`);

    const deployedContract = zilliqa.contracts.at(zrc6contract);

    const user_address = "0x0000000000000000000000000000000000000000";

    const multi_call = {
      constructor: "Pair",
      argtypes: ["ByStr20", "String"],
      arguments: [user_address, ""],
    };
    const arrayOfPairs = Array(n).fill(multi_call);

    const mintMsg = {
      vname: "to_token_uri_pair_list",
      type: "List (Pair ByStr20 String)",
      value: arrayOfPairs,
    };

    const callTx = await deployedContract.callWithoutConfirm(
      "BatchMint",
      [mintMsg],
      {
        // amount, gasPrice and gasLimit must be explicitly provided
        version: VERSION,
        amount: new BN(0),
        gasPrice: myGasPrice,
        gasLimit: Long.fromNumber(61000),
        nonce,
      },
      false
    );

    console.log(
      `The transaction id is: https://viewblock.io/zilliqa/tx/0x${callTx.id}?network=testnet for ${n} mints`
    );
    console.log(`Waiting transaction be confirmed`);
    const confirmedTxn = await callTx.confirm(callTx.id);

    console.log(`The transaction status is:`);
    console.log(confirmedTxn.receipt);

    if (confirmedTxn.receipt.success === true) {
      console.log(`Contract address is: ${deployedContract.address}`);
    }
  } catch (err) {
    console.log(err);
  }
}

async function BatchMint() {
  // Get Balance
  const address = getAddressFromPrivateKey(privateKey);
  console.log(`My account address is: ${address}`);
  console.log(`My account bech32 address is: ${toBech32Address(address)}`);
  const { result } = await zilliqa.blockchain.getBalance(address);
  console.log(result);
  let originalNonce;
  if (result) {
    originalNonce = result.nonce;
  } else {
    originalNonce = 0;
  }

  // ==============================================
  const minted = (
    await zilliqa.blockchain.getSmartContractSubState(
      zrc6contract,
      "total_supply"
    )
  )["result"]["total_supply"];

  const maxToMint = 100;
  const leftToMint = maxToMint - minted;
  const numInBatch = 50;

  console.log(leftToMint + " left to mint");
  // ===============================================

  let nonce = originalNonce;

  const callsOfBatch = Math.floor(leftToMint / numInBatch);

  console.log(`${callsOfBatch} callsOf ${numInBatch}`);
  console.log(nonce);

  for (let i = 1; i <= callsOfBatch; i++) {
    console.log(`nonce: ${nonce + i}`);
    IncrementingNonceBatchMint(nonce + i, numInBatch);
  }

  console.log(`nonce: ${nonce + callsOfBatch + 1}`);
  IncrementingNonceBatchMint(nonce + callsOfBatch + 1, leftToMint % numInBatch);
}

BatchMint();
```
