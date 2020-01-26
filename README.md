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