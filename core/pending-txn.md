# Pending Txn API

As of Zilliqa version 6.3.0, the pending Txn API contains added the transactions which have been pending or dropped
The error codes corresponding to it are:

| Error Code | Status | Description |
|------------|:------------------------------:|:-----------:|
| 0 |  Confirmed | Txn was already processed and confirmed |
| 1 | Pending | Txn has higher nonce than expected |
| 2 | Pending | Txn Pending because the microblock exceeded gas limit |
| 3 | Pending | Txn Pending due to consensus failure in network |
| 4 | Error | Txn could not be found inside the pool |
| 10 | Dropped |  Txn caused math underflow or overflow |
| 11 | Dropped | Failure in invocation of scilla libraries |
| 12 | Dropped | Failure in contract account initialisation |
| 13 | Dropped | The account from which the transaction is sent is invalid |
| 14 | Dropped | The gas limit of a txn is higher than the shard/DS limit |
| 15 | Dropped | The transaction could not be classified as normal, contract deploy or contract call  |
| 16 | Dropped | The txn is not sharded to the correct shard |
| 17 | Dropped | The contract call txn does not have contract txn and from account in same shard |
| 18 | Dropped | Code of the contract txn is higher than the prescribed limit |
| 19 | Dropped | Verifiaction of transaction failed (Signature check failure, chain id, version check failure) |
| 20 | Dropped | The gas limit of txn is insufficient |
| 21 | Dropped | The account has insufficient balance |
| 22 | Dropped | The transaction has insufficient gas to invoke scilla checker |
| 23 | Dropped | Same Txn was already present |
| 24 | Dropped | A txn with same nonce and higher gas price was present|
| 25 | Dropped | The account for which the transaction is meant for (to address) is invalid. Trying to send contract account zils. or calling a non-contract account. |
| 26 | Dropped | Failure to add the contract account to the state |