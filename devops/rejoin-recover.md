## Recover

  The `recover` is a kubectl-based command that can recover broken node(s) from using a healthy node's persistence. Here is the command prototype:

  ```bash
  ./testnet.sh recover TYPE "INDEX1 INDEX2 INDEX3 ..." [-u UPLOAD_TYPE UPLOAD_INDEX] 
  ```

  If not specify `-u`, by default we will use persistence from `lookup-0` for recovering `level2lookup-x`, and use persistence from `level2lookup-0` for recovering al the other nodes.

- Scenario 1:
  If you want to recover `dsguard-0`, please type:
  
  ```bash
  ./testnet.sh recover dsguard 0
  ```

  We don't specify `-u` here, by default it will use persistence from `level2looup-0`.

- Scenario 2:
  If you want to recover `level2lookup-0`, please type:
  
  ```bash
  ./testnet.sh recover level2lookup 0
  ```

  We don't specify `-u` here, by default it will use persistence from `lookup-0`.

- Scenario 3:
  If you want to recover `normal-3`, `normal-55`, `normal-77`, please type:

  ```bash
  ./testnet.sh recover normal "3 55 77"
  ```

  We don't specify `-u` here, by default it will use persistence from `level2looup-0`.

- Scenario 4:
  If you want to recover `normal-0` using `dsguard-3`, please type:
  
  ```bash
  ./testnet.sh recover normal 0 -u dsguard 3
  ```

- Scenario 5:
  If you want to recover `normal-0`, `normal-4`, `normal-52` using `lookup-9`, please type:
  
  ```bash
  ./testnet.sh recover normal "0 4 52" -u lookup 9
  ```

## Rejoin

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
