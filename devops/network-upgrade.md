# Network Upgrade

This doc describe the way to conduct the upgrade entire network. So far we implement following 2 methods:

- Upgrade on Separated Network
- Rolling Upgrade

There are some pros/cons for each method thus they would be applied in different scenarios. Following is more detail about how to apply these 2 upgrade precedures on entire network.

## Upgrade on Separated Network
- Pros
  - Fast, around 2 hours
  - Changes take effect immediately after upgrading
  - Once failed, just try again

- Steps
  - [Login to mkops](https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit)
  ```bash
  ssh mkops
  cd testnet
  ```
  - Under `testnet` folder, bootstrap a separated testnet `new_testnet` by original testnet (`ori_cluster`/`ori_testnet`). The options should be as the same as possible with the bootstrap options that launch `ori_testnet`.
  ```bash
  ./bootstrap.py new_testnet --recover-from-testnet ori_testnet --recover-from-cluster ori_cluster -c commit -t tag...
  cd new_testnet
  ```
  - Under `new_testnet` folder, upload `ori_testnet`'s persistence to S3. Go to [AWS webpage](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-persistence/?region=ap-southeast-1&tab=overview) and make sure `ori_testnet.tar.gz` is uploaded to `S3://zilliqa-persistence`
  ```bash
  ./testnet.sh upload ori_cluster ori_testnet
  (For the scenario need to restore previous block (time-machine), use following command alternatively:)
  ./testnet.sh upload ori_cluster ori_testnet level2lookup 0 restoredTxBlock
  ```
  - Manuall confirm the correctness of constant file inside `configmap/constants.xml`
  - Launch `new_testnet`
  ```bash
  ./testhet.sh up
  ```
  - After `new_testnet` being launched, remove unnecessary files
  ```bash
  ./testnet.sh remove-ipMap
  ```
## Rolling Upgrade
- Pros
  - Ethereum also adopt [this method](https://blog.ethereum.org/2019/02/22/ethereum-constantinople-st-petersburg-upgrade-announcement/)
  - Seamless and silent
  - All transcation will be kept

- Preparation

- Release

- Rolling upgrade
