# Mainnet Maintenance

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
- If you need more details, go into the `common-00001-log.txt` file generated inside the lookup's subfolder:

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
