# Mainnet Maintenance

## Table of Contents

1. [Checklist for Mainnet Activities](#checklist-for-mainnet-activities)
2. [How to Find and Delete Instance for a Pod](#how-to-find-and-delete-instance-for-a-pod)
3. [How to Validate Persistence for a Node](#how-to-validate-persistence-for-a-node)

## Checklist for Mainnet Activities

This comprehensive list is meant as a guiding template for whenever we have to do upgrading, recovery, or other maintenance work around the mainnet. As each situation can be quite unique, it is advised to create an actual checklist by taking the relevant items from this template. This template must also be regularly updated as the nature of our activities evolve.

| #  | ITEM                                           | DESCRIPTION                                                                                                                                                                                                                            | SCENARIO | REFERENCES                                                                                        |
|----|------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----|---------------------------------------------------------------------------------------------------|
| 1  | Manage API ports                               | Do this in the event that we want to prevent transactions or other queries during maintenance period. We also keep a few level2lookups closed for use as reserves by the lookup_autorecover script                              | Launch, recover entire network | Type `./testnet.sh jsonrpc` to see usage                                                          |
| 2  | Validate persistence                           | Should be done regularly and during critical operations to ensure persistence is good                                                                                                                                              | Recover entire network | See [below](#how-to-validate-persistence-for-a-node)                                                                |
| 3  | Manage joining page                            | Evaluate if you need to create or update the joining page                                                                                                                                                                             | Launch, recover entire network |                                                                                                   |
| 4  | Manage versioning                              | (1) Create PR for version update (2) Tag the version (3) Check docker build (4) Update version in README (5) Update version in Mining wiki (6) Create release note (7) Create release branch if necessary        | Release, rolling-upgrade | See **Zilliqa Code Release** Google Doc inside **Infrastructure** folder                          |
| 5  | Manage monitoring scripts                      | We may need to terminate, re-deploy, or start our scripts depending on the situation                                                                                                                                         | Launch, recover entire network | See [Mainnet Monitoring Scripts](mainnet-monitoring-scripts.md)                                     |
| 6  | Update DNS records                             | We may need to switch DNS (for mainnet components and for hosted seeds) depending on the situation                                                                                                                        |             | See [Private API Endpoint](private-api-endpoint.md) for info on DNS management for our hosted seeds |
| 7  | Manage constants file updates                  | Local git repo is maintained in `mkops:/home/ubuntu/mainnet-constant`. Whenever a new mainnet is created, do the following: (1) `git log` (2) `cp <path to new constants.xml> .` (3) `git add -u && git commit -m "mainnet-<name>"` | Recover entire network, rolling-upgrade |                                  |
| 8  | Do spot-check tests                            | Usually we perform these tests during a mainnet upgrade or recovery: (1) transaction processing (2) docker joining (3) native joining                                                              | Recover entire network, rolling-upgrade |                                                                                                   |
| 9  | Manage S3 storage and incremental DB operation | Check existing S3 storage as necessary (e.g., check the persistence file to be used for recovery). Also ensure `uploadIncrDB.py` and `auto_back_up.py` are correctly deployed                    | Launch, recover entire network, rolling-upgrade |                                                                                                   |
| 10 | Manage seed nodes                              | Scale `newlookup` as necessary, and re-register any existing community-hosted seed nodes to our multipliers                                                                                | Launch, recover entire network | See [Private API Endpoint](private-api-endpoint.md) for info on our hosted seeds                    |
| 11 | Terminate unused nodes                         | `normal-0` to `normal-179` were only needed during bootstrap. Terminate the zilliqa process to prevent unnecessarily harassing the lookups with syncing requests. Do this by invoking `for i in {0..179}; do kubectl exec mainnet-<name>-normal-$i -- bash -c "touch SUSPEND_LAUNCH; pkill zilliqa"; done`. | Launch, recover entire network |                |

## How to Find and Delete Instance for a Pod

There are cases where pod deletion is not enough, and we need to delete the instance of the pod to get a working replacement. As this process is not used frequently, it is not automated for now. Here are the steps:

- Locate the node

```bash
kubectl --context brighthill.kube.z7a.xyz get pod mainnet-brightill-dsguard-90 -ojsonpath={..nodeName}
ip-172-20-51-78.us-west-2.compute.internal
```

- Convert node name to EC2 instance ID

```bash
kubectl --context brighthill.kube.z7a.xyz get node ip-172-20-51-78.us-west-2.compute.internal -ojsonpath={.spec.providerID}
aws:///us-west-2a/i-059a343325afc6f9e
```

- Terminate the instance

```bash
ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ aws ec2 terminate-instances --instance-ids i-059a343325afc6f9e --region us-west-2
{
    "TerminatingInstances": [
        {
            "InstanceId": "i-059a343325afc6f9e",
            "CurrentState": {
                "Code": 32,
                "Name": "shutting-down"
            },
            "PreviousState": {
                "Code": 16,
                "Name": "running"
            }
        }
    ]
}
```

- Wait and confirm instance deletion

```bash
ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ aws ec2 describe-instance-status --instance-ids i-059a343325afc6f9e --region us-west-2
{
    "InstanceStatuses": []
}
```

- Confirm the new instance assigned to your pod is of the right specs (you'll have to convert once again the node name to EC2 instance ID)

```bash
ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ aws ec2 describe-instances --instance-ids <new instance id> --region us-west-2
```

## How to Validate Persistence for a Node

We now have a dedicated worker pod `validator-0` which can be used to run the `validatePersistence.sh` script to check the health of any lookup node's persistence. This sequence is now automated in our `testnet.sh` script.

### Optional pre-check

- Check that the validator pod is running

```bash
ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ kubectl get statefulsets
NAME                             DESIRED   CURRENT   AGE
bastion                          1         1         25d
mainnet-brightill-dsguard        420       420       24d
mainnet-brightill-level2lookup   15        15        24d
mainnet-brightill-lookup         5         5         24d
mainnet-brightill-multiplier     5         5         24d
mainnet-brightill-new            0         0         24d
mainnet-brightill-newlookup      7         7         24d
mainnet-brightill-normal         880       880       24d
mainnet-brightill-validator      1         1         16m <--- Check for this statefulset

ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ ./testnet.sh ssh validator 0
root@mainnet-brightill-validator-0:/run/zilliqa# ls
mainnet-brightill-newlookup-0  mainnet-brightill-newlookup-0.tar.gz
```

- If validator pod is not running, execute `./testnet.sh create validator` and try checking again.

### Usage steps

- Just run the `validate` subcommand like this:

```bash
ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ ./testnet.sh validate newlookup 0
persistence/
persistence/txBodiesTmp/
...
persistence/shardStructure/LOCK
persistence/shardStructure/LOG
constants.xml
dsnodes.xml
tar: Removing leading `/' from member names
persistence/
persistence/txBodiesTmp/
...
persistence/shardStructure/LOCK
persistence/shardStructure/LOG
constants.xml
dsnodes.xml
Missing components
```

- The last line should say either "Complete" or "Missing components"
- If you need more details, go into the `common-00001-log.txt` file generated inside the subfolder for that node (in this example, `newlookup-0`):

```bash
ubuntu@ip-172-31-42-105:~/testnet/mainnet-brightill$ ./testnet.sh ssh validator 0
root@mainnet-brightill-validator-0:/run/zilliqa# ls
mainnet-brightill-newlookup-0  mainnet-brightill-newlookup-0.tar.gz
root@mainnet-brightill-validator-0:/run/zilliqa# cd mainnet-brightill-newlookup-0
root@mainnet-brightill-validator-0:/run/zilliqa/mainnet-brightill-newlookup-0# ls
constants.xml  dsnodes.xml  persistence  validateDB
root@mainnet-brightill-validator-0:/run/zilliqa/mainnet-brightill-newlookup-0# cd validateDB/
root@mainnet-brightill-validator-0:/run/zilliqa/mainnet-brightill-newlookup-0/validateDB# ls
common-00001-log.txt  constants.xml  download  dsnodes.xml  persistence
```

### Optional post-usage

- Should the validator pod not be needed anymore, execute `./testnet.sh delete validator` and confirm the pod/statefulset deletion.
