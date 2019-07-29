# Zillqa Network DevOps Roadmap

We are making some changes to the infrastructure and workflows in order to have more isolation and access control.

- [Background](#background)
  - [Bastions](#bastions)
    - [`kops.z7a.xyz`](#kopsz7axyz)
    - [`mkops.z7a.xyz`](#mkopsz7axyz)
  - [Toolings](#toolings)
  - [Challenges and Solutions](#challenges-and-solutions)
- [Goals](#goals)
- [Zilliqa Network Category](#zilliqa-network-category)
  - [Devnet](#devnet)
    - [Devnet Infrastructure](#devnet-infrastructure)
    - [Devnet Known Limitations](#devnet-known-limitations)
  - [Testnet](#testnet)
    - [Testnet Infrastructure](#testnet-infrastructure)
    - [Testnet Known Limitaitons](#testnet-known-limitaitons)
  - [Mainnet](#mainnet)
- [Cheatsheet](#cheatsheet)
- [References](#references)

## Background

Prior to this transition, all the Zilliqa networks were running in one AWS account and managed through two bastion hosts running from the same account. As a result, many of the AWS resources were shared between different Kubernetes clusters and networks, which created security challenges in complete isolation and access control.

### Bastions

In the previous operations and also as of now, two EC2 instances were acting as bastions and basic isolation between testnets and mainnet were established. The full documentation can be found [here][1].

#### `kops.z7a.xyz`

This bastion is used to manage all the testnets, including the ones for developers, community and partners.

Currently the running networks and clusters are

| Network                         | Cluster              |
|---------------------------------|----------------------|
| all developer-launched testnets | `dev.k8s.z7a.xyz`    |
| community testnet               | `dev.k8s.z7a.xyz`    |
| proton testnet                  | `proton.k8s.z7a.xyz` |

#### `mkops.z7a.xyz`

This bastion is used to manage mainnet only. Find more information by accessing the basion directly.

### Toolings

The boostrap tools in the testnet repository <https://github.com/Zilliqa/testnet> has been in use for a long time. However, several key features are missing so the _next-generation_ version is developed in <https://github.com/Zilliqa/genet>. The testnet repository now is feature freeze and will not accept major breaking changes and refactorings.

### Challenges and Solutions

| # | Challenges                                                                                     | Solutions                                                                                   |
|---|------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------|
| 1 | IP whitelisting of allowed users is troublesome                                                | [Use Cloud9 bastion](https://github.com/Zilliqa/dev-docs/blob/master/devops/aws-bastion.md) |
| 2 | [Using same bucket across networks is not good](https://github.com/Zilliqa/testnet/issues/512) | make the bucket location configurable                                                       |
| 3 | Isolation between dev/test/prod is missing                                                     | Run different networks in different AWS account                                             |

## Goals

1. Migrate the running networks from the current AWS account to multiple new AWS account.
2. Upgrade / Migrate the existing bastions to Cloud9 and adopt more Cloud9-based DevOps.
3. Make proper plan to ensure a smooth and stable transition.

## Zilliqa Network Category

There will be three categories of the network in the future: devnet, testnet, and mainnet. Their main differences are summarized below.

|                     | Devnet               | Testnet                 | Mainnet                 |
|---------------------|----------------------|-------------------------|-------------------------|
| Definition          | Development network  | Testing network         | Main network            |
| Quantity            | Many                 | Many                    | One                     |
| Public/Private      | Private              | Public / Private        | Public                  |
| No. of AWS accounts | One AWS account      | Many AWS accounts       | One AWS account         |
| No. of K8s clusters | One or more clusters | One cluster per account | One cluster per account |

### Devnet

The devnet is a new category for the network used in day-to-day development.

#### Devnet Infrastructure

In the new design, all the devnets will run in one cluster from a selected AWS account. However, there are exceptions such as the devnet for testing network upgrade at large scale. This one has to run in a separate cluster regardless of which account is used.

#### Devnet Known Limitations (DEPRECATED)  

This issue is resolved and one can specify the bucket for recovery/upgrade/persistence using `--bucket` option during bootstrapping.  

1. Due to the blocking [issue regarding S3](https://github.com/Zilliqa/testnet/issues/512), the following S3 buckets in mainnet AWS acount are not writeable from any other AWS accounts:
    - `s3://zilliqa-persistence`
    - `s3://zilliqa-incremental`
    - `s3://zilliqa-statedelta`
    - etc.

    As a result, the devnet / testnet running in different AWS account will lose the recovery features unless the issue is resolved permanently or the access is enabled temporarily during the recovery. Also, new node joining is unsupported as the persistence data cannot be uploaded to the S3 bucket.

### Testnet

This category includes all the networks used by real users from outside for long-term testing and evaluation, such as:

- public testnets for community
- private testnets for partners

These testnets are important as it contains user data and sometime has to satisfy the partners' security requirements (e.g., isolated infrastucture).

#### Testnet Infrastructure

Each testnet needs to run in different AWS accounts with Cloud9 bastion used. However for the existing ones, we will not touch them until a clean migration is ready.

| Network           | Users/Partners | Bastion                     | Migration Status |
|-------------------|----------------|-----------------------------|------------------|
| community testnet | community      | [kops.z7a.xyz](#kopsz7axyz) | pending*         |
| proton testnet    | mindshare      | [kops.z7a.xyz](#kopsz7axyz) | pending*         |
| blue testnet      | mindshare      | Cloud9                      | done             |

> *: The existing testnets will stay unchanged until the existing issues are resolved.

#### Testnet Known Limitaitons

1. The same blocking issue as per [Devnet Known Limitations #1](#devnet-known-limitations).

### Mainnet

The mainnet will still run in the current infrastructure and will be managed through [`mkops.z7a.xyz` bastion](#mkopsz7axyz).

## Cheatsheet

| Use case                                   | Bastion                                             | Bootstrap HTTPS options                                  |
|--------------------------------------------|-----------------------------------------------------|----------------------------------------------------------|
| new devnet cluster                         | Cloud9 bastion in devnet AWS account                | `--https=dev.z7a.xyz`                                    |
| new devnet*                                | Any user-owned Cloud9 bastion in devnet AWS account | `--https=dev.z7a.xyz`                                    |
| new testnet cluster                        | Cloud9 bastion in a dedicated testnet AWS account   | `--https=any.custom.domain --https-profile=profile.yaml` |
| new testnet*                               | Same Cloud9 bastion where the cluster is launched   | `--https=any.custom.domain --https-profile=profile.yaml` |
| mainnet cluster                            | [`mkops.z7a.xyz` bastion](#mkopsz7axyz)             | `--https=aws.zilliqa.com`                                |
| mainnet                                    | [`mkops.z7a.xyz` bastion](#mkopsz7axyz)             | `--https=aws.zilliqa.com`                                |

> *: for any S3 related features, please specify the bucket with --bucket and make sure policies are coherent. [Bucket policies](https://github.com/Zilliqa/dev-docs/blob/master/devops/bucket-separation.md)

## References

- [Zilliqa/testnet](https://github.com/Zilliqa/testnet)
- [Zilliqa/genet](https://github.com/Zilliqa/genet)
- [Kops Bastion Host User Guide][1]
- [AWS Cloud9 Bastion](aws-bastion.md)

[1]: https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit#heading=h.jc0npl4cfb8u
