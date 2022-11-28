# Transaction Checks

## Lookup level

[Validate Txn Function](https://github.com/Zilliqa/Zilliqa/blob/7b684a25f81dd4e790f596ca19672990c41d4b72/src/libServer/LookupServer.cpp#L309)

1. Check if the chain ID matches the chain ID of the network. This is done to prevent replay attacks.
2. Check if the txn code size is within the limits of max code size.
3. Check if the txn gas price is greater than the minimum gas price required
4. Check the signature of the corresponding pubKey in the txn
5. Check if the sender has non-zero balance
6. If it is a contract creation transaction, check if the txn gas is greater than the minimum gas for a contract creation transaction
7. If it is a contract call transaction, check if the txn gas is greater than the minimum gas for a contract call transaction.
8. Check if the txn nonce is not less than the sender nonce.

## Shard Level

[CheckCreatedTransactionFromLookup](https://github.com/Zilliqa/Zilliqa/blob/7b684a25f81dd4e790f596ca19672990c41d4b72/src/libValidator/Validator.cpp#L94)

1. Checks 1,2,3,4,5 are similar as lookup level.
2. Also if it is a non-DS node, check if the txn is in correct shard.
3. Also check if the sender has enough balance for the txn.
