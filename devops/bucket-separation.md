
## PERSISTENCE BUCKET CONFIGURATION

To launch a testnet in a cluster with s3 support, you can specify a bucket for s3 operations with the 
`--bucket=<bucket_name>` option in bootstrap. 
For now, we have 3 types of storage we use the s3 for, these all be a new path in the bucket provided by the argument

*NOTE: The default bucket now is zilliqa-devnet*

1. `<bucket_name>`/incremental/`<testnet_name>`: This would contain the persistence snapshot per 10 ds epoch, for joining/rejoining purpose.
2. `<bucket_name>`/statedelta/`<testnet_name>` : This would contain the statedeltas for constructing the state.
3. `<bucket_name>`/persistence : This would contain the persistence tars used for recovery/back-up.

where `bucket_name` is the parameter specified and `testnet_name` is the name assigned to the testnet.

#### Bucket Permissions:

For this bucket, we would need to give write, delete access to the nodes in the cluster. Also due to a caveat in `UploadIncrDB.py` the bucket must be publicly readable.

Example, `zilliqa-devnet` bucket policy

```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:ListBucket",
            "Resource": "arn:aws:s3:::zilliqa-devnet"
        },
        {
            "Effect": "Allow",
            "Principal": {
                "AWS": "arn:aws:iam::273122647034:role/nodes.newton.dev.z7a.xyz.k8s.local"
            },
            "Action": [
                "s3:PutObject",
                "s3:DeleteObject"
            ],
            "Resource": "arn:aws:s3:::zilliqa-devnet/*"
        },
        {
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:GetObject",
            "Resource": "arn:aws:s3:::zilliqa-devnet/*"
        }
    ]
}

```

## RELEASE BUCKET CONFIGURATION

For upgrading, you can specify the `--release-bucket-name` for choosing the bucket to put the release files (.deb and other corresponding files) needed for upgrade. Make sure the same bucket is being used in `release.sh` script in core repo

*NOTE: The default bucket is zilliqa-release-data*

#### Bucket Permissions:

The bucket needs to be publicly readable if we are trying to upgrade community (or public nodes) as well. Otherwise read permission for the cluster nodes and write, delete permission for the bastion node is required.


Example policy:

```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {
                "AWS": "arn:aws:iam::273122647034:role/nodes.newton.dev.z7a.xyz.k8s.local"
            },
            "Action": "s3:ListBucket",
            "Resource": "arn:aws:s3:::zilliqa-test-release"
        },
    ]
}
```

## CONFIGURE BASTION TO GIVE ACCESS TO S3 [OPTIONAL]

If the bastion is already transparently authenticated, then no need for this step

1. Go to zilliqa login page and login
2. Choose the role which has S3 access.
3. Choose `Command line or programmatic access.`
4. Copy keys and export into bastion

You may have to modify certain commands (for example in [release.sh](https://github.com/Zilliqa/Zilliqa/blob/627caccb948e52a91f72384422692186d79e4fb3/scripts/release.sh#L291)) so the request sent is authenticated. 
