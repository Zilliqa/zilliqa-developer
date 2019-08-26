# Proof of Work

Proof-of-Work, or PoW, is the original consensus algorithm in a Blockchain network.

In other Blockchains for example Bitcoin and Ethereum, this algorithm is used to confirm transactions and produce new blocks to the chain. With PoW, miners compete against each other to complete transactions on the network and get rewarded.

But in Zilliqa, the PoW is used as a threshold shard node need to meet to join the network. And then it can start to sign transactions and get rewarded. So in Zilliqa, finished PoW doesn't mean this node can get reward.

## Why need PoW

The main benefits are the anti-DoS attacks defense and low impact of stake on mining possibilities.

Defense from DoS attacks.  PoW imposes some limits on actions in the network. They need a lot of efforts to be executed. Efficient attack requires a lot of computational power and a lot of time to do the calculations. Therefore, the attack is possible but kind of useless since the costs are too high.

Mining possibilities. It doesn’t matter how much money you have in your wallet. What matters is to have large computational power to solve the puzzles and form new blocks. Thus, the holders of huge amounts of money are not in charge of making decisions for the entire network.

## Ethash algorithm

Zilliqa blockchain is using the Ethash algorithm, which is originally from Ethereum.

Ethash is the proof-of-work function in Ethereum-based blockchain currencies. It uses Keccak, a hash function eventually standardized to SHA-3. These two are different, and should not be confused. Since version 1.0, Ethash has been designed to be ASIC-resistant via memory-hardness (harder to implement in special ASIC chips) and easily verifiable. It also uses a slightly modified version of earlier Dagger and Hashimoto hashes to remove computational overhead. Previously referred to as Dagger-Hashimoto, the Ethash function has evolved over time. Ethash uses an initial 1 GB dataset known as the Ethash DAG and a 16 MB cache for light clients to hold. These are regenerated every 30,000 blocks, known as an epoch. Miners grab slices of the DAG to generate mix-hashes using transaction and receipt data, along with a cryptographic nonce to generate a hash below a dynamic target difficulty.

## PoW mode

Now zilliqa support 5 modes of PoW, some are suitable for local or small scale test, some are for mainnet mining.

### Light Dataset Mine
This is the default mining mode if don't change any parameter in constants.xml. It uses CPU to do PoW. It will generate the dag data dynamically and doesn't store it in memory, hence it is the slowest method but it doesn't require the 1GB RAM. It is suitable for local test or small scale cloud test, not suitable for Mainnet mining.

### Full Dataset Mine
It will be enabled if change the "FULL_DATASET_MINE" to true if constants.xml. It uses CPU to do PoW. It is similar to light dataset mine, the dag is generated dynamically, but after generate, it is saved in memory. So next time, if need to use the same dag, it will directly read from memory. This method is faster than the light dataset mine, but it require 1GB RAM on the hardware. It is suitable for local test or small scale cloud test, not suitable for Mainnet mining.

### GPU Mine
It will be enabled if change the "CUDA_GPU_MINE" or "OPENCL_GPU_MINE" to true if constants.xml. It uses GPU to do PoW. There are more parameters available in `GPU` section in constants.xml. This mode uses GPU to generate the DAG and saved in GPU ram. It requires GPU has at least 1GB ram. Because GPU has thousands of cores, so the mining speed is much faster than CPU mine. It it suitable for mining mainnet in the beginning, but now the mainnet difficulty is too high, single machine is not possible to finish the PoW, so now it is only suitable for test purpose.

### Get Work Server Mine
This mode is enabled by set "GETWORK_SERVER_MINE" to true in constants.xml. The zilliqa node will be used as an mining server, other GPU machine can get work from this server and submit the result if the GPU machine find the result. It can combine the hash power of multiple GPU machines together to finish a high difficulty PoW job. But if there are multiple zilliqa node using this mode, it is not easy to maintain.

### Remote Mine
This mode is enabled by set "REMOTE_MINE" to true in constants.xml, and MINING_PROXY_URL need to set to the address of the mining proxy listening address. In this mode, multiple zilliqa node can send PoW work request to the mining proxy, and mining proxy dispatches the work packages to multiple mining machines. If the mining machine find result, it send it to the mining proxy, and mining proxy send it to Zilliqa node. This mode can support multiple Zilliqa nodes and mining machines, but it need to run a mining proxy server seperately.

## Reference
1. [Ethash](https://en.wikipedia.org/wiki/Ethash)
2. [Mining Proxy](https://github.com/DurianStallSingapore/Zilliqa-Mining-Proxy)