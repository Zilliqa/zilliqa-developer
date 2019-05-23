# Network Upgrade

This doc describe the way to conduct the upgrade entire network. So far we implement following 2 methods:

- Upgrade on Separated Network
- Rolling Upgrade

Hers is comparison between these 2 methods while they would be applied in different scenarios:

|                   |Upgrade on Separated Network|Rolling Upgrade           |
|-------------------|----------------------------|--------------------------|
|Speed              |Fast (2 hrs)                |Slow (2 days)             |
|User Experience    |Sense time-shifting         |Seamless, Ethereum adopt [this](https://blog.ethereum.org/2019/02/22/ethereum-constantinople-st-petersburg-upgrade-announcement/)|
|Changes Take Effect|Immediately                 |Delayed by specified epoch|
|Transaction History|Drop                        |Keep                      |
|Once Failed        |Try again                   |Target testnet destroyed  |

Following is more detail about how to apply these 2 upgrade precedures on entire network.

## Upgrade on Separated Network
- [Login to mkops](https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit)
  ```bash
  ssh mkops
  cd testnet
  ```

- Under `testnet` folder, bootstrap a separated testnet `new_testnet` by original testnet (`ori_cluster`/`ori_testnet`).
  ```bash
  ./bootstrap.py new_testnet --recover-from-testnet ori_testnet --recover-from-cluster ori_cluster -c commit -t tag...
  cd new_testnet
  ```
  Keep the options as the same as possible with the `ori_testnet`'s bootstrap options.

- Under `new_testnet` folder, upload `ori_testnet`'s persistence to S3.
  ```bash
  ./testnet.sh upload ori_cluster ori_testnet
  (For the scenario need to restore previous block (time-machine), use following command alternatively:)
  ./testnet.sh upload ori_cluster ori_testnet level2lookup 0 restoredBlockNum
  ```
  Go to [AWS webpage](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-persistence/?region=ap-southeast-1&tab=overview) and make sure `ori_testnet.tar.gz` is uploaded to `S3://zilliqa-persistence`.

- Manuall confirm the correctness of constant file inside `configmap/constants.xml`.

- Launch `new_testnet`
  ```bash
  ./testhet.sh up
  ```

- After `new_testnet` being launched, remove unnecessary files.
  ```bash
  ./testnet.sh remove-ipMap
  ```
## Rolling Upgrade
- [Login to mkops](https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit)
  ```bash
  ssh mkops
  ```

- Download Zilliqa/Scilla with specified commit
  ```bash
  git clone --recursive git@github.com:Zilliqa/Zilliqa.git (Mandatory)
  git clone git@github.com:Zilliqa/scilla.git (Optional, download it ONLY when you want to upgrade Scilla)
  ```
  Note that rolling upgrade can ONLY upgrade Zilliqa OR Scilla at a time.

- Run `Zilliqa/build/bin/genkeypair` several times to generate some privKey/pubKey pairs, then paste them into 2 separated files named `privKeyFile` and `pubKeyFile`. 
  ```bash
  ./Zilliqa/build/bin/genkeypair
  ```

- Edit `Zilliqa/scripts/release.sh`
  ```console
  privKeyFile="../key/privKeyFile"
  pubKeyFile="../key/pubKeyFile"
  testnet_to_be_upgraded="target_testnet"
  cluster_name="beautyworld.kube.z7a.xyz" # eg: dev.k8s.z7a.xyz
  releaseZilliqa="true"                   # "true" for Zilliqa upgrade, "false" for Scilla upgrade
  scillaPath=""                           # "" for Zilliqa upgrade, "scilla_path" for Scilla upgrade
  ```
- Release Zilliqa/Scilla image to S3
  ```bash
  cd Zilliqa
  ./scripts/release.sh
  ```
  Go to [AWS webpage](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-release-data/?region=ap-southeast-1&tab=overview) and make sure `target_testnet.tar.gz` is uploaded to `S3://zilliqa-release-data`.

- Change directory to `target_testnet` to be rolling upgraded.
  ```bash
  cd target_testnet
  ```

- Apply rolling upgrade for each TYPE, then once completed, remove unnecessary files.
  ```bash
  ./testnet.sh upgrade-all TYPE
  ./testnet.sh upgrade-all TYPE finish
  ```
  TYPE should be applied in sequence: {`lookup`, `level2lookup`, `newlookup`, `dsguard`, `normal`}.

- When running rolling upgrade, separated log files for upgrading pods would be generated under `upgrade_log/` folder.
