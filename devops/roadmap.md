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
  - [Testnet](#testnet)
    - [Testnet Infrastructure](#testnet-infrastructure)
  - [Mainnet](#mainnet)
- [Cheatsheet](#cheatsheet)
- [References](#references)

## Background

Prior to this transition, all the Zilliqa networks were running in one AWS account and managed through two bastion hosts running from the same account. As a result, many of the AWS resources were shared between different Kubernetes clusters and networks, which created security challenges in complete isolation and access control.

### Bastions

In the previous operations and also as of now, two EC2 instances were acting as bastions and basic isolation between testnets and mainnet were established. The full documentation can be found [here][1].

#### `kops.z7a.xyz`

**Notice: This bastion is terminated. Please use Cloud9 bastion instead.**

This bastion was used to manage all the testnets, including the ones for developers, community and partners. 

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

### Testnet

This category includes all the networks used by real users from outside for long-term testing and evaluation, such as:

- public testnets for community
- private testnets for partners

These testnets are important as it contains user data and sometime has to satisfy the partners' security requirements (e.g., isolated infrastucture).

#### Testnet Infrastructure

Each testnet needs to run in different AWS accounts with Cloud9 bastion used. However for the existing ones, we will not touch them until a clean migration is ready.

| Network           | Users/Partners | Bastion | Migration Status |
|-------------------|----------------|---------|------------------|
| community testnet | community      | Cloud9  | done             |
| blue testnet      | mindshare      | Cloud9  | done             |

### Mainnet

The mainnet will still run in the current infrastructure and will be managed through [`mkops.z7a.xyz` bastion](#mkopsz7axyz).

## Cheatsheet

| Use case             | Bastion                                             | Bootstrap HTTPS options              |
|----------------------|-----------------------------------------------------|--------------------------------------|
| new devnet cluster*  | Cloud9 bastion in devnet AWS account                | See README.md in side Cloud9 bastion |
| new devnet*          | Any user-owned Cloud9 bastion in devnet AWS account | See README.md in side Cloud9 bastion |
| new testnet cluster* | Cloud9 bastion in a dedicated testnet AWS account   | See README.md in side Cloud9 bastion |
| new testnet*         | Same Cloud9 bastion where the cluster is launched   | See README.md in side Cloud9 bastion |
| mainnet cluster      | [`mkops.z7a.xyz` bastion](#mkopsz7axyz)             | `--https=aws.zilliqa.com`            |
| mainnet              | [`mkops.z7a.xyz` bastion](#mkopsz7axyz)             | `--https=aws.zilliqa.com`            |

> *: for any S3 related features, please specify the bucket with --bucket and make sure policies are coherent. See **[Bucket Policies](https://github.com/Zilliqa/dev-docs/blob/master/devops/bucket-separation.md)** for more.

## References

- [Zilliqa/testnet](https://github.com/Zilliqa/testnet)
- [Zilliqa/genet](https://github.com/Zilliqa/genet)
- [Kops Bastion Host User Guide][1]
- [AWS Cloud9 Bastion](aws-bastion.md)

[1]: https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit#heading=h.jc0npl4cfb8u
