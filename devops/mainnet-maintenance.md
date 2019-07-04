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