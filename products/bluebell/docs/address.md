## `Address` type format

The `Address` data structure in Scilla represents the 20-byte Ethereum-like
address format. It is utilized to identify contracts and external accounts
within the Zilliqa blockchain network.

```text
+--------------------+
|     20 bytes       |
|                    |
|  Address Payload   |
+--------------------+
```

Addresses are fundamental in various aspects of contract development in Scilla,
such as sending and receiving funds, calling other contracts, or designating
permissions. It is crucial to ensure the correctness of these addresses to
prevent unintentional transactions.

```scilla
(* Example of contract using Scilla address *)

import ListUtils

contract SimpleStorage(owner: ByStr20)

field storedData : Int32 = 0

transition Store(newData : Int32)
  is_owner = builtin eq owner _sender;
  match is_owner with
  | False =>
    e = {_eventname : "Unauthorized"; msg : "You are not the contract owner."};
    event e
  | True =>
    storedData := newData;
    e = {_eventname : "DataStored"; data : newData};
    event e
  end
end
```

## Address Verification

To verify the validity of a Scilla address, one typically checks the following:

1. Ensure the address has a length of 20 bytes.
2. Check if the address does not contain non-hexadecimal characters.
3. For contract addresses, confirm that a corresponding contract exists on the
   blockchain.

It's also common to employ checksums to further validate addresses, though this
isn't mandated by Scilla or Zilliqa.

## Print `Address`

The new `print` function allows developers to print the `Address` types, which
is especially useful for debugging purposes. The function outputs the address in
its standard hexadecimal format.

```scilla
(* Example of using print to display Address *)

transition PrintAddress()
  owner_address = owner; (* Fetch the owner address *)
  print owner_address;
end
```

## Using Addresses with `zilliqa-js`

The `zilliqa-js` library provides a comprehensive suite of functions to manage,
validate, and work with addresses in Zilliqa's blockchain. Here's how you can
work with addresses using `zilliqa-js`:

### Initializing the SDK

Before performing any operations, you need to initialize the SDK.

```javascript
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const zilliqa = new Zilliqa("https://api.zilliqa.com/");
```

### **Creating a New Address**

To generate a new keypair and associated address:

```javascript
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const privateKey = zilliqa.wallet.create();
const address = getAddressFromPrivateKey(privateKey);
console.log(`Address: ${address}`);
```

### Validating an Address

Before any operations involving an address, it's always good practice to
validate it:

```javascript
const { validation } = require("@zilliqa-js/util");
const isValid = validation.isAddress("Your_Address_Here");
console.log(`Is valid address: ${isValid}`);
```

### Converting Between Address and Bech32

Zilliqa uses the Bech32 format for human-readable addresses. Here's how to
convert between a standard address and its Bech32 format:

```javascript
const { toBech32Address, fromBech32Address } = require("@zilliqa-js/crypto");

const bech32 = toBech32Address("Your_Address_Here");
console.log(`Bech32 format: ${bech32}`);

const originalAddress = fromBech32Address(bech32);
console.log(`Original format: ${originalAddress}`);
```

### Using an Address in Transactions

When you're sending a transaction, you'll typically need to specify the
recipient's address:

```javascript
const tx = zilliqa.transactions.new({
  toAddr: "Recipient_Address_Here",
  amount: zilliqa.utils.units.toQa("1", zilliqa.utils.units.Units.Zil), // 1 ZIL
  gasPrice: zilliqa.utils.units.toQa("1000", zilliqa.utils.units.Units.Li), // Gas Price in Li
  gasLimit: Long.fromNumber(50),
});
```

Always remember to handle private keys securely. Avoid exposing them in
client-side code or any public space.
