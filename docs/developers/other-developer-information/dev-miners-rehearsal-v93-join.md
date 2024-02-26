---
id: dev-miners-rehearsal-v93-join
title: Miners Rehearsal network v9.3 joinin procedure
keywords:
  - miners
  - v9.3.0
  - rehearsal
  - join
description: How to join the v9.3.0 miners' rehearsal network
---

---

## Introduction

This is page collects the steps to configure a `Zilclient` mining node and join the `v930-miners` rehearsal network deployed to test the mining facility of the desharded Zilliqa version 1 network.

The procedure to deploy a mining node for the Zilliqa Mainnet is available at the following [link](https://dev.zilliqa.com/miners/mining/mining-zilclient/).

## Requirements

The [**Zilliqa Client**](https://github.com/Zilliqa/zilliqa) is officially supported on Ubuntu 22.04 OS.

The **minimum** requirements for running the **Zilliqa Client** are:

- Ubuntu Linux 22.04
- Recent dual-core processor @ 2.2 GHZ. Examples: Intel Xeon (Skylake)
- 8GB DRR3 RAM or higher
- Public static IP address
- 300GB Solid State Drive
- 100MB/s upload and download bandwidth
- Docker version 24+

## Joining steps

After you have your Ubuntu Linux up and running with Docker installed (you can install docker following the steps [here](https://docs.docker.com/install/linux/docker-ce/ubuntu/)), you can now bootstrap your mining node using the below procedure.

### 1. Create a working directory for your Zilclient node

```bash
mkdir v930-miners-join
```

### 2. Get the joining configuration

Get the joining configuration for our mining rehearsal network and extract it in the previous created directory.

```bash
cd v930-miners-join && \
curl -L https://v930-join.miners-rehearsal.zq1.network/configuration.tar.gz | tar xzf -
```

### 3. Find out your current IP address in the command prompt and record it down

```bash
curl ipconfig.io
```

!!! note

    NAT IP is not supported. Kindly use the public IP address during the
    launch step.

### 4. Run the shell script in your command prompt to launch your docker image

```bash
./launch_docker.sh
```

### 5. You will be prompted to enter some information as shown below:

    - `Assign a name to your container (default: zilliqa):` <br/> [Press
      **Enter** to skip if using default]

    - `Enter your IP address (*.*.*.*):` <br/> [Key in your IP address as
      found in step 5]

    - `Enter your listening port (default: 33133):` <br/> [Press **Enter** to
      skip if using default]

### 6. Monitoring Progress

You are now a miner in the Zilliqa Mainnet. You can monitor your progress on your CPU node by using:

```shell
tail -f zilliqa.log
```

### 7. Stop the mining node

```shell
sudo docker stop <zilliqa container name>
```
