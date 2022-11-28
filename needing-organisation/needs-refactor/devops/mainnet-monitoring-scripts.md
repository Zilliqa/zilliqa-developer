# Mainnet Monitoring Scripts

This document lists the different scripts we have deployed for monitoring the health of the mainnet across various aspects of its operation.

All scripts are currently located in the [testnet](https://github.com/Zilliqa/testnet/monitoring) repo.

All scripts are usually launched with a Slack webhook URL as an input parameter. Click [here](https://api.slack.com/incoming-webhooks) to learn more about setting up a webhook.

> **Note:** Our webhooks for **stall-alert** and **testnet-alert** channel can be found in the **Slack Webhooks** Google Doc under our Design Docs folder in Google Drive

## Deployment locations

As of Zilliqa v4.8.0, we have the following scripts deployed:

| Script                | Machine                     | Location                                          | Run Parameters                                                                                                                                                                       |
| --------------------- | --------------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| ds-ip-check.py        | MonitoringBastion (Mainnet) | ~/environment/mainnet_somerset                    | nohup ./ds-ip-check.py --context \<somerset\> --dscount 420 --frequency 720 --testnet mainnet-somerset --webhook \<stall-alert URL\> &                                               |
| lookup_autorecover.py | MonitoringBastion (Mainnet) | ~/environment/mainnet_somerset                    | nohup ./lookup_autorecover.py --context \<somerset\> --frequency 5 --liveness 100 --testnet mainnet-somerset --webhookalert \<stall-alert URL\> --webhookalive \<status URL\> &      |
| persistence-check.py  | MonitoringBastion (Mainnet) | ~/environment/mainnet_somerset                    | nohup python3.4 ./persistence-check.py --apiurl `https://api.zilliqa.com` --concurrent 2 --context \<somerset\> --frequency 12 --testnet mainnet-somerset --webhook \<status URL\> & |
| monitor_blockchain.py | MonitoringBastion (DevOps)  | ~/environment/monitors_mainnet/monitor_blockchain | nohup stdbuf -oL ./monitor_blockchain.py `https://api.zilliqa.com` -w \<ZilliqaDevelopment stall-alert URL\> \<Zilliqa stall-alert URL\> > nohup.out &                               |
| txn-sanity-check.py   | Currently not deployed      |                                                   |

## DS guard IP address consistency check (ds-ip-check.py)

This script retrieves the IP addresses of the DS guards and cross-checks these against the DS committee list returned by querying "GetDSCommittee" in each DS guard.

```bash
antonio@antonio-Latitude-7490:~/testnet/testnet$ ./monitoring/ds-ip-check.py --help
usage: ds-ip-check.py [-h] --context CONTEXT --dscount DSCOUNT
                      [--frequency FREQUENCY] --testnet TESTNET
                      [--webhook WEBHOOK]

Script to check for DS guard IP change

optional arguments:
  -h, --help            show this help message and exit
  --context CONTEXT     Cluster name
  --dscount DSCOUNT     Number of DS guards
  --frequency FREQUENCY
                        Checking frequency in minutes (default = 0 or run
                        once)
  --testnet TESTNET     Testnet name (e.g., mainnet-beautyworld)
  --webhook WEBHOOK     Slack webhook URL
```

### Deploying the script (ds-ip-check.py)

- Copy `ds-ip-check.py` into your testnet folder
- Execute the script as a background process, like this, for example:

```bash
nohup stdbuf -oL ./ds-ip-check.py \
 --context brighthill.kube.z7a.xyz \
 --testnet mainnet-brightill \
 --dscount 420 \
 --webhook https://hooks.slack.com/services/ABCDEFGHI/JKLMNOPQR/Abcdefghijklmnopqrstuvwx \
 --frequency 60 > nohup-ds-ip-check.out &
```

### While the script is running (ds-ip-check.py)

A report is sent to the Slack webhook in any of the following cases:

- The script failed to query the DS committee list from a significant number of DS guards (controlled by the constant `FAULTY_MINERINFO_TOLERANCE` inside the script)
- One or more IP mismatches are detected
- An exception occurs during the process

To check what the report looks like, you can run the script in test mode (set the constant `TEST_MODE` to `True` inside the script). This will introduce some random errors during the run.

### Terminating the script (ds-ip-check.py)

Simply kill the process.

## Lookup monitor and recovery (lookup_autorecover.py)

This script retrieves the epoch number of each lookup and recovers the ones that are not within the threshold of the latest epoch number. It also looks for replacement lookups to service API requests.

```bash
ansnunez@ansnunez-Latitude-7490:~/testnet/testnet/monitoring$ ./lookup_autorecover.py --help
usage: lookup_autorecover.py [-h] --context CONTEXT [--debugmode]
                             [--frequency FREQUENCY] [--liveness LIVENESS]
                             [--testmode] --testnet TESTNET
                             [--webhookalert WEBHOOKALERT]
                             [--webhookalive WEBHOOKALIVE]

Lookup autorecovery script

optional arguments:
  -h, --help            show this help message and exit
  --context CONTEXT     Testnet cluster (e.g., fortcanning.kube.z7a.xyz)
  --debugmode           Print but avoid executing the actual testnet commands
  --frequency FREQUENCY
                        Polling frequency in minutes (default = 0 or run once)
  --liveness LIVENESS   Send liveness message to Slack (default = 10 or after
                        every 10th run)
  --testmode            Use test/dummy data instead of real one
  --testnet TESTNET     Testnet name (e.g., mainnet-fortcanning)
  --webhookalert WEBHOOKALERT
                        Slack webhook URL for alert messages
  --webhookalive WEBHOOKALIVE
                        Slack webhook URL for liveness message
```

### Deploying the script (lookup_autorecover.py)

- Copy `lookup_autorecover.py` into your testnet folder
- Execute the script as a background process, like this, for example:

```bash
nohup ./lookup_autorecover.py \
 --context somerset.cluster.abc.def \
 --frequency 5 \
 --liveness 100 \
 --testnet mainnet-somerset \
 --webhookalert https://hooks.slack.com/services/ABCDEFGHI/JKLMNOPQR/Abcdefghijklmnopqrstuvwx \
 --webhookalive https://hooks.slack.com/services/STUVWXYZ/JKLMNOPQR/lmnopqrstuvwxAbcdefghijk &
```

- For the first time executing the script, it is recommended to run with option `--testmode` first to confirm that the script works with dummy test data, and then to run a second time with option `--debugmode` to confirm that the script works with real data but without actually executing the recovery commands yet.

### While the script is running (lookup_autorecover.py)

A report is sent to the Slack `webhookalert` channel in any of the following cases:

- One or more lookups were recovered
- A non-numeric or unrecognized epoch number format was processed
- An exception occurs during the process

A report is sent to the Slack `webhookalive` channel in any of the following cases:

- After initially launching the script
- After every `liveness` number of runs

### Terminating the script (lookup_autorecover.py)

Simply kill the process.

## Tx block mining stall checker (monitor_blockchain.py)

This script checks if no new Tx block has been mined for a specified amount of time.

```bash
ansnunez@ansnunez-Latitude-7490:~/testnet/testnet$ ./monitoring/monitor_blockchain.py --help
usage: monitor_blockchain.py [-h] [--frequency FREQUENCY] [--output OUTPUT]
                             [--timeout TIMEOUT] --webhook WEBHOOK
                             [WEBHOOK ...]
                             url

positional arguments:
  url                   URL for querying

optional arguments:
  -h, --help            show this help message and exit
  --frequency FREQUENCY, -f FREQUENCY
                        Polling frequency in seconds {default: 60}
  --output OUTPUT, -o OUTPUT
                        output csv names, {default=status}
  --timeout TIMEOUT, -t TIMEOUT
                        The time if a tx block is not mined, will alert (in
                        minutes) (default=14)
  --webhook WEBHOOK [WEBHOOK ...], -w WEBHOOK [WEBHOOK ...]
                        WebHook to send to slack or any other service
```

### Deploying the script (monitor_blockchain.py)

This script has no dependence on the testnet files. As such, it can be run from anywhere. The steps below are for running it in a new pod in the `dev` cluster.

- Create a `<podname>.yaml` file with the following contents (in this example, we will use `my-stall-checker` for the `<podname>`):

```bash
kind: Pod
apiVersion: v1
metadata:
  name: <podname>
spec:
  containers:
    - name: <podname>
      image: ubuntu
      command: ["/bin/bash", "-ce", "tail -f /dev/null", ]
  restartPolicy: OnFailure
```

- Verify you are in the right cluster:

```bash
antonio@ip-172-31-44-129:~$ kubectl config current-context
dev.k8s.z7a.xyz
```

- Launch the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl create -f my-stall-checker.yaml
pod "my-stall-checker" created
antonio@ip-172-31-44-129:~$ kubectl get pod my-stall-checker
NAME               READY     STATUS    RESTARTS   AGE
my-stall-checker   1/1       Running   0          2m
```

- Create a folder in the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl exec -it my-stall-checker -- /bin/bash
root@my-stall-checker:/# mkdir stallchecker
root@my-stall-checker:/# ls
bin  boot  dev  etc  home  lib  lib64  media  mnt  opt  proc  root  run  sbin  srv  stallchecker  sys  tmp  usr  var
root@my-stall-checker:/# exit
exit
antonio@ip-172-31-44-129:~$
```

- Copy the script into the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl cp monitor_blockchain.py my-stall-checker:/stallchecker/
```

- Execute the script (here `python3.6` installation is also done):

```bash
antonio@ip-172-31-44-129:~$ kubectl exec -it my-stall-checker -- /bin/bash
root@my-stall-checker:/# apt-get update
...
root@my-stall-checker:/# apt-get install software-properties-common
...
root@my-stall-checker:/# add-apt-repository ppa:jonathonf/python-3.6
...
root@my-stall-checker:/# apt-get update
...
root@my-stall-checker:/# apt-get install python3.6
...
root@my-stall-checker:/# python3 -V
Python 3.6.8
root@my-stall-checker:/# apt-get -y install python3-pip
...
root@my-stall-checker:/# pip3 install requests
...
root@my-stall-checker:/# cd stallchecker
root@my-stall-checker:/stallchecker# nohup stdbuf -oL ./monitor_blockchain.py https://api.zilliqa.com \
 -w https://hooks.slack.com/services/ABCDEFGHI/JKLMNOPQR/Abcdefghijklmnopqrstuvwx \
 > nohup-monitor-blockchain.out &
```

### While the script is running (monitor_blockchain.py)

A report is sent to the Slack webhook in the event that the stall timeout (default = 14 minutes) has been triggered.
Another report is sent when the stall is averted.

### Terminating the script (monitor_blockchain.py)

- Kill the process inside the pod
- Delete the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl delete pod my-stall-checker
pod "my-stall-checker" deleted
antonio@ip-172-31-44-129:~$ kubectl get pod my-stall-checker
NAME               READY     STATUS        RESTARTS   AGE
my-stall-checker   1/1       Terminating   0          44m
antonio@ip-172-31-44-129:~$ kubectl get pod my-stall-checker
Error from server (NotFound): pods "my-stall-checker" not found
```

## Transaction processing health checker (txn-sanity-check.py)

This script sends ZILs back-and-forth between two addresses to check if the network is still successfully processing transactions.

> Limitation: The script only works for a network with `CHAIN_ID=1`

```bash
root@txn-sanity-check:/antonio# ./txn-sanity-check.py --help
usage: txn-sanity-check.py [-h] --srchex SRCHEX --srczil SRCZIL --srckey
                           SRCKEY --dsthex DSTHEX --dstzil DSTZIL --dstkey
                           DSTKEY [--frequency FREQUENCY] --apiurl APIURL
                           [--webhook WEBHOOK]

Script to check if testnet can still process txns (NOTE: This only works for
CHAIN_ID=1)

optional arguments:
  -h, --help            show this help message and exit
  --srchex SRCHEX       Src address (base16, omit 0x)
  --srczil SRCZIL       Src address (bech32)
  --srckey SRCKEY       Src privkey (omit 0x)
  --dsthex DSTHEX       Dst address (base16, omit 0x)
  --dstzil DSTZIL       Dst address (bech32)
  --dstkey DSTKEY       Dst privkey (omit 0x)
  --frequency FREQUENCY
                        Checking frequency in minutes (default = 0 or run
                        once)
  --apiurl APIURL       URL for querying
  --webhook WEBHOOK     Slack webhook URL
```

### Deploying the script (txn-sanity-check.py)

This script has no dependence on the testnet files. As such, it can be run from anywhere. The steps below are for running it in a new pod in the `dev` cluster.

- Create a `<podname>.yaml` file with the following contents (in this example, we will use `my-txn-checker` for the `<podname>`):

```bash
kind: Pod
apiVersion: v1
metadata:
  name: <podname>
spec:
  containers:
    - name: <podname>
      image: ubuntu
      command: ["/bin/bash", "-ce", "tail -f /dev/null", ]
  restartPolicy: OnFailure
```

- Verify you are in the right cluster:

```bash
antonio@ip-172-31-44-129:~$ kubectl config current-context
dev.k8s.z7a.xyz
```

- Launch the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl create -f my-txn-checker.yaml
pod "my-txn-checker" created
antonio@ip-172-31-44-129:~$ kubectl get pod my-txn-checker
NAME               READY     STATUS    RESTARTS   AGE
my-txn-checker   1/1       Running   0          2m
```

- Create a folder in the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl exec -it my-txn-checker -- /bin/bash
root@my-txn-checker:/# mkdir txnchecker
root@my-txn-checker:/# ls
bin  boot  dev  etc  home  lib  lib64  media  mnt  opt  proc  root  run  sbin  srv  txnchecker  sys  tmp  usr  var
root@my-txn-checker:/# exit
exit
antonio@ip-172-31-44-129:~$
```

- Copy the script into the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl cp txn-sanity-check.py my-txn-checker:/txnchecker/
```

- Execute the script (here `python3.6` and `pyzil` installation are also done):

```bash
antonio@ip-172-31-44-129:~$ kubectl exec -it my-txn-checker -- /bin/bash
root@my-txn-checker:/# apt-get update
...
root@my-txn-checker:/# apt-get install software-properties-common
...
root@my-txn-checker:/# add-apt-repository ppa:jonathonf/python-3.6
...
root@my-txn-checker:/# apt-get update
...
root@my-txn-checker:/# apt-get install python3.6
...
root@my-txn-checker:/# python3 -V
Python 3.6.8
root@my-txn-checker:/# apt-get -y install python3-pip
...
root@my-txn-checker:/# apt-get install python3.6-dev
...
root@my-txn-checker:/# apt-get install libgmp-dev
...
root@my-txn-checker:/# pip3 install pyzil
...
root@my-txn-checker:/# cd txnchecker
root@my-txn-checker:/txnchecker# nohup stdbuf -oL ./txn-sanity-check.py \
 --srchex 0102030405060708090a0b0c0d0e0f1011121314 \
 --srczil zil1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5f99mqr \
 --srckey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20 \
 --dsthex 1112131415161718191a1b1c1d1e1f2021222324 \
 --dstzil zil1zyfpx9q4zct3sxg6rvwp68slyqsjygey5uczk5 \
 --dstkey 1112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30 \
 --frequency 15 \
 --apiurl https://api.zilliqa.com \
 --webhook https://hooks.slack.com/services/ABCDEFGHI/JKLMNOPQR/Abcdefghijklmnopqrstuvwx \
 > txn-sanity-check.log &
```

### While the script is running (txn-sanity-check.py)

A report is sent to the Slack webhook in the event the `pyzil` operations fail.

### Terminating the script (txn-sanity-check.py)

- Kill the process inside the pod
- Delete the pod:

```bash
antonio@ip-172-31-44-129:~$ kubectl delete pod my-txn-checker
pod "my-txn-checker" deleted
antonio@ip-172-31-44-129:~$ kubectl get pod my-txn-checker
NAME               READY     STATUS        RESTARTS   AGE
my-txn-checker   1/1       Terminating   0          44m
antonio@ip-172-31-44-129:~$ kubectl get pod my-txn-checker
Error from server (NotFound): pods "my-txn-checker" not found
```

## Persistence health checker (persistence-check.py)

This script performs `testnet.sh validate` on all the lookups' persistence to check for any errors.

```bash
ansnunez@ansnunez-Latitude-7490:~/testnet/testnet/monitoring$ ./persistence-check.py --help
usage: persistence-check.py [-h] --apiurl APIURL [--concurrent CONCURRENT]
                            --context CONTEXT [--frequency FREQUENCY]
                            --testnet TESTNET [--webhook WEBHOOK]

Lookup persistence auto-checker script

optional arguments:
  -h, --help            show this help message and exit
  --apiurl APIURL       URL for querying
  --concurrent CONCURRENT
                        Number of concurrent validations (default = 1)
  --context CONTEXT     Testnet cluster (e.g., fortcanning.kube.z7a.xyz)
  --frequency FREQUENCY
                        Polling frequency in hours (default = 0 or run once)
  --testnet TESTNET     Testnet name (e.g., mainnet-fortcanning)
  --webhook WEBHOOK     Slack webhook URL (default = no URL)
```

### Deploying the script (persistence-check.py)

- Copy `persistence-check.py` into your testnet folder
- Execute the script as a background process, like this, for example:

```bash
nohup ./persistence-check.py \
 --apiurl https://api.zilliqa.com \
 --concurrent 2 \
 --context brighthill.kube.z7a.xyz \
 --frequency 12 \
 --testnet mainnet-brighthill \
 --webhook https://hooks.slack.com/services/ABCDEFGHI/JKLMNOPQR/Abcdefghijklmnopqrstuvwx &
```

### While the script is running (persistence-check.py)

A report is sent to the Slack webhook both when the script starts a new run and after the run completes.

### Terminating the script (persistence-sanity-check.py)

Simply kill the process.
