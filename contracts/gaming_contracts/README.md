## Gaming SmartContracts

By default these smart contracts are running on local isolated server but you can always change the configuration in `./test.config.js`. For changing the network please change `defaultNetwork` key to the supported network `local_isolated_server | isolated_server | testnet`. For other networks apart from `local_isolated_server` you need to add zils to the private keys in `./test.config.js` against the network you have selected to continue the operations.

### The local isolated server comes with default accounts with ZIL balance so no need to topup. To install isolated server on your local machine:
`
https://github.com/Zilliqa/zilliqa-isolated-server
`

### Facet for public Isolated server and Testnet:
`
https://dev-wallet.zilliqa.com/faucet?network=testnet
`

To run the test:
```shell
npm run test
```
