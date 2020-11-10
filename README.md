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
For normal use case, the provided constants.xml would be sufficient. For any special requirements, you can edit constants.xml to tailor the isolated server to your specific needs.

## To Create the Image
1) run the following:

`./local_scripts/rebuild_image.sh`

A image named `zilliqa-isolated-server:1.0` will be created on your local docker registry

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
