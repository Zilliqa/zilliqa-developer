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

#### Do we have to upgrade the seed/ssn node after the Zilliqa 1 upgrade?

Almost every major upgrade of the Zilliqa blockchain requires all seed or SSN node operators to upgrade their nodes immediately after the upgrade. The Zilliqa team will continually update the node operators with the instructions to upgrade the nodes through Telegram/Discord channels.

#### Where can I find the configuration files to download?

The seed/ssn-specific configuration files are available on the [join page](https://mainnet-join.zilliqa.com/).

#### Where should I look for the instructions for running the node?

You can follow the node installation instructions in the [Getting Started](../exchanges/exchange-integration/getting-started/exchange-introduction.md). Note that the whitelisting of the node is only required for the new installation. For existing node installation, you can follow the `launch_docker.sh` instructions.
If you need to run more than one node, you can launch with the same whitelisted public/private keypair.
We highly recommend node operators use key whitelisting mode for running the seed/ssn node.

#### What additional infrastructure checks I should do before starting the node?

Please ensure the following:

- You must be using the public IP of the machine when following launch_docker.sh instructions.
- Kindly ensure that you are following the Minimum Hardware Requirements defined in the [Minimum Hardware requirements](../exchanges/exchange-integration/getting-started/exchange-introduction.md#minimum-hardware-requirements) section.

- The inbound port should be open in the firewall. You can check by issuing the following command. Note that you may need to install the netcat tool for this.

  ```sh
      nc -vz <IP address of the machine>  <port>
  ```

#### How do I check if the node is running fine?

Generally, a node downloads the complete persistence(blockchain data and state) at the start which can be time-consuming depending on the network bandwidth. It may take up to an hour. You can check the following.

- Check for ports 33133 - the application port, 4201 - the API port, or port 4501 - the staking port(In case of SSN) are visible in the netstat output through the command `netstat -tnlp|egrep "4201|4501|33133"`

- Additionally, you can query the following API to check if the API is returning the latest block number and confirm the same with the [view block URL](https://viewblock.io/zilliqa).

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
