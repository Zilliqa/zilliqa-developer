---
id: faq-seed-ssn
title: Frequently Asked Questions
keywords:
  - FAQ
  - Questions
  - Seed
  - SSN
description: Frequently asked questions
---

## Frequently asked questions

<!-- markdownlint-disable MD001 -->

#### Do I need to upgrade the seed/ssn node after the Zilliqa upgrade?

In almost every upgrade of zilliqa blockchain requires all seed or SSN node operators to upgrade their nodes immediately after the upgrade. The zilliqa team will always update the node operators with the instructions to upgrade the nodes through Telegram/Discord.

#### Where can I find the configuration files to download?

The seed/ssn specific configuration files are available in the [join page](https://mainnet-join.zilliqa.com/).

#### Where should i look for the instructions for running the node ?

You can follow the node installation instructions given in the [exchange section](http://localhost/exchanges/exchange-integration/getting-started/exchange-introduction/).
We highly recommend node operators to use key whitelisting mode for running the seed/ssn node.

#### What additional things i should check before starting the node ?

Please ensure the following.
You must be using the public IP of the machine when following launch_docker.sh instructions.
The IP inbound port should be open in firewall. You can check by issuing the following command.

```sh
nc -vz <IP address of the machine>  <port>
```

#### How do i check if the node is running fine ?

Generally node downloads the complete persistence(blockchain data and state) on the start which can be timeconsuming depending on the network bandwidth. It may take upto an hour. After the successful start, node will open 4201 API port. You can check the following.
netstat -tnlp|grep 4201

Additionally you can query the following API to check if the API is returning the latest block number and confirm the same with the [view block URL](https://viewblock.io/zilliqa).

```sh
curl --request POST \
  --url http://localhost:4201/ \
  --header 'Content-Type: application/json' \
  --data '{
    "id": "1",
    "jsonrpc": "2.0",
    "method": "GetLatestTxBlock",
    "params": [""]
}'
```
