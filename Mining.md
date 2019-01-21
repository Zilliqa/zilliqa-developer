# Mining Guide

Welcome to Zilliqa testnet-v3, code-named _Mao Shan Wang_. We are inviting all miners to test out the process of joining as a public node on _Mao Shan Wang_ testnet. We hope this exercise will familiarise everyone with the workflow and also help to find out potential bugs before the mainnet launch by end January 2019. We also encourage all community developers to join the _Mao Shan Wang_ testnet in order to better understand the architecture of the Zilliqa network.

- [General information for mining](#general-information)
- [Recommended hardware requirements](#hardware-requirement-for-mao-shan-wang-testnet)
- [Initial Setup required](#initial-setup)
- [Steps for mining with docker image](#steps-for-mining-with-docker-for-cpu-or-nvidia-gpus-only)
- [Steps for mining with native build](#steps-for-mining-natively)
- [Discussion channels](#discussion-channels)

For the Chinese version(中文版) of these instructions, please visit [**HERE**](https://github.com/FireStack2018/Awesome-Zilliqa/blob/master/Documents/Mining/Zilliqa%E6%8C%96%E7%9F%BF%E6%8C%87%E5%8D%97.md). (Credits to #Hash1024)

## General Information

### Testnet Epoch Architecture

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

### Testnet Difficulty

The bootstrapped difficulty level for the _Mao Shan Wang_ testnet is set at `3`. This difficulty level is dynamic and adjusts for every `+/- 100 PoW` submissions per DS epoch.

> **NOTE:** Difficulty level is the log2(Difficulty).

Say if there are `1800` seats available in the network but there are `1900` PoW submissions, the difficulty level will increase by `1` for the next DS epoch.

Say if there are `1800` seats available in the network but there are `1700` PoW submissions, the difficulty level will decrease by `1` for the next DS epoch.

### Reward Mechanism

In the Zilliqa network, rewards are based on the accepted PoW solutions and the amount of signatures submitted by a node during a DS epoch. PoW solutions and signatures submitted by both shard and DS nodes are both rewarded equally. The rewards are consolidated for an entire DS epoch and only distributed during the vacuous epoch.

Say for example, if there are a total of `2,400` nodes in the Zilliqa network and the `COINBASE_REWARD` is set at `191780.82` ZILs per DS Epoch, the reward distribution will be:

- For PoW submissions (base reward for nodes joining Zilliqa network)
    ```shell
    191780.82 * 0.25 / 1800
    = 26.636225 ZILs per qualifying PoW submission
    ```
- For pBFT signature submissions (on a first-come-first-serve basis)
    ```shell
    191780.82 * 0.70 / (2,400 * 2/3 [Successful signers] * 99 [TX blocks])
    = 0.847516 ZILs per signature
    ```

## Hardware requirement for _Mao Shan Wang_ testnet

The Zilliqa client is officially supported only on Ubuntu OS. Other Debian distributions may also work. Please follow the steps [**HERE**](https://itsfoss.com/install-ubuntu-1404-dual-boot-mode-windows-8-81-uefi/) if you wish to dual boot Windows and Ubuntu 16.04.

We currently support both AMD (with OpenCL) and Nvidia (with OpenCL or CUDA) GPUs.

The minimum requirements for Zilliqa mining nodes are:

- x64 Linux operating system (e.g Ubuntu 16.04.05)
- Intel i5 processor or later
- 8GB DRR3 RAM or higher
- NAT environment or with Public IP address
- Any GPU cards with at least 3 GB vRAM

## Initial Setup

### Network setup

> **NOTE:** If you are using a home router, you are most probably in a NAT environment.

If you are in NAT environment, you can either:

- Do single port forwarding using **Option 1a**. This should be your **DEFAULT OPTION**.
- Enable UPnP mode using **Option 1b** if your router does support UPnP.

If you have a public IP address, you can skip this network setup entirely.

- **(Option 1a)** Port forward to port `30303` for both external port (port range) and internal port (local port). You will also have to select for **BOTH** TCP and UDP protocol in your router menu when port forwarding. An example this process can be found [**HERE**](https://www.linksys.com/us/support-article?articleNum=136711).

- **(Option 1b)** Enable UPnP mode on your home router. Please Google how to access your home router setting to enable UPnP, an example can be found [**HERE**](https://routerguide.net/how-to-enable-upnp-for-rt-ac66u/). You can check if you have enabled it UPnP by installing the following tool:
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
   The first message means UPnP mode has been enabled successfully, while the latter means the enabling of UPnP mode has failed.

### OpenCL driver setup (for AMD/Nvidia)

If you wish to use OpenCL supported GPUs for PoW, please run the following to install the OpenCL developer package:

   ```shell
   sudo apt install ocl-icd-opencl-dev
   ```

You may need to reboot your PC for the installation to take effect. After reboot, check if your drivers are installed properly with the following command:

   ```shell
   clinfo
   ```
If you are facing issues such as missing OpenCL drivers, please follow this forum guide found [**HERE**](https://forum.zilliqa.com/t/guide-to-setting-up-6-amd-gpus-on-ubuntu-16-04/180). (Credits to @Speccles96)

### CUDA driver setup (for Nvidia)

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from the [**NVIDIA official webpage**](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation to take effect.

### Multiple GPUs setup

If you wish to run multiple AMD or Nvidia GPUs concurrently, edit the `GPU_TO_USE` parameter in the _constants.xml_ file located in your _join_ folder.

The index start from `0` and you can input one or more multiple GPUs by separating their indexes with a `,`. 

For example:

- `0` for just 1 GPU.
- `0, 1, 2` or `0, 2, 4` for 3 GPUs.

Do note that the largest index must correspond to the number of GPUs you have physically in your mining rig.

## Steps for mining with docker (For CPU or Nvidia GPUs only)

1. Install Ubuntu 16.04.5 OS by following instructions [**HERE**](http://releases.ubuntu.com/xenial/).

2. Install Docker CE for Ubuntu by following instructions [**HERE**](https://docs.docker.com/install/linux/docker-ce/ubuntu/).

3. Make a new directory in your Desktop and change directory to it:

      ```shell
      cd ~/Desktop && mkdir join && cd join
      ```

4. Get the joining configuration files:

      ```shell
      wget https://testnet-join.zilliqa.com/configuration.tar.gz
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
    - **(Option 2)** For mining with Nvidia GPUs, please first install the `nvidia-docker` [**HERE**](https://github.com/NVIDIA/nvidia-docker#ubuntu-140416041804-debian-jessiestretch). Then, launch your docker container:

         ```shell
         ./launch_docker.sh --cuda
         ```

       > **NOTE:** If you wish to run multiple Nvidia GPUs concurrently, you will need to modify your _**constants.xml**_ file following instructions as found above [**HERE**](#multiple-gpus-setup).

7. You will be prompted to enter some information as shown below:
    - `Assign a name to your container (default: zilliqa):` [Press **Enter** to skip if using default]

    - `Enter your IP address ('NAT' or *.*.*.*):` [Key in your IP address as found in step 6 **OR** `NAT` if you using Option 1b]

    - `Enter your listening port (default: 30303):` [Press **Enter** to skip if using default]

8. You are now a miner in _Mao Shan Wang_ testnet. You can monitor your progress using:

      ```shell
      tail -f zilliqa-00001-log.txt
      ```

9. To check your locally generated public and private key pairs, you can enter the followin in your command prompt:

      ```shell
      less mykey.txt
      ```

    The first hex string is your **public key**, and the second hex string is your **private key**.

    > **NOTE:** This key pair is generated locally on your disk. Do remember to keep your private key somewhere safe!

10. To stop the mining client, stop your running docker container:

      ```
      sudo docker stop zilliqa
      ```

## Steps for mining natively

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
      git clone https://github.com/Zilliqa/Scilla.git Scilla && cd Scilla && git checkout v0.0.3
      ```

5. Find out your Scilla directory path and record it down:

      ```shell
      pwd
      ```

6. First, download the Scilla dependencies for Ubuntu following instructions found [**HERE**](https://github.com/Zilliqa/scilla/blob/master/INSTALL.md#ubuntu). Then, build the Scilla binary:

      ```shell
      make clean; make
      ```

7. Clone the Zilliqa repository and change directory to it:

      ```
      cd ~/Desktop && git clone https://github.com/Zilliqa/Zilliqa.git Zilliqa && cd Zilliqa && git checkout v3.3.0
      ```

8. Find out your Zilliqa directory path again and write it down:

      ```
      pwd
      ```

9. First, download the Zilliqa dependencies. Then, build Zilliqa for CPU mining with **Option 1** or for GPU mining with **Option 2**/**Option 3**.

    - Download the dependencies:

        ```shell
        sudo apt-get update
        sudo apt-get install git libboost-system-dev libboost-filesystem-dev libboost-test-dev \
        libssl-dev libleveldb-dev libjsoncpp-dev libsnappy-dev cmake libmicrohttpd-dev \
        libjsonrpccpp-dev build-essential pkg-config libevent-dev libminiupnpc-dev \
        libprotobuf-dev protobuf-compiler libcurl4-openssl-dev libboost-program-options-dev \
        libssl-dev
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

10. Download and unpack the compressed joining configuration file:

    ```shell
    cd ../join && wget https://testnet-join.zilliqa.com/configuration.tar.gz && tar zxvf configuration.tar.gz
    ```
11. Edit the _constants.xml_ in your _join_ folder to key in the Scilla directory for the `SCILLA_ROOT` parameter. An example is shown below:

    ```shell
    <SCILLA_ROOT>/home/ubuntu/Scilla</SCILLA_ROOT>
    ```

12. **(Optional)** If you wish to mine with GPUs, please continue to edit the following parameters in the _constants.xml_ file in your _join_ folder:

    - **For AMD GPUs:** Change `FULL_DATASET_MINE` parameter from `false` to  `true`. Change `OPENCL_GPU_MINE` parameter from `false` to `true`.
    - **For Nvidia GPUs:** Change `FULL_DATASET_MINE` parameter from `false` to  `true`. Change `CUDA_GPU_MINE` parameter from `false` to `true`.

    > **NOTE:** If you wish to run multiple GPUs concurrently, you will need to modify your _**constants.xml**_ file following instructions as found above [**HERE**](#multiple-gpus-setup).

13. Find out your current IP address in the command prompt and record it down.

    > **NOTE:** If you are using **Option 1b** as stated in the [**Network Setup**](#network-setup) above, you can skip this step.

    ```shell
    curl https://ipinfo.io/ip
    ```

14. Launch the Zilliqa client:

    ```shell
    ./launch.sh
    ```

15. You will be prompted to key in the following details:
    - `Enter the full path of your zilliqa source code directory:` [Key in the path you found it step 8]
    - `Enter your IP address (NAT or *.*.*.*):` [Key in your IP address as found in step 13 **OR** `NAT` if you are using Option 1b]
    - `Enter your listening port (default: 30303):` [Press **Enter** to skip if using default]

16. You are now a miner in _Mao Shan Wang_ testnet. You can monitor your progress using:
    ```shell
    tail -f zilliqa-00001-log.txt
    ```

17. To check your locally generated public and private key pairs, you can enter this in your command prompt:

    ```shell
    less mykey.txt
    ```
    The first hex string is your **public key**, and the second hex string is your **private key**.

    > **NOTE:** The key pair is generated locally on your disk. Do remember to keep your private key somewhere safe!

18. To stop Zilliqa client:

    ```shell
    pkill zilliqa
    ```

## Discussion channels

### Channels

Join our official mining discussion forum here: https://forum.zilliqa.com/c/Mining

Join the community managed Telegram channel here: https://t.me/zilliqaminer