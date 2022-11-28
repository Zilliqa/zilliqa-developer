# Network Upgrade

This doc describes the way to conduct the upgrade entire network. So far we implement the following 2 methods:

- Upgrade on Separated Network
- Rolling Upgrade

See the [link](https://drive.google.com/drive/u/1/folders/1r6xz0zhj-QJr_EEVgKPcYeHw125Dp3Rh) for the presentation of network upgrade, including some demo.

Here is a comparison table between these 2 methods, and meanwhile, the methods would be applied in different scenarios.

|                     | Upgrade on Separated Network | Rolling Upgrade                                                                                                                      |
| ------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Speed               | Fast (2 hrs)                 | Slow (2 days)                                                                                                                        |
| User Experience     | Sense time-shifting          | Seamless and Ethereum adopt [this](https://blog.ethereum.org/2019/02/22/ethereum-constantinople-st-petersburg-upgrade-announcement/) |
| Changes Take Effect | Immediately                  | Delayed by specified epoch                                                                                                           |
| Transaction History | Drop                         | Keep                                                                                                                                 |
| Once Failed         | Try again                    | Target testnet destroyed                                                                                                             |

Following is detailed steps of how to apply these 2 upgrade procedures on the entire network.

## Upgrade on Separated Network

- [Log-in to mkops](https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit), and go to `testnet` folder.

  ```bash
  ssh mkops
  cd testnet
  ```

- Under `<ori_testnet>` folder, upload `<ori_testnet>`'s persistence to S3, and backup key files in local.
  If you want to recover to latest tx epoch, backup immediately:

  ```bash
  cd <ori_testnet>
  ./testnet.sh back-up level2lookup 0
  cd -
  ```

  (Optional) If `./testnet.sh back-up auto` is not executed before, execute this and the persistence will be upload every xxx80 tx epoch.

  ```bash
  cd <ori_testnet>
  ./testnet.sh back-up auto
  cd -
  ```

  Go to [AWS webpage](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-persistence/?region=ap-southeast-1&tab=overview) and make sure `<ori_testnet>.tar.gz` is uploaded to `S3://zilliqa-persistence`.
  The `<ori_cluster>-<ori_testnet>-key.tar.gz` will be generated under `<ori_testnet>` folder.

- (Optional, **Time-machine**) If you want to restore previous tx block in current ds epoch, upload `<ori_testnet>`'s persistence and relative key files to S3 with given `<restoredBlockNum>`.

  ```bash
  cd <ori_testnet>
  ./testnet.sh back-up level2lookup 0 <restoredBlockNum>
  cd -
  ```

- Bootstrap a separate testnet `<new_testnet>` from the original cluster/testnet (`<ori_cluster>`/`<ori_testnet>`), with given `<S3PersistencePath>` (`s3://301978b4-0c0a-4b6b-ad7b-3a2f63c5182c/persistence/<ori_testnet>.tar.gz`) and `<keyFile>` (`<ori_testnet>/<ori_cluster>-<ori_testnet>-key.tar.gz`)

  ```bash
  ./bootstrap.py <new_testnet> \
  --recover-from-s3 <S3PersistencePath> \
  --recover-key-files <keyFile> \
  -c <commit> \
  -t <tag>...
  ```

  Keep the other options the same as much as possible from the `<ori_testnet>`'s bootstrap options.

- Go to AWS webpage for updating `Bucket Policy` of [S3://zilliqa-incremental](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-incremental/?region=ap-southeast-1&tab=permissions), and [S3://zilliqa-statedelta](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-statedelta/?region=ap-southeast-1&tab=permissions); by adding `<new_cluster>` information:

  ```bash
  "Statement": [
      {
          ...
          "Principal": {
              "AWS": [
                  "arn:aws:iam::648273915458:role/nodes.<new_cluster>"
  ```

- Manually confirm the correctness of constant file inside `configmap/constants.xml`.

- Go to `<new_testnet>` folder, and launch it.

  ```bash
  cd <new_testnet>
  ./testhet.sh up
  ```

- (Optional, deprecated after `v4.7.0`) After `<new_testnet>` being launched, remove unnecessary files (`ipMapping.xml`, used to map `<ori_testnet>`'s IP to `<new_testnet>`'s IP).

  ```bash
  ./testnet.sh remove-ipMap
  ```

## Rolling Upgrade

- [Log-in to mkops](https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit)

  ```bash
  ssh mkops
  ```

- Download Zilliqa/Scilla, and check-out to the commit-to-be-upgraded.

  ```bash
  git clone git@github.com:Zilliqa/Zilliqa.git (Mandatory)
  git reset --hard <commit-to-be-upgraded>
  git clone git@github.com:Zilliqa/scilla.git (Optional, download it ONLY when you want to upgrade Scilla)
  ```

  Note that rolling upgrade ONLY can upgrade Zilliqa or Scilla at a time.

- Run `Zilliqa/build/bin/genkeypair` several times to generate some privKey/pubKey pairs, then paste the keys into 2 separated files named `privKeyFile` and `pubKeyFile`.

  ```bash
  ./Zilliqa/build/bin/genkeypair
  ```

- Edit `Zilliqa/scripts/release.sh`.

  ```console
  privKeyFile="../key/privKeyFile"
  pubKeyFile="../key/pubKeyFile"
  testnet_to_be_upgraded="<target_testnet>"
  cluster_name="<target_cluster>"         # eg: dev.k8s.z7a.xyz
  release_bucket_name=""                  # specify the S3 bucket-name that will store released binary
  releaseZilliqa="true"                   # "true" for Zilliqa upgrade, "false" for Scilla upgrade
  scillaPath=""                           # "" for Zilliqa upgrade, "<scilla_path>" for Scilla upgrade
  ```

  Note the variables `<target_testnet>`, `<target_cluster>`, `<scilla_path>` should be changed properly.

- Release Zilliqa/Scilla image to S3.

  ```bash
  cd Zilliqa
  ./scripts/release.sh
  ```

  Go to [AWS webpage](https://s3.console.aws.amazon.com/s3/buckets/zilliqa-release-data/?region=ap-southeast-1&tab=overview) and make sure `<target_testnet>.tar.gz` is uploaded to `s3://zilliqa-release-data`.

- (Optional) Manually confirm the correctness of constant file inside `Zilliqa/constantDir/<type>/constants.xml_<type>`. If anything changes, release Zilliqa/Scilla image to S3 again.

  ```bash
  tar cfz <target_testnet>.tar.gz -C <pubKeyPath> pubKeyFile -C $(realpath ./scripts) miner_info.py -C $(realpath ./scripts) auto_back_up.py -C $(realpath ./scripts) downloadIncrDB.py -C $(realpath ./scripts) download_and_verify.sh -C $(realpath ./scripts) fetchHistorical.py -C $(realpath ./scripts) fetchHistorical.sh -C $(realpath ./scripts) uploadIncrDB.py -C $(realpath ./tests/Zilliqa) daemon_restart.py -C $(realpath release) VERSION -C $(realpath constantsDir) constants.xml -C $(realpath constantsDir/l) constants.xml_lookup -C $(realpath release) <xxx-Linux-Zilliqa.deb> -C $(realpath constantsDir/l2) constants.xml_level2lookup -C $(realpath constantsDir/n) constants.xml_newlookup
  aws s3 cp <target_testnet>.tar.gz s3://zilliqa-release-data/
  ```

  Note the variables `<pubKeyPath>`, `<xxx-Linux-Zilliqa.deb>` (the deb name could be found in `Zilliqa/release/`), `<target_testnet>` should be changed properly.

- Change directory to `<target_testnet>`.

  ```bash
  cd <target_testnet>
  ```

- Apply rolling upgrade for each `<TYPE>`; once completed, running `finish` will remove unnecessary files (`UPGRADE_DONE`, used to mark this POD is upgraded) and update the commit hash inside `<target_testnet>`/manifest/`<TYPE>`.yaml.

  ```bash
  # Zilliqa only can be upgrade type-by-type
  ./testnet.sh upgrade-all <TYPE>
  ./testnet.sh upgrade-all <TYPE> finish

  # Scilla can be upgrade all types in one shot, or type-by-type
  ./testnet.sh upgrade-all all scilla
  ./testnet.sh upgrade-all all finish scilla
  ```

  `<TYPE>` should be applied in following sequence: {`lookup`, `level2lookup`, `newlookup`, `dsguard`, `normal`}.
  For the POD with no Zilliqa process running (e.g., the process crashed, the POD is not alive, etc), rolling upgrade script will attempt to delete & restart the POD first, then apply upgrading.

- (Optional, Zilliqa only) For verifying pupose, you can specify the start/end index for a small-scale rolling upgrade. (Default: rolling upgrade all if nothing specified)

  ```bash
  ./testnet.sh upgrade-all <TYPE> <start_node> <end_node>
  ```

- When running the rolling upgrade, separate log files for upgrading pods would be generated under `upgrade_log/` folder. In the meantime, we can monitor the rolling upgrade status by following:

  ```bash
  ./testnet.sh status | grep <target_testnet>-<TYPE>-<index>   # See if this POD is alive
  ./testnet.sh epoch <TYPE>                                      # List the epoch number of every <TYPE> nodes
  ```

  Or, ssh to the node-under-upgrading, see the rolling upgrade progress:

  ```bash
  tail -f daemon-log.txt        # See if Zilliqa process is restarted successfully
  tail -f state-00001-log.txt   # Monitor the epoch number
  vi zilliqa-00001-log.txt      # Monitor the detail log message
  gdb zilliqa core.xxx          # For zilliqa node crashed, use gdb to debug
  ```
