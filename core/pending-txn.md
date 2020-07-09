# Pending Txn API

## Error Codes

As of Zilliqa version `6.3.0`, the pending Txn API contains added the transactions which have been pending or dropped

The error codes corresponding to it are:

| Error Code | Status | Description |
|:------------:|:------------------------------:|:-----------:|
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
| 19 | Dropped | Verifiaction of transaction failed (Signature , chain id, version check failure) |
| 20 | Dropped | The gas limit of txn is insufficient |
| 21 | Dropped | The account has insufficient balance |
| 22 | Dropped | The transaction has insufficient gas to invoke scilla checker |
| 23 | Dropped | Same Txn was already present |
| 24 | Dropped | A txn with same nonce and higher gas price was present|
| 25 | Dropped | The account for which the transaction is meant for (to address) is invalid. Maybe the txn is trying to send contract account zils or is calling a non-contract account. |
| 26 | Dropped | Failure to add the contract account to the state |

## Pending Transaction Lifecycle

### Broadly two types of transactions are reported by `GetPendingTxns` and `GetPendingTxn`

1. Dropped Transactions (Error code >= 10) : These transactions are somehow deemed to be invalid by the zilliqa nodes and are returned to the lookup. The lookup stores the hash and the error code till only 5 epochs after it was recieved. Hence, these are only reported by the API till the duration of 5 epochs.

2. Pending Transactions (Error code 1 to 3) : These transactions are currently present in the transaction mempool due to the above mentioned reasons. These transactions will be reported by the API till they exist in the mempool.

### Lifecycle

A transaction hash which is reported by the API generally follows this cycle:

1. The lookup accepts the transaction, does premilinary checks and sends it to the shard. This part may introduce some delay as the lookup sends transactions to the shard only after it recieves a final block.

2. The shard processes the transaction and if the transaction is not valid, pushes it to dropped transactions in its own memory. If the transaction if deemed to be valid but cannot be confirmed right now, it is pushed to the mempool. This transaction is hereby pending.

3. The shard then collects all the dropped and pending transactions and sends it to the lookup. This step also may introduce some delay as it only happens after the microblock consensus.

### Exceptions

1. These APIs do not report transactions yet to be dispachted by the lookup to the shard.

2. These APIs do not report transactions which is just recieved by the shard and is under verification/processing.

## Testing

Example Script:

```python

from pprint import pprint

from pyzil.zilliqa import chain
from pyzil.zilliqa.units import Zil, Qa
from pyzil.contract import Contract
from pyzil.account import Account, BatchTransfer

import time
import pytest
import requests
import json


URL = "" #API URL
g_key = "" #private key of an account with sufficient balance
new_keys = ("","") #new account (privkey, addr)

myNet = chain.BlockChain(URL,version=,network_id=) #Fill in the network details
chain.set_active_chain(myNet)

def test_same_nonce(account,addr):
    account.transfer(to_addr = addr, zils = 1, nonce = 3, gas_price= 1000000000)

def send_more_nonce_multiple(account, addr, starting_nonce, num):
    amount = Qa(1000)
    for i in range(starting_nonce,starting_nonce+num):
        account.transfer(to_addr = addr, zils = amount, nonce = i, gas_price=10000000000)

def main():
    g_account = Account(private_key=g_key)
    new_account = Account(private_key=new_keys[0])
    send_more_nonce_multiple(g_account, new_keys[1], 3, 20)
    time.sleep(5)
    test_same_nonce(g_account, new_keys[1])

if __name__ == '__main__':
    main()

```

### Procedure

1. Fill in the necessary information as described by the comments in the script. Ensure you have pyzil installed and the account is new. (Nonce = 0)

2. Run the script. The script would send 20 transactions of nonce which is greater than expected. This should show the code `1` when GetPendingTxns is queried.

3. After 5 seconds, the script sends a transaction with same nonce but lower gas price. This should be rejected with the code `24`.
