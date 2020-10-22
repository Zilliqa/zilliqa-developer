# Mining Guide

## Contents

- [**General information for mining**](#general-information)
- [**Recommended hardware requirements**](#hardware-requirement)
- **Steps for mining on Zilliqa**
   - **Step 1:** [Initial setup required](#initial-setup)
   - **Step 2** (Option 1): [Setup for local mining with docker image](#mining-with-docker-locally-for-cpu-or-nvidia-gpus-only)
   - **Step 2** (Option 2): [Setup for remote mining with docker image](#mining-with-docker-remotely)
   - **Step 2** (Option 3): [Setup for local mining with native build](#mining-with-native-build-locally)
- [**Discussion channels**](#discussion-channels)

For the Chinese version (中文版) of these instructions, please visit [**HERE**](https://github.com/FireStack2018/Awesome-Zilliqa/blob/master/Documents/Mining/Zilliqa%E4%B8%BB%E7%BD%91%E6%8C%96%E7%9F%BF%E6%8C%87%E5%8D%97.md). (Credits to #Hash1024)

## General Information

### Epoch Architecture

![Zilliqa Epoch Architecture](https://i.imgur.com/Da4t6FW.png)

At the start of each DS Epoch, all mining candidates will run the Proof-of-Work (Ethash algorithm) cycle for a `60` seconds window in order to compete to join the Zilliqa network.

- Nodes that fulfilled the `DS_POW_DIFFICULTY` parameter will qualify to join as DS nodes.
- Nodes that fulfilled the `POW_DIFFICULTY` parameter will qualify to join as shard nodes.

There are a total of `100` TX epochs (each 1-2 min) within each DS Epoch (2-3 hrs). Every 100th TX epoch is known as the **Vacuous epoch**.

The vacuous epoch is solely for:

- Distributing the coinbase rewards to all nodes.
- Processing of the upgrade mechanism (as there are no forks in pBFT).
- Writing of persistent state storage (updating of the nodes’ levelDB).

During a vacuous epoch, the network does not process any transactions.

### Proof-of-Work algorithm

Zilliqa is using [**Ethash**](https://github.com/ethereum/wiki/wiki/Ethash) for its PoW algorithm. Hence, Zilliqa uses a DAG in its proof-of-work algorithm, that is generated at an incremental rate for each **DS epoch**. The bootstrap DAG size will be roughly `1.02GB`.

### Network Difficulty

The bootstrapped shard difficulty level for the mainnet is set at `10`. This difficulty level is dynamic and adjusts by `+/- 0.125` for every `+/- 99` deviation from the target `1810` PoW submissions per DS epoch. Maximum increase per DS epochs is `+/- 1`.

> **NOTE:** Difficulty level is the log2(Difficulty).

Say if there are `1810` seats available in the network:

* But there are `1909` PoW submissions, the shard difficulty level will increase by `0.125` for the next DS epoch.
* But there are `2602` PoW submissions, the shard difficulty level will increase by `1` for the next DS epoch.
* But there are `1711` PoW submissions, the shard difficulty level will decrease by `0.125` for the next DS epoch.
* But there are `1018` PoW submissions, the shard difficulty level will decrease by `1` for the next DS epoch.

### Reward Mechanism

In the Zilliqa network, rewards are split into:

* Base rewards **[25% of total]** for all validating nodes (DS/shard) in the network.
* Flexible rewards **[70% of total]** that are based on the amount of valid and accepted (first 2/3 signers within a shard) signatures submitted by a node during a TX epoch while doing the pBFT consensus.

Both base rewards and flexible rewards has the same weightage for both DS and shard nodes. All rewards are consolidated over an entire DS epoch and only distributed during the vacuous epoch. Do note that the last **5%** of the rewards are given to the lookup and seed nodes.

Say for example, if there are a total of `2400` nodes in the Zilliqa network and the `COINBASE_REWARD` is set at `197,244.577625571` ZILs per DS Epoch, the reward distribution will be:

- For Base rewards:
    ```shell
    197,244.577625571 * 0.25 / 2400
    = 20.5463101693 ZILs per node per DS Epoch
    ```
- For Flexible rewards: (on a first-come-first-serve basis)
    ```shell
    197,244.577625571 * 0.70 / (2,400 * 2/3 [Successful signers] * 99 [TX blocks])
    = 0.87166164354 ZILs per valid and accepted signature
    ```

> **NOTE:** Guard nodes by Zilliqa are not rewarded. However, the division of rewards before distribution does include the guard nodes in the count. Hence, there are no "bonus" rewards for non-guard nodes.

Find our your daily mining profitability by making a copy of the [**Reward Calculator**](https://docs.google.com/spreadsheets/d/1iA3DvXMiAql6bf1mGHHxfGLICm0wZ2Gav5HzRkP81j4/edit?usp=sharing) and editing the yellow-highlighted cells.

## Hardware requirement

The Zilliqa client is officially supported only on Ubuntu OS. 

Please follow the steps [**HERE**](https://itsfoss.com/install-ubuntu-1404-dual-boot-mode-windows-8-81-uefi/) if you wish to dual boot Windows and Ubuntu 16.04.

If you wish to mine using mining rigs that operate on Windows OS, please follow the remote mining with docker image guide [**HERE**](#mining-with-docker-remotely).

Both **AMD** (with OpenCL) and **Nvidia** (with OpenCL or CUDA) GPUs are supported for PoW.

The **minimum** requirements for Zilliqa mining nodes are:

- x64 Linux operating system (e.g Ubuntu 16.04.05)
- Recent dual core processor @ 2.2 GHZ. Examples:
   - Intel Core i5 or i7 (Skylake)
   - Intel Xeon (Skylake)
   - AMD Ryzen
- 4GB DRR3 RAM or higher
- NAT environment **OR** Public IP address
- Any GPUs with at least 2 GB vRAM

## Initial Setup

### Network setup

> **NOTE:** If you are using a home router, you are most probably in a NAT environment.

If you are in NAT environment, you can either:

- Do single port forwarding using **Option 1a**. This should be your **DEFAULT OPTION**.
- Enable UPnP mode using **Option 1b** if your router does support UPnP.

If you have a public IP address, you can skip this network setup entirely.

- **(Option 1a)** Port forward to port `33133` for both external port (port range) and internal port (local port). You will also have to select the option for **BOTH** TCP and UDP protocol in your router menu when port forwarding. <br><br> An example of this process can be found [**HERE**](https://www.linksys.com/us/support-article?articleNum=136711). After port forwarding, you may check if you have successfully port forwarded with this [**Open Port Check Tool**](https://www.yougetsignal.com/tools/open-ports/).

- **(Option 1b)** Enable UPnP mode on your home router. Please Google how to access your home router setting to enable UPnP, an example can be found [**HERE**](https://routerguide.net/how-to-enable-upnp-for-rt-ac66u/). You can check if you have successfully enabled UPnP by installing the following tool:
    ```shell
    sudo apt-get install miniupnpc
    ```
   Then type the following in the command line:
    ```shell
    upnpc -s
    ```
   You should get a message showing either:

     - "List of UPNP devices found on the network : ..."
     - **OR** "No IGD UPnP Device found on the network !".

   The first message means UPnP mode has been enabled successfully, while the latter means the enabling of UPnP mode has failed. If you receive the latter message, proceed with using **Option 1a** instead.

### OpenCL driver setup (for AMD/Nvidia)

If you wish to use OpenCL supported GPUs for PoW, please run the following to install the OpenCL developer package:

   ```shell
   sudo apt install ocl-icd-opencl-dev
   ```

You may need to reboot your PC for the installation to take effect. After reboot, check if your drivers are installed properly with the following command:

   ```shell
   clinfo
   ```
### CUDA driver setup (for Nvidia)

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from the [**NVIDIA official webpage**](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation to take effect.

### Multiple GPUs setup

If you wish to run multiple AMD or Nvidia GPUs concurrently, edit the `GPU_TO_USE` parameter in the _constants.xml_ file located in your _join_ folder.

The index start from `0` and you can input one or more multiple GPUs by separating their indexes with a `,`.

For example:

- `0` for just 1 GPU.
- `0, 1, 2` or `0, 2, 4` for 3 GPUs.

Do note that the largest index must correspond to the number of GPUs you have physically in your mining rig.

## Mining with docker locally (For CPU or Nvidia GPUs only)

1. Install Ubuntu 16.04.5 OS by following instructions [**HERE**](http://releases.ubuntu.com/xenial/).

2. Install Docker CE for Ubuntu by following instructions [**HERE**](https://docs.docker.com/install/linux/docker-ce/ubuntu/).

3. Make a new directory in your Desktop and change directory to it:

      ```shell
      cd ~/Desktop && mkdir join && cd join
      ```

4. Get the joining configuration files:

      ```shell
      wget https://mainnet-join.zilliqa.com/configuration.tar.gz
      tar zxvf configuration.tar.gz
      ```

5. Find out your current IP address in the command prompt and record it down.

   > **NOTE:** If you are using **Option 1b** as stated in the [**Network Setup**](#network-setup) above, you can skip this step.

      ```
      curl https://ipinfo.io/ip
      ```

6. Run the shell script in your command prompt to launch your docker image.

    - **(Option 1)** For mining with CPU, launch your docker container:

         ```shell
         ./launch_docker.sh
         ```
    - **(Option 2)** For mining with Nvidia GPUs, please first install the `nvidia-docker` [**HERE**](https://github.com/NVIDIA/nvidia-docker#ubuntu-140416041804-debian-jessiestretch). Then, change the following parameter in the _constants.xml_ file within the _join_ folder to `true`:
         ```shell
         <CUDA_GPU_MINE>true</CUDA_GPU_MINE>
         ```
         After which, launch your docker container:

         ```shell
         ./launch_docker.sh --cuda
         ```

       > **NOTE:** If you wish to run multiple Nvidia GPUs concurrently, you will need to modify your _**constants.xml**_ file following instructions as found above [**HERE**](#multiple-gpus-setup).

7. You will be prompted to enter some information as shown below:
    - `Assign a name to your container (default: zilliqa):` <br> [Press **Enter** to skip if using default]

    - `Enter your IP address ('NAT' or *.*.*.*):` <br> [Key in your IP address as found in step 6 **OR** `NAT` if you using Option 1b]

    - `Enter your listening port (default: 33133):` <br> [Press **Enter** to skip if using default]

       > **NOTE:** **DO NOT** duplicate your IP address and use different ports on different CPU nodes. You will be blacklisted by the network and hence not be able to receive any flexible rewards.

8. You are now a miner in the Zilliqa mainnet. You can monitor your progress using:

      ```shell
      tail -f zilliqa-00001-log.txt
      ```

9. To check your locally generated public and private key pairs, you can enter the following in your command prompt:

      ```shell
      less mykey.txt
      ```

    The first hex string is your **public key**, and the second hex string is your **private key**.

    > **NOTE:** This key pair is generated locally on your disk. Do remember to keep your private key somewhere safe!

10. To check your balance, get your address and input the address in the search bar of https://viewblock.io/zilliqa:

      ```shell
      less myaddr.txt
      ```

11. To stop the mining client, stop your running docker container:

      ```
      sudo docker ps (get zilliqa's CONTAINER_ID here)
      sudo docker stop zilliqa
      sudo docker rm CONTAINER_ID
      ```

## Mining with docker remotely

The setup architecture is illustrated in the image shown below. All communications between these two parties is via JSON-RPC protocol.

![1-to-many](https://i.imgur.com/qReRpRx.jpg)

- The CPU node instance will run the **Zilliqa client** and carry out the pBFT consensus process to receive rewards.
- The GPU rigs in the GPU cluster will run **Zilminer** on a separate GPU cluster to do PoW mining and provide PoW solutions directly to the CPU node.

For hooking up several GPU rigs in the GPU cluster to a single CPU node, you will be required to do the following steps:

***

1. Create a single local/remote CPU node instance with Ubuntu 16.04 OS installed following instructions [**HERE**](http://releases.ubuntu.com/xenial/).

2. Install Docker CE for Ubuntu on your CPU node instance by following instructions [**HERE**](https://docs.docker.com/install/linux/docker-ce/ubuntu/).

3. Make a new directory in your Desktop and change directory to it:

      ```shell
      cd ~/Desktop && mkdir join && cd join
      ```

4. Get the joining configuration files:

      ```shell
      wget https://mainnet-join.zilliqa.com/configuration.tar.gz
      tar zxvf configuration.tar.gz
      ```

5. Find out your current IP address in the command prompt and record it down:

      ```shell
      curl https://ipinfo.io/ip
      ```

6. Edit your _constant.xml_ file in your configuration folder:

    * Set `GETWORK_SERVER_MINE` to `true`.
    * Set `GETWORK_SERVER_PORT` to the port you will be using to GetWork. (default is `4202`)
    * Set the following mining parameters to `false`:

        ```shell
        <CUDA_GPU_MINE>false</CUDA_GPU_MINE>
        <FULL_DATASET_MINE>false</FULL_DATASET_MINE>
        <OPENCL_GPU_MINE>false</OPENCL_GPU_MINE>
        <REMOTE_MINE>false</REMOTE_MINE>
        ```

7. Run the shell script in your command prompt to launch your docker image.

      ```shell
      ./launch_docker.sh
      ```

8. You will be prompted to enter some information as shown below:
    - `Assign a name to your container (default: zilliqa):` <br> [Press **Enter** to skip if using default]

    - `Enter your IP address ('NAT' or *.*.*.*):` <br> [Key in your IP address as found in step 5 **OR** `NAT` if you chose Option 1b during Network setup]

    - `Enter your listening port (default: 33133):` <br> [Press **Enter** to skip if using default]

       > **NOTE:** **DO NOT** duplicate your IP address and use different ports on different CPU nodes. You will be blacklisted by the network and hence not be able to receive any flexible rewards.

9. Once the CPU Zilliqa node is running, you can install **Zilminer** on your separate GPU rigs:

    - **For Windows OS:** [**DOWNLOAD THE LASTEST RELEASE HERE**](https://github.com/DurianStallSingapore/ZILMiner/releases/)
    - **For Ubuntu OS:** [**DOWNLOAD THE LASTEST RELEASE HERE**](https://github.com/DurianStallSingapore/ZILMiner/releases/)

10. Setup your **Zilminer** on your separate GPU rigs with the following command:

    ```shell
    zilminer -P zil://wallet_address.worker_name@zil_node_ip:get_work_port
    ```

    > **NOTE:** You have to change the *wallet_address*, *worker_name*, *zil_node_ip*, and *get_work_port* accordingly.

    - For `wallet_address` : You can input any arbitrary Zilliqa address if you are mining solo.
    - For `worker_name` : You can input any arbitrary worker name you desire.
    - For `zil_node_ip` : Please input the IP address of the Zilliqa node in step 5.
    - For `get_work_port` : Please input the port used in `GETWORK_SERVER_PORT`. Default is `4202`.

11. You are now a proxy miner in the Zilliqa mainnet. You can monitor your progress on your CPU node using:

      ```shell
      tail -f zilliqa-00001-log.txt
      ```

12. To check your locally generated public and private key pairs, you can enter the following in your command prompt on your CPU node:

      ```shell
      less mykey.txt
      ```

    The first hex string is your **public key**, and the second hex string is your **private key**.

    > **NOTE:** This key pair is generated locally on your disk. Do remember to keep your private key somewhere safe!

13. To check your balance, get your address and input the address in the search bar of https://viewblock.io/zilliqa:

      ```shell
      less myaddr.txt
      ```

14. To stop the mining client, stop your running docker container on the CPU node and kill your **Zilminer** process on your GPU rigs:

      ```
      sudo docker stop zilliqa
      ```

## Mining with native build locally

1. Make a new directory for Zilliqa client:

      ```shell
      cd ~/Desktop && mkdir Zilliqa
      ```

2. Make a new directory for Scilla binary:

      ```shell
      mkdir Scilla
      ```

3. Make a new directory for the join folder:

      ```shell
      mkdir join
      ```

4. Clone the Scilla repository and change directory to it:

      ```shell
      git clone https://github.com/Zilliqa/Scilla.git Scilla && cd Scilla && git checkout v0.7.0
      ```

5. Find out your Scilla directory path and record it down:

      ```shell
      pwd
      ```

6. First, download the Scilla binary's dependencies for Ubuntu following instructions found [**HERE**](https://github.com/Zilliqa/scilla/blob/master/INSTALL.md#ubuntu).

7. Then, build the Scilla binary:

      ```shell
      make clean; make
      ```

8. Clone the Zilliqa repository and change directory to it:

      ```
      cd ~/Desktop && git clone https://github.com/Zilliqa/Zilliqa.git Zilliqa && cd Zilliqa && git checkout v6.4.2
      ```

9. Find out your Zilliqa directory path again and write it down:

      ```
      pwd
      ```

10. First, download the Zilliqa client's dependencies. Then, build Zilliqa with **Option 1** for CPU mining, or with **Option 2**/**Option 3** for GPU mining.

    - Download the dependencies:

        ```shell
        sudo apt-get update
        sudo apt-get install git libboost-system-dev libboost-filesystem-dev libboost-test-dev \
        libssl-dev libleveldb-dev libjsoncpp-dev libsnappy-dev cmake libmicrohttpd-dev \
        libjsonrpccpp-dev build-essential pkg-config libevent-dev libminiupnpc-dev \
        libprotobuf-dev protobuf-compiler libcurl4-openssl-dev libboost-program-options-dev \
        libssl-dev python3-dev libboost-python-dev python3-setuptools python3-pip gawk
        ```

    - **(Option 1)** Build Zilliqa for CPU mining:

       ```shell
       ./build.sh
       ```
    - **(Option 2)** Build Zilliqa for Nvidia GPU mining with CUDA support:

       ```shell
       ./build.sh cuda
       ```
    - **(Option 3)** Build Zilliqa for AMD or Nvidia GPU mining with OpenCL support:

       ```shell
       ./build.sh opencl
       ```

11. Download and unpack the compressed joining configuration file:

    ```shell
    cd ../join && wget https://mainnet-join.zilliqa.com/configuration.tar.gz && tar zxvf configuration.tar.gz
    ```
12. Edit the _constants.xml_ in your _join_ folder to key in the Scilla directory for the `SCILLA_ROOT` parameter. An example is shown below:

    ```shell
    <SCILLA_ROOT>/home/ubuntu/Scilla</SCILLA_ROOT>
    ```

13. **(Optional)** If you wish to mine with GPUs, please continue to edit the following parameters in the _constants.xml_ file in your _join_ folder:

    > **NOTE:** If you wish to run multiple GPUs concurrently, you will need to modify your _**constants.xml**_ file following instructions as found above [**HERE**](#multiple-gpus-setup).

    - **For AMD GPUs:** Change `FULL_DATASET_MINE` parameter from `false` to  `true`. Change `OPENCL_GPU_MINE` parameter from `false` to `true`.
    - **For Nvidia GPUs:** Change `FULL_DATASET_MINE` parameter from `false` to  `true`. Change `CUDA_GPU_MINE` parameter from `false` to `true`.


14. Find out your current IP address in the command prompt and record it down.

    > **NOTE:** If you are using **Option 1b** as stated in the [**Network Setup**](#network-setup) above, you can skip this step.

    ```shell
    curl https://ipinfo.io/ip
    ```

15. Install python dependencies:
    ```shell
    sudo apt install python-pip
    export LC_ALL=C
    pip install request requests clint futures
    ```

16. Launch the Zilliqa client:

    ```shell
    ./launch.sh
    ```

17. You will be prompted to key in the following details:
    - `Enter the full path of your zilliqa source code directory:` <br> [Key in the path you found it step 8]
    - `Enter your IP address (NAT or *.*.*.*):` <br> [Key in your IP address as found in step 13 **OR** `NAT` if you are using Option 1b]
    - `Enter your listening port (default: 33133):` <br> [Press **Enter** to skip if using default]

       > **NOTE:** **DO NOT** duplicate your IP address and use different ports on different CPU nodes. You will be blacklisted by the network and hence not be able to receive any flexible rewards.

18. You are now a miner in Zilliqa mainnet. You can monitor your progress using:
    ```shell
    tail -f zilliqa-00001-log.txt
    ```

19. To check your locally generated public and private key pairs, you can enter this in your command prompt:

    ```shell
    less mykey.txt
    ```
    The first hex string is your **public key**, and the second hex string is your **private key**.

    > **NOTE:** The key pair is generated locally on your disk. Do remember to keep your private key somewhere safe!

20. To check your balance, get your address and input the address in the search bar of https://viewblock.io/zilliqa:

      ```shell
      less myaddr.txt
      ```

21. To stop Zilliqa client:

    ```shell
    pkill zilliqa
    ```

## Discussion channels

### Channels

Join our official Discord channel: https://discord.gg/XMRE9tt
Join the community-managed Telegram channel: https://t.me/zilliqaminer
