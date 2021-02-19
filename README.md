# Dockerized Zilliqa Isolated Server

Based on https://github.com/Zilliqa/Zilliqa/pull/1879

`boot.json` contains default accounts

Build image

```docker build --rm -f "Dockerfile" -t isolatedserver:1 "."```

Run by 

`docker run -d -p 5555:5555 isolatedserver:1`

Server will be accessible on `http://localhost:5555`

Available APIs

- `CreateTransaction` : Input a transaction json payload
- `IncreaseBlocknum` : Increase the blocknum by a given input
- `GetSmartContractSubState` : Get the state of a smart contract
- `GetSmartContractCode` : Get code at a given address
- `GetMinimumGasPrice` : Get the minimum gas price
- `SetMinimumGasPrice`: Set the minimum gas price
- `GetBalance`: Get balance and nonce of a account
- `GetSmartContracts`: get smart contract for an address
- `GetNetworkID`
- `GetSmartContractInit` : get init json for a SC.
- `GetTransaction `: Get Transaction info by hash

# Local Isolated Server Administration
If you need to run an local instance of isolated server with a loaded state use the following instructions

## Edit the constants.xml
You would need to download the latest constants.xml file that can be found on the mainnet join page
```
curl -O https://mainnet-join.zilliqa.com/seed-configuration.tar.gz
tar -zxvf seed-configuration.tar.gz
vim constants.xml
```

Keep the following tags set to these values
- `<LOOKUP_NODE_MODE>true</LOOKUP_NODE_MODE>`
- `<MAX_ENTRIES_FOR_DIAGNOSTIC_DATA>50</MAX_ENTRIES_FOR_DIAGNOSTIC_DATA>`
- `<CHAIN_ID>222</CHAIN_ID>`
- `<ENABLE_SC>true</ENABLE_SC>`
- `<SCILLA_ROOT>/scilla</SCILLA_ROOT>`


## To Create the Image
1) run the following:

`./local_scripts/rebuild_image.sh`

An image named `zilliqa-isolated-server:1.0` will be created on your local docker registry

## Preparing the Persistence
1) If you know the mainnet bucket id that you're using, run the following:

`export BUCKET_ID=<insert bucket id here> && ./local_script/download_persistence.sh <persistence file>`

2) If you do not have the mainnet bucket id, run the following:

Get someone with mainnet access to pass u the latest backed up persistence

`mkdir downloads`

copy the persistence file into the downloads folder

## Launch the Isolated Server
1) `./local_script/run_isolated_with_persistence.sh <persistence file>`

## Stop the Isolated Server
1) `./local_script/stop_isolated.sh`
