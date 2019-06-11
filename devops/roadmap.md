# DevOps Roadmap

We are making the transition in the infrastructure with more isolation and access control.

- [Zilliqa Network Category](#zilliqa-network-category)
  - [Devnet](#devnet)
  - [Testnet](#testnet)
  - [Mainnet](#mainnet)
- [Other Resources](#other-resources)

## Zilliqa Network Category

There are three categories of the network: devnet, testnet, and mainnet. Their main differences are summarized below.

|                | Devnet              | Testnet          | Mainnet             |
|----------------|---------------------|------------------|---------------------|
| Definition     | Development network | Testing network  | Main network        |
| Quantity       | Many                | Many             | One                 |
| Public/Private | Private             | Public / Private | Public              |
| Bastion        | Cloud9              | Cloud9 / `kops`  | `mkops`             |
| major HTTPS*   | `*.dev.z7a.xyz`     | `*.aws.z7a.xyz`  | `*.aws.zilliqa.com` |

> *: This refers to the major HTTPS used for the network URLs. However, sometimes other HTTPS may also be used.

### Devnet

The devnet is a new category for the network used in day-to-day development.

#### Devnet Infrastructure

By default, all the devnets will in one cluster from one AWS account. However, there are exceptions such as the devnet for testing network upgrade at large scale. This one has to run in a separate cluster in the same account.

#### Devnet Known Limitations

1. Due to the blocking issue (Zilliqa/testnet#512) regarding S3, the following S3 buckets in mainnet AWS acount are not writeable from any other AWS accounts:
    - `s3://zilliqa-persistence`
    - `s3://zilliqa-incremental`
    - `s3://zilliqa-statedelta`
    - etc.

    As a result, the devnet / testnet running in different AWS account will lose the recovery features unless the issue is resolved permanently or the access is enabled temporarily during the recovery.

### Testnet

This category includes all the networks used by real users from outside for long-term testing and evaluation, such as:

- public testnets for community
- private testnets for partners

These testnets are important as it contains user data and sometime has to satisfy the partners' security requirements (e.g., isolated infrastucture).

#### Testnet Infrastructure

Every testnet needs to run in different AWS accounts with Cloud9 bastion used. To reduce the impact, we will only apply this rule to the new testnets.

| Network           | Users/Partners | Bastion      | Migration Status |
|-------------------|----------------|--------------|------------------|
| community testnet | community      | kops.z7a.xyz | pending*         |
| proton testnet    | mindshare      | kops.z7a.xyz | pending*         |
| blue testnet      | mindshare      | Cloud9       | done             |

> *: The existing testnets will stay unchanged until the blocking issue (Zilliqa/testnet#512) is resolved.

#### Testnet Known Limitaitons

1. The same blocking issue (Zilliqa/testnet#512) as per [Devnet Known Limitations #1](#devnet-known-limitations).

### Mainnet

The mainnet will still run in the current infrastructure and will be managed through `mkops.z7a.xyz`. The only thing we might need to "migrate" is converting the existing bastion to Cloud9.

## Other Resources

- Code Repositories
  - [Zilliqa/testnet](https://github.com/Zilliqa/testnet)
  - [Zilliqa/genet](https://github.com/Zilliqa/genet)
- Google Docs
  - [Kops Bastion Host User Guide](https://docs.google.com/document/d/1SMnflWGmGQGc3qJOOlGtq-85eBYuyQUg1fjkZlcSIKo/edit#heading=h.jc0npl4cfb8u)