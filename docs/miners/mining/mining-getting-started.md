---
id: mining-getting-started
title: Getting Started
keywords:
  - proxy mining
  - participating as miner
  - mining pools
  - zilliqa
description: Mining Getting Started
---

---

## Participating as a Miner

There are theoretically multiple ways to participate in the Zilliqa Mainnet as a
miner.

1. Operating a mining setup to support a pool of 300+ modern GPU's
2. Participating in mining pools

!!! note

    To enter the shard network requires >300 modern GPUs. As such, it will be
    advisable to participate in a mining pool if you do not have a sufficient amount
    of GPUs.

    If you have less than 300 modern GPU's your nodes will not be selected. You should
    join an existing pool and contribute hashpower there.

### Participation in existing mining pools

The Zilliqa page on [MiningPoolStats](https://miningpoolstats.stream/zilliqa)
has a list of mining pools that support Zilliqa. Please refer to the pool's
website for specific instructions on how to participate.

### Setting up Zilliqa Miner infrastructure

If you have less than 300 modern GPU's your nodes will not be selected. You should join an existing pool and contribute hashpower there.

To proceed with proxy mining, you'll need the following services:

#### Zilliqa Client

[Zilliqa Client repository](https://github.com/Zilliqa/Zilliqa). The Zilliqa Client instructions can be found [here](mining-zilclient.md).

A CPU node instance will run the **Zilliqa Client** and carry out the pBFT consensus process to receive rewards. It forwards messages to the GPU cluster via the proxy. It recieves a response which is the GPU's solution to the PoW round which is forwarded onto the chain for reward distribution.

The Zilliqa client is managed by Zilliqa. Though the GPU cluster and proxy could be bespoke client applications.

#### GPU cluster

The GPU cluster will do PoW mining and provide solutions back to the Zilliqa Client via the proxy.

The GPU cluster could be an existing mining protocol such as Claymore, CGMiner, BFGMiner.

Zilliqa have a GPU cluster, but it is not as popular with mining pools already intergrating with Zilliqa since they will typically use one of the above existing mining clients. The ZilMiner instructions can be found [here](mining-zilminer.md).

#### Mining proxy

The [Zilliqa Mining Proxy](https://github.com/DurianStallSingapore/Zilliqa-Mining-Proxy) provides an abstraction between the Zilliqa client and the GPU's. It forwards messages between the internal IPs of the two services.
