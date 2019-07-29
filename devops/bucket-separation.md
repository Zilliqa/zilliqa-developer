# AWS S3 Bucket Configuration

- [Introduction](#introduction)
- [Authenticated Access](#authenticated-access)
- [Unauthenticated Access](#unauthenticated-access)
  - [Configuring Bucket Policy in Command Line](#configuring-bucket-policy-in-command-line)
- [References](#references)

## Introduction

Zilliqa network uses AWS Simple Storage Service (S3) to achieve high-volume data transfer, distribution and persistence for blockchain-related data that includes persistence data, snapshot, state deltas, logs and release package.

The following tables shows the storage locations for different objects and their permissions for our nodes running on AWS (through *authenticated access*) or unlimited access from any nodes (through *unauthenticated access*). More details can be found in this doc.

| Location Pattern                                  | Authenticated Access | Unauthenticated Access | Usage                                                               |
|---------------------------------------------------|----------------------|------------------------|---------------------------------------------------------------------|
| `s3://<bucket_name>/incremental/<network_name>/*` | R/W                  | R/-                    | persistence snapshot per 10 ds epoch, for joining/rejoining purpose |
| `s3://<bucket_name>/statedelta/<network_name>/*`  | R/W                  | R/-                    | state deltas for constructing the state                             |
| `s3://<bucket_name>/persistence/*`                | R/W                  | -/-                    | persistence tarballs used for recovery/back-up                      |
| `s3://<bucket_name>/logs/<network_name>/*`        | R/W                  | -/-                    | logs for each node in the network                                   |
| `s3://<bucket_name>/release/*`                    | R/W                  | -/-                    | release tarballs                                                    |

> Note:
>
> 1. This pattern table describes the latest development on `master` branch in <https://github.com/Zilliqa/testnet>.
> 2. `<bucket_name>` is the bucket name used in `bootstrap.py --bucket=<bucket_name>`.
> 3. `<network_name>` is the network name used in `bootstrap.py <network_name>`.
> 4. Authenticated access is automatically configured during cluster creation.
> 5. Unauthenticated access configuration requires manual effort as described at **[Configuring Bucket Policy in Command Line](#configuring-bucket-policy-in-command-line)**
> 6. `R/W` means read and write. `R/-` means read-only. `-/-` means no access allowed.

## Authenticated Access

The authenticated access is automatically configured during cluster creation. The IAM role of the EC2 instances that run Zilliqa nodes will have additional policy configured. An example of the policy document is here:

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            ... // other non-S3 policies
        },
        {
            "Effect": "Allow",
            "Action": [
                "s3:ListBucket"
            ],
            "Resource": [
                "arn:aws:s3:::301978b4-****-****-****-3a2f63c5182c"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "s3:*Object"
            ],
            "Resource": [
                "arn:aws:s3:::301978b4-****-****-****-3a2f63c5182c/*"
            ]
        }
    ]
}
```

The requests from these instances will be trasparently authenticated using the IAM role of instances, so the nodes will be allowed to access.

## Unauthenticated Access

The unauthenticated access, or public access, is configured manually [using AWS commandline tools](#configuring-bucket-policy-in-commandline). This involves creating a bucket policy document like this:

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:ListBucket",
            "Condition": {
                "StringLike": {
                    "s3:prefix": [
                        "incremental/*",
                        "statedelta/*"
                    ]
                }
            },
            "Resource": "arn:aws:s3:::301978b4-****-****-****-3a2f63c5182c"
        },
        {
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:GetObject",
            "Resource": [
                "arn:aws:s3:::301978b4-****-****-****-3a2f63c5182c/incremental/*",
                "arn:aws:s3:::301978b4-****-****-****-3a2f63c5182c/statedelta/*"
            ]
        }
    ]
}
```

> Note: This policy document is meant to be configured for a specific S3 bucket in its bucket policy, whereas the policy document in **[Authenticated Access](#authenticated-access)** is meant to be used as IAM policy.

The use of `"Principal": "*"` in the policy allows public access and the condition and resource with prefix limits the access to certain prefixes (or folders).

### Configuring Bucket Policy in Command Line

Make sure you have admin access to the bucket `<bucket-name>`. If you are inside the Cloud9 bastion with S3 admin access to `<bucket-name>`, you can directly invoke the following commands. Otherwise, please configured the AWS credentials in command line first.

```bash
aws s3api put-bucket-policy --bucket <bucket-name> --policy policy.json
```

The file `policy.json` should contain a valid bucket policy document as above. Do remember to replace the example bucket name string from `301978b4-****-****-****-3a2f63c5182c` to the one you are configuring (i.e., `<bucket-name>`).

Other commands under `aws s3api` can be found at [AWS documentation](https://docs.aws.amazon.com/cli/latest/reference/s3api/index.html#cli-aws-s3api).

## References

- [Writing IAM Policies: Grant Access to User-Specific Folders in an Amazon S3 Bucket](https://aws.amazon.com/blogs/security/writing-iam-policies-grant-access-to-user-specific-folders-in-an-amazon-s3-bucket/)