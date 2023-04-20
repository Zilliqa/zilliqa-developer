# EVM ZIL Wallets test

The purpose of this test is to check ZIL and EVM wallet address formats and also to demonstrate sending native ZIL tokens from ZIL address to an EVM address, both of which have been derived from the same private key

### Important dependency

You have to get the isolated server running in your local machine before you can execute this test

Clone the isolated server repo https://github.com/Zilliqa/zilliqa-isolated-server 

Change the Dockerfile to use the correct Zilliqa image. This test was run against v8.9.0rc6 (this has the EVM turned on)

Make sure that the wallet pvt keys in boot.json (in the isolated server) match the pvt keys in the hardhat.config.ts (in this folder). Failure to match that will result in not enough ZILs in the wallets.

### Tests

To run the tests against the isolated server on port 5555

```sh
npm install
npx hardhat test
```

The tests are in `test/EVM_ZIL_Wallets.test.ts` ; edit the parameters at the top of the file to run tests against localdev, testnet or mainnet.



