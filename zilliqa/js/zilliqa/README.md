# @zilliqa-js/zilliqa

JavaScript library for interacting with the Zilliqa blockchain.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Documentation](#documentation)
- [License](#license)

## Installation

```bash
npm install @zilliqa-js/zilliqa --save
```

or

```bash
yarn add @zilliqa-js/zilliqa
```

## Usage

First, you need to import the library:

```javascript
const { Zilliqa } = require("@zilliqa-js/zilliqa");
```

or using ES6 imports:

```javascript
import { Zilliqa } from "@zilliqa-js/zilliqa";
```

Then, create an instance:

```javascript
const zilliqa = new Zilliqa("https://api.zilliqa.com");
```

## Examples

### Sending a Transaction

```javascript
const myAddress = "Your Zilliqa Address";
const recipient = "Recipient Zilliqa Address";
const amount = Zilliqa.utils.units.toQa("1", Zilliqa.utils.units.Units.Zil);

const tx = await zilliqa.blockchain.createTransaction({
  toAddr: recipient,
  amount: amount,
  gasPrice: "2000",
  gasLimit: "1",
});

console.log(`Transaction ID: ${tx.id}`);
console.log(
  `After sending, your new balance is: ${await zilliqa.blockchain.getBalance(
    myAddress
  )}`
);
```

### Fetching Account Balance

```javascript
const myAddress = "Your Zilliqa Address";
const balance = await zilliqa.blockchain.getBalance(myAddress);
console.log(`Your balance is: ${balance}`);
```

## Documentation

For in-depth documentation, please refer to
[Zilliqa Official Documentation](https://dev.zilliqa.com/).

## License

This project is licensed under the GPL License. See [LICENSE](./LICENSE) for
details.
