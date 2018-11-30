# Mining Guide

## General Information
Welcome to Zilliqa testnet-v3, code-named "placeholder". We are inviting all miners to test out the process of joining as a public node on "placeholder" testnet. We hope this exercise will familiarise everyone with the workflow and also help to find out potential bugs before the mainnet launch by end January 2019. We also encourage all community developers to join the "placeholder" testnet in order to better understand the architecture of the Zilliqa network.

- [Minimum hardware requirements](#hardware-requirement)
- [Steps for mining with docker](#steps-for-mining-with-docker-for-cpu-or-nvidia-gpus-only)
- [Steps for mining natively without docker](#steps-for-mining-natively-without-docker)

### Testnet Difficulty
The bootstrapped minimum difficulty level is set at `3` for the "placeholder" testnet. This difficulty level is dynamic and adjusts according to number of nodes that are competing to join the Zilliqa network.

>**NOTE:** Difficulty level is the log2(Difficulty).

### Testnet Epoch Architecture
![Zilliqa Epoch Architecture](https://i.ibb.co/hgY1j3r/Screenshot-2018-11-28-16-29-39.png)

At the start of each DS Epoch, all candidates will run the Proof-of-Work (Ethash algorithm) process for a `150` seconds window in order to compete to join the Zilliqa network.

Then, nodes that fulfilled the `DS_POW_DIFFICULTY` parameter will be able to join as DS nodes. While, nodes that fulfilled the `POW_DIFFICULTY` parameter will join as shard nodes.

There are a total of `50` TX epochs (each 1-2 min) within each DS Epoch (1-2 hrs). The 50th TX epoch is known as the **Vacuous epoch**.

A vacuous epoch handles the coinbase transactions (reward mechanism), upgrade mechanism (as there are no forks in pBFT), and persistent state storage (writing to nodes’ DB instead of just storing in just the memory). During a vacuous epoch, the network does not process any regular transactions.

### Reward Mechanism
In the Zilliqa network, rewards are based on the amount of signatures done by a node during a DS epoch. Signatures that are submitted by both shard and DS nodes are rewarded equally. The rewards are consolidated for a DS epoch and given out during the vacuous epoch.

Say for example, if there are a total of `2,500` nodes in the Zilliqa network and the `COINBASE_REWARD` is set at `100,000` ZILs per DS Epoch, the reward distributed per signature will be:

`100,000 / (2,500 * 2/3 [Successful signers] * 49 [TX blocks]) = 1.224489795920000 ZILs per signature`

## Hardware requirement
Currently, mining only works with Ubuntu 16.04 OS. Please follow the steps [HERE](https://itsfoss.com/install-ubuntu-1404-dual-boot-mode-windows-8-81-uefi/) if you wish to dual boot Windows and Ubuntu 16.04.

We currently support both AMD (with OpenCL) and Nvidia (with CUDA) GPUs.

The minimum requirements for Zilliqa mining nodes are:
* x64 Linux operating system such as Ubuntu 16.04.5
* Intel i5 processor or later
* 8GB DRR3 RAM or higher
* Any GPU cards with at least 20 Mh/s (e.g 1 x GTX 1060, 3 GB)


### For OpenCL

If you wish to use OpenCL supported GPU for PoW, please run `sudo apt install ocl-icd-opencl-dev` to install the OpenCL developer package.

### For CUDA

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from [NVIDIA official webpage](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation to take effect. 

### For Multiple GPUs

If you have multiple OpenCL or CUDA GPUs, they can work concurrently. Please edit the `GPU_TO_USE` parameter in **constants.xml** file to select amount of the GPUs that you would wish to use. 

The index start from `0` and you can select one or more multiple GPUs. For example, `0` for 1 GPU, `0, 1, 2` or `0, 2, 4` for 3 GPUs. Do make sure the largest index corresponds to the number of GPUs you have physically in your mining rig.

## Steps for mining with docker (For CPU or Nvidia GPUs only)
1. Install Ubuntu 16.04.5 OS by following instructions here: http://releases.ubuntu.com/xenial/.
***

2. Install Docker CE for Ubuntu by following instructions here: https://docs.docker.com/install/linux/docker-ce/ubuntu/.
***

3. Install Nvidia CUDA drivers as mentioned above [HERE](#for-cuda). You can skip this step if you are mining with CPU.
***

4. Get the docker image in your command prompt:
```
To be released...
```
***

5. Enable UPnP if you are in NAT environment or find out your current IP address in your command prompt:
> **NOTE:** Only public IP address and UPnP are supported. If you are using a home router, you are most probably in a NAT environment.
* Enable UPnP mode on your home router. Please Google your home router setting, an example can be found [HERE](https://routerguide.net/how-to-enable-upnp-for-rt-ac66u/).
* Find your IP address if your have a public IP address in your command prompt:
```
curl https://ipinfo.io/ip
```
***

6. Run the shell script in your command prompt to launch your docker image.
* For CPU mining:
```
./launch_docker.sh
```
* For Nvidia GPUs mining: We will be adding support using [nvidia-docker](https://github.com/NVIDIA/nvidia-docker) for Nvidia GPUs shortly. Please stay tuned.

>**NOTE:** Unfortunately, we don't have direct support for this docker build for AMD GPUs. We recommend you to navigate through this guide [HERE](https://instinct.radeon.com/en/amd-deep-learning-stack-using-docker/) if you still wish to use docker **OR** build Zilliqa natively instead of using docker by following instructions found [HERE](#steps-for-mining-natively-without-docker).
***

7. You will then be prompted to enter some information as shown below:
```
Assign a name to your container (default: zilliqa): [Press Enter to skip if using default]
Enter your IP address ('NAT' or *.*.*.*): [Key in NAT OR your public IP address as found in step 5]
Enter your listening port (default: 30303): [Press Enter to skip if using default]
```
***

8. You are now a miner in "placeholder" testnet. You can monitor your progress using:
```
tail -f zilliqa-00001-log.txt
``` 
You will be notified in the logs that you have become a shard/DS node in the network, if you managed to win the PoW process at the start of the DS epoch.
***

9. To check your locally generated public and private key pairs, you can do `less mykey.txt`. The first hex string is your public key, and the second is your private key.

>**NOTE:** The key pair is generated locally on your disk. Do remember to keep your private key somewhere safe!
***

10. If you wish to run multiple GPUs concurrently, you will need to modify your **constants.xml** file following instruction above as found [HERE](#for-multiple-gpus).

## Steps for mining natively without docker
To be released...

## Discussion channels and error reporting
### Channels
Join our official mining discussion Gitter channel here: https://gitter.im/Zilliqa/Mining

Join the community managed Telegram channel here: https://t.me/zilliqaminer

### Reporting
If you face an issues or errors while joining the "placeholder" testnet, please do submit your log.txt files to this Google form here: https://goo.gl/forms/y21CZrSwotvyNoKY2. 

We will help you out whenever possible.




