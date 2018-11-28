# Mining Guide

## General Information
Welcome to testnet-v3 of Zilliqa. We are inviting all miners to test out the process of Zilliqa’s public node joining with their mining rigs, in order to familiarise everyone with the workflow before the mainnet launch on January 2019. We also encourage all community developers to join this testnet in order to better understand the architecture of the Zilliqa network.

### Difficulty
The bootstrap minimum difficulty level is set at 3 for testnet-v3. This difficulty level is dynamic and adjusts according to number of nodes competing to join the network.

**NOTE:** Difficulty level is log2(Difficulty).

### Epoch Architecture
![Zilliqa Epoch Architecture](https://i.ibb.co/hgY1j3r/Screenshot-2018-11-28-16-29-39.png)

At the start of each DS Epoch, nodes will run Proof-of-Work (Ethash algorithm) for a 150 seconds window to compete to join the Zilliqa network.

Nodes that fulfilled the `DS_POW_DIFFICULTY` parameter will then be able to join as DS nodes.
While, nodes that fulfilled the `POW_DIFFICULTY` parameter will be able to join as shard nodes.

There are a total of 50 TX epochs (each 1-2 min) within each DS Epoch (1-2 hrs). The 50th TX epoch is known as the Vacuous epoch.

A vacuous epoch handles the all the coinbase transactions (reward mechanism), upgrade mechanism (as there are no forks in pBFT), and persistent state storage (writing to nodes’ DB instead of just storing in just the memory). During a vacuous epoch, the network does not process any regular transactions.

### Reward Mechanism
In Zilliqa, each signature submitted by both shard or DS nodes are rewarded equally. The rewards are given out during the Vacuous epoch as discussed earlier.

Say for example, there are 2,500 nodes in the network and the `COINBASE_REWARD` is set at `100,000 ZILs` per DS Epoch, the reward distributed per signature is:

100,000 / (2,500 * 2/3 [Successful signers] * 49 [TX blocks]) = 1.22448979592 ZILs per signature

## Hardware requirement
Currently, mining only works with Ubuntu 16.04 OS. Please follow the steps [HERE](https://itsfoss.com/install-ubuntu-1404-dual-boot-mode-windows-8-81-uefi/) if you wish to dual boot Windows and Ubuntu 16.04.

We currently support both AMD (with OpenCL) and Nvidia GPUs (with CUDA).

The minimum requirements for Zilliqa mining nodes are:
* x64 Linux operating system such as Ubuntu 16.04.05
* Intel i5 processor
* 8G DRR3 RAM
* GPU cards (e.g 1 x GTX 1060, 3 GB)


### For OpenCL

If you wish to use OpenCL supported GPU for PoW, please run `sudo apt install ocl-icd-opencl-dev` to install the OpenCL developer package.

### For CUDA

If you wish to use CUDA supported GPU for PoW, please download and install CUDA package from [NVIDIA official webpage](https://developer.nvidia.com/cuda-downloads). You may need to reboot your PC for the installation to take effect. 

### For Multiple GPUs

If you have multiple OpenCL or CUDA GPUs, they can work concurrently. Please edit the `GPU_TO_USE` parameter in **constants.xml** to select the GPUs you want to use. The index start from 0, and you can use select one or multiple GPUs, for example, `0` for 1 GPU, `0, 1, 2` or `0, 2, 4` for 3 GPUs. Do make sure the largest index is within the physical number of GPUs you have in your mining rig.

## Steps for mining with docker
1. Install Ubuntu 16.04.05 OS by following instructions here: http://releases.ubuntu.com/xenial/

2. Install Docker CE for Ubuntu by following instructions here: https://docs.docker.com/install/linux/docker-ce/ubuntu/

3. Install Nvidia/AMD Drivers as mentioned above.

4. Get the docker image in your command line:
```
wget http://afec44962f2dc11e8984a066602678dc-420710417.us-west-2.elb.amazonaws.com/configuration.tar.gz
tar axvf configuration.tar.gz
```

5. Find out your current IP address in your command line:
```
curl https://ipinfo.io/ip
```
   **NOTE:** We only support public IP address and UPnP. Please do check if your router supports UPnP and how to enable the UPnP function.

6. Run the shell script to launch your docker image.
```
./launch_docker.sh
// If you are wish to switch to OpenCL or CUDA mode, do add the suffix like this: ./launch_docker.sh cuda
```

7. You will then be prompted to enter some information:
```
Assign a name to your container (default: zilliqa): [Press Enter to skip if using default]
Enter your IP address ('NAT' or *.*.*.*): [Key in your IP address as found in step 5] (e.g 18.136.119.220)
Enter your listening port (default: 30303): [Press Enter to skip if using default]
```
8. You will now be miner in the testnet-v3. You can monitor your node using `tail -f zilliqa-00001-log.txt`.

9. To check your public and private key, you can do `less mykey.txt`. The first hex string is your public key, and the second is your private key.

**NOTE:** The key pair is generated locally in your container. Do keep your private key somewhere safe!

## Discussion channels and error reporting
### Channels
Join our Official mining discussion channel here: https://gitter.im/Zilliqa/Mining

We also have a community managed telegram channel here: https://t.me/zilliqaminer

### Reporting
If you face an issues or errors while joining the testnet-v3, please do submit your log.txt files to this Google form: https://goo.gl/forms/zYe6ZIM9b5m2BdAa2. We will help you out whenever possible.




