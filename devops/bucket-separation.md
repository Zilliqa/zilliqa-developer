# Bucket Configuration

## Persistence Bucket Configuration

To launch a testnet in a cluster with s3 support, you can specify a bucket
for s3 operations with the `--bucket=<bucket_name>` option in bootstrap.

For now, we have 3 types of storage we use the s3 for, these all will be a new path
in the bucket provided by the argument.

*NOTE:* The default bucket now is zilliqa-devnet

1. `<bucket_name>`/incremental/`<testnet_name>`: This would contain the persistence
snapshot per 10 ds epoch, for joining/rejoining purpose.
2. `<bucket_name>`/statedelta/`<testnet_name>` : This would contain the statedeltas
for constructing the state.
3. `<bucket_name>`/persistence : This would contain the persistence tars
used for recovery/back-up.

where `bucket_name` is the parameter specified and `testnet_name` is the name assigned
to the testnet.

## Release Bucket Configuration

For upgrading, you can specify the `--release-bucket-name` for choosing the bucket
to put the release files (.deb and other corresponding files) needed for upgrade.

Make sure the same bucket is being used in `release.sh` script in core repository.

*NOTE:* The default bucket is zilliqa-release-data.

## Configure Bastion to give access to s3 [OPTIONAL]

If the bastion is already transparently authenticated, then no need for this step

1. Go to zilliqa login page and login
2. Choose the role which has S3 access.
3. Choose `Command line or programmatic access.`
4. Copy keys and export into bastion

You may have to modify certain commands
(for example in [release.sh](https://github.com/Zilliqa/Zilliqa/blob/627caccb948e52a91f72384422692186d79e4fb3/scripts/release.sh#L291))
so the request sent is authenticated.

## Adding a policy to a bucket using command line

Useful when bastion has access to s3 transaprently but the user does
not have access to the s3 web console.

1. Login to the cloud9 bastion
2. `aws s3api put-bucket-policy --bucket <bucket-name> --policy policy.json` where policy.json might
   look like

   ```json
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
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": "arn:aws:s3:::zilliqa-devnet/*"
            }
        ]
    }
   ```

   This would make all the objects of `zilliqa-devnet` publicly readable.

3. Other command lines commands can be found at [S3 APIs list](https://docs.aws.amazon.com/cli/latest/reference/s3api/index.html#cli-aws-s3api)

## Appendix

### Bucket Policies

These ideally should be auto-updated while cluster creation.

#### Persistence bucket

For this bucket, we would need to give write, delete access to the nodes in the cluster.  
*Also due to a caveat in `downloadIncrDB.py` the bucket must be publicly readable.*  
The reason for this is that even if an internal node needs syncing from the s3 bucket, it
sends unauthorized HTTP requests.

Example, `zilliqa-devnet` bucket policy

``` json
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

#### Release Bucket

The bucket needs to be publicly readable if we are trying to upgrade community
(or public nodes) as well.

Otherwise read permission for the cluster nodes and write, delete permission
for the bastion node is required.

Example policy:

``` json
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
        {
            "Effect": "Allow",
            "Principal": {
                "AWS": "arn:aws:iam::273122647034:role/nodes.newton.dev.z7a.xyz.k8s.local"
            },
            "Action": [
                "s3:PutObject",
                "s3:DeleteObject",
                "s3:GetObject"
            ],
            "Resource": "arn:aws:s3:::zilliqa-test-release/*"
        }
    ]
}
```