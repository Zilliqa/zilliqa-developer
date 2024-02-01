---
id: dev-tools-pyzil
title: pyzil
keywords:
  - python
  - pyzil
  - sdk
  - installation
  - apis
  - examples
  - zilliqa
description: pyzil
---

---

## Introduction

[pyzil](https://github.com/Zilliqa/pyzil) is a python library for
interacting with the Zilliqa network. It can create wallets, deploy contracts,
and invoke transitions to interact with smart contracts on the Zilliqa network.

## Source Code

The github repository can be found at
[https://github.com/Zilliqa/pyzil](https://github.com/Zilliqa/pyzil)

## Releases

All releases of gozilliqa can be found at
[https://github.com/Zilliqa/pyzil/releases](https://github.com/Zilliqa/pyzil/releases)

## Installation

```bash
pip install pyzil
```

or from the source code:

```bash
git clone https://github.com/Zilliqa/pyzil
cd pyzil
pip install -r requirements.txt
python setup.py install
```

## Demo

### Import pyzil

```python
from pprint import pprint

from pyzil.crypto import zilkey
from pyzil.zilliqa import chain
from pyzil.zilliqa.units import Zil, Qa
from pyzil.account import Account, BatchTransfer
```

### Set Active Chain, MainNet or TestNet

```python
chain.set_active_chain(chain.MainNet)
chain.set_active_chain(chain.TestNet)
chain.set_active_chain(chain.IsolatedServer)

SeedNode = chain.BlockChain(
    "https://test-api.zilliqa.com/",
    version=65537, network_id=1)
chain.set_active_chain(SeedNode)
```

### ZILs Transaction

```python
# load account from wallet address
account = Account(address="95B27EC211F86748DD985E1424B4058E94AA5814")
balance = account.get_balance()
print("{}: {}".format(account, balance))

# load account from private key
# private key is required to send ZILs
account = Account(private_key="...")
balance2 = account.get_balance()
print("Account balance: {}".format(balance2))

# to_addr must be bech32 address or 20 bytes checksum address
to_addr = "zil1k5xzgp8xn87eshm3ktplqvs9nufav4pmcm52xx"
# send ZILs
txn_info = account.transfer(to_addr=to_addr, zils=2.718)
pprint(txn_info)
txn_id = txn_info["TranID"]

# wait chain confirm, may takes 2-3 minutes on MainNet
txn_details = account.wait_txn_confirm(txn_id, timeout=300)
pprint(txn_details)
if txn_details and txn_details["receipt"]["success"]:
    print("Txn success: {}".format(txn_id))
else:
    print("Txn failed: {}".format(txn_id))
```

#### Send by Qa

```python
amount = Qa(1234567890)
txn_info = account.transfer(to_addr=to_addr, zils=amount)
pprint(txn_info)
txn_id = txn_info["TranID"]
```

#### Wait for confirm

```python
amount = Zil(3.14)
txn_details = account.transfer(to_addr, zils=amount,
                               confirm=True, timeout=300, sleep=20)
print("Transfer Result:")
pprint(txn_details)
pprint(account.last_params)
pprint(account.last_txn_info)
pprint(account.last_txn_details)

```

#### Batch Transfer (Send zils to multi addresses)

```python
batch = [BatchTransfer(to_addr=to_addr, zils=i) for i in range(10)]
pprint(batch)

txn_info_list = account.transfer_batch(batch)
pprint(txn_info_list)

for txn_info in txn_info_list:
    if not txn_info:
        print("Failed to create txn")
        continue

    txn_details = account.wait_txn_confirm(txn_info["TranID"], timeout=300)
    pprint(txn_details)
    if txn_details and txn_details["receipt"]["success"]:
        print("Txn success")
    else:
        print("Txn failed")

balance2 = account.get_balance()
print("Account balance: {}".format(balance2))
```

#### Send ZILs from nodes to wallet

```python
nodes_keys = [
    "private_key1",
    "private_key2",
    "private_key3",
]

to_address = "your wallet address"
to_account = Account(address=to_address)
print("Account balance: {}".format(to_account.get_balance()))

min_gas = Qa(chain.active_chain.api.GetMinimumGasPrice())

txn_info_list = []
for key in nodes_keys:
    if not key:
       continue
    account = Account(private_key=key)
    # send all zils
    amount = account.get_balance_qa() - min_gas * 2
    if amount <= 0:
        continue

    txn_info = account.transfer(to_addr=to_account.bech32_address, zils=amount, gas_price=min_gas)
    pprint(txn_info)

    txn_info_list.append(txn_info)

for txn_info in txn_info_list:
    txn_details = chain.active_chain.wait_txn_confirm(txn_info["TranID"], timeout=300)
    pprint(txn_details)
    if txn_details and txn_details["receipt"]["success"]:
        print("Txn success")
    else:
        print("Txn failed")

print("Account balance: {}".format(to_account.get_balance()))

```

#### Load account from mykey.txt

```python
account = Account.from_mykey_txt("mykey.txt")
print(account)
```

#### Load account from keystore.json

```python
account = Account.from_keystore("keystore.json")
print(account)

```

See more examples in [tests/test_account.py](https://github.com/Zilliqa/pyzil/blob/main/pyzil/tests/test_account.py)

#### bech32 address

```python
# init from bech32 address
account1 = Account(address="zil1r5verznnwvrzrz6uhveyrlxuhkvccwnju4aehf")
print("address: {}".format(account1.address))
account2 = Account(address="1d19918a737306218b5cbb3241fcdcbd998c3a72")
print("bech32 address: {}".format(account2.bech32_address))
assert account1 == account2

# tranfer to bech32 address
account = Account.from_mykey_txt("mykey.txt")
txn_info = account.transfer(to_addr="zil1r5verznnwvrzrz6uhveyrlxuhkvccwnju4aehf", zils=0.01)
pprint(txn_info)
txn_id = txn_info["TranID"]

```

### Zilliqa Low-level APIs

```python
from pyzil.zilliqa.api import ZilliqaAPI, APIError

api = ZilliqaAPI("https://api.zilliqa.com/")
print(api)

info = api.GetBlockchainInfo()
pprint(info)

sharding = api.GetShardingStructure()
pprint(sharding)

ds_block = api.GetCurrentDSEpoch()
pprint(ds_block)

tx_block = api.GetCurrentMiniEpoch()
pprint(tx_block)

```

See more examples in [tests/test_lowlevel_api.py](https://github.com/Zilliqa/pyzil/blob/main/pyzil/tests/test_lowlevel_api.py)

### Zilliqa Currencies Units

```python
from pyzil.zilliqa.units import Zil, Qa

zil = Zil(1000.123)
print(repr(zil))
assert zil == Zil("1000.123")

qa = Qa(1000123000000000)
print(repr(qa))
assert qa == zil
assert zil == qa

print(repr(zil + qa))
print(repr(zil - 2))
print(repr(zil * 2))
print(repr(zil / 2.0))

print(repr(qa - 2))
print(repr(qa * 2))
print(repr(qa // 2))
```

See more examples in [tests/test_units.py](https://github.com/Zilliqa/pyzil/blob/main/pyzil/tests/test_units.py)

### Zilliqa Smart Contract

```python
from pprint import pprint
from pyzil.zilliqa import chain
from pyzil.account import Account
from pyzil.contract import Contract


chain.set_active_chain(chain.TestNet)

account = Account.from_keystore("zxcvbnm,", "zilliqa_keystore.json")
```

#### Get contract from address

```python
address = "45dca9586598c8af78b191eaa28daf2b0a0b4f43"
contract = Contract.load_from_address(address, load_state=True)
print(contract)
print(contract.status)
pprint(contract.state)
contract.get_state(get_code=True, get_init=True)
pprint(contract.code)
pprint(contract.init)
pprint(contract.state)
```

#### New contract from code

```python
code = open("HelloWorld.scilla").read()
contract = Contract.new_from_code(code)
print(contract)

# set account before deploy
contract.account = account

init = [
    Contract.value_dict("_scilla_version", "Uint32", "0"),
    Contract.value_dict("owner", "ByStr20", account.address0x)
]
contract.deploy(init_params=init, timeout=300, sleep=10)
assert contract.status == Contract.Status.Deployed
```

#### Get contracts

```python
owner_addr = account.address
contracts = Contract.get_contracts(owner_addr)
pprint(contracts)
contracts2 = account.get_contracts()
pprint(contracts2)

assert contracts == contracts2
```

#### Call contract

```python
contract_addr = "45dca9586598c8af78b191eaa28daf2b0a0b4f43"
contract = Contract.load_from_address(contract_addr)

contract.account = account

resp = contract.call(method="getHello", params=[])
pprint(resp)
pprint(contract.last_receipt)

resp = contract.call(method="setHello", params=[Contract.value_dict("msg", "String", "hi contract.")])
pprint(resp)
pprint(contract.last_receipt)

resp = contract.call(method="getHello", params=[])
pprint(resp)
pprint(contract.last_receipt)

# call contract and deposit Zils to contract (default is 0)
resp = contract.call(method="getHello", params=[], amount=Qa(110))
pprint(resp)
pprint(contract.last_receipt)

```

See more examples in [test_contract.py](https://github.com/Zilliqa/pyzil/blob/main/pyzil/tests/test_contract.py)
