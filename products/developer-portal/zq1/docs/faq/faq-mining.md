---
id: faq-mining
title: Frequently Asked Questions
keywords:
  - FAQ
  - Questions
  - Mining
description: Frequently asked questions
---

## Frequently asked questions

<!-- markdownlint-disable MD001 -->

#### Do we have to upgrade the mining node after the Zilliqa 1 upgrade?

Almost every major upgrade of the Zilliqa blockchain requires the miners to upgrade their nodes immediately after the upgrade. The Zilliqa team will continually inform the node operators with the instructions to upgrade the nodes through Telegram/Discord channels.

#### Where can I find the configuration files to download?

The configuration files are available in the [join page](https://mainnet-join.zilliqa.com/) in the `configuration.tar.gz` file.

#### Where should I look for the instructions for running the node?

You can follow the node installation instructions in the [Zilclient](../miners/mining/mining-zilclient.md) section.

#### What additional infrastructure checks I should do before starting the node?

Please ensure the following.

- Follow the [Hardware Requirements section](../miners/mining/mining-zilclient.md#hardware-requirements) section.
- You must be using the public IP of the machine when following launch_docker.sh instructions.
- The inbound port on 33133 should be open in the firewall. You can check by issuing the following command from any machine. Note that you may need to install the netcat tool for this.

  ```sh
      nc -vz <IP address of the machine>  <port>
  ```

#### How do I check if the node is running fine?

Generally, a node downloads the complete persistence(blockchain data and state) at the start which can be time-consuming depending on the network bandwidth. It may take up to an hour. After the successful start, you should notice that port 33133 is opened by the application by checking netstat output through command `netstat -tnlp|grep 33133`.
