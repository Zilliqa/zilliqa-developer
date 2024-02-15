# @zilliqa-js/subscriptions

JavaScript library for managing and handling subscription-based events with the
Zilliqa blockchain.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Supported Events](#supported-events)
- [Documentation](#documentation)
- [License](#license)

## Installation

```bash
npm install @zilliqa-js/subscriptions --save
```

or

```bash
yarn add @zilliqa-js/subscriptions
```

## Usage

To start using the subscriptions package, first import the necessary components:

```javascript
const { SubscriptionBuilder, Zilliqa } = require("@zilliqa-js/subscriptions");
```

or using ES6 imports:

```javascript
import { SubscriptionBuilder, Zilliqa } from "@zilliqa-js/subscriptions";
```

## Examples

### Subscribe to New Blocks

```javascript
const zilliqa = new Zilliqa("https://api.zilliqa.com");
const subscription = zilliqa.subscriptionBuilder.buildNewBlockSubscriptions();

subscription.subscribe();
subscription.emitter.on(StatusType.SUBSCRIBE_NEW_BLOCK, (event) => {
  console.log("New block:", event.value);
});
```

### Unsubscribing

After subscribing, you can also unsubscribe from events:

```javascript
subscription.unsubscribe();
```

## Supported Events

- New Block Subscriptions (`SUBSCRIBE_NEW_BLOCK`)
- Transaction Subscriptions (`SUBSCRIBE_TX`)
- [Add more supported events as per the library]

## Documentation

For comprehensive documentation, please refer to
[Zilliqa Official Documentation](https://dev.zilliqa.com/).

## License

This project is licensed under the GPL License. View [LICENSE](./LICENSE) for
more information.
