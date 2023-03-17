# NodeJS Examples

This section will demonstrate how to use the **NodeJS variant** of ZilliqaJS in the Node way. This can also be used on frontend frameworks and is the **recommended** approach to use ZilliqaJS.

## How to get started

There is an existing `package.json` with `@zilliqa-js/zilliqa`, `tslib` and `bn.js` dependency. Just perform the usual `yarn install` or `npm install` and invoke the respective examples.

```
cd node
yarn add @zilliqa-js/zilliqa
yarn add tslib
yarn add bn.js
yarn install

node helloWorld.js
```

Sample `helloWorld.js` output:

```
// console output:
My account address is: 0x8254b2C9aCdf181d5d6796d63320fBb20D4Edd12
My account bech32 address is: zil1sf2t9jdvmuvp6ht8jmtrxg8mkgx5ahgj6h833r

Your account balance is:
{ balance: '10722145999990000', nonce: 1468 }

Current Minimum Gas Price: 2000000000
My Gas Price 2000000000
Is the gas price sufficient? true

Sending a payment transaction to the network...

The transaction id is: 035a2ae08d0b4d12f31ee6c0315d91b4bb9150c1f078fc88e0ee3b5640f2d318

The transaction status is:
{ cumulative_gas: 50, epoch_num: '3129050', success: true }

Deploying a new contract....
Deployment Transaction ID: 6a67c5d1fb65f7fae9c02ee81b37fb5855bbfd3d17d163a908bb5f667419d1df

Deployment Transaction Receipt:
{ cumulative_gas: 482, epoch_num: '3129052', success: true }
The contract address is:
0xD1F5c962F1c6A77253BFD799B3472D05de414ae2

Calling setHello transition with msg: Hello, the time is 1629086853792
{
    "accepted": false,
    "cumulative_gas": 357,
    "epoch_num": "3129054",
    "event_logs": [
        {
            "_eventname": "setHello()",
            "address": "0xd1f5c962f1c6a77253bfd799b3472d05de414ae2",
            "params": [
                {
                    "type": "Int32",
                    "value": "2",
                    "vname": "code"
                }
            ]
        }
    ],
    "success": true
}

Getting contract state...
The state of the contract is:
{
    "_balance": "0",
    "welcome_msg": "Hello, the time is 1629086853792"
}
```
