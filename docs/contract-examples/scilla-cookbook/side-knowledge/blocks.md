---
tags:
  - block
  - production
  - finality 
  - blocks per day
---
# Block production

When a transaction occurs, the data is submitted onchain, and the user pays some gas for the privilege to submit and have the chain respect the data they have posted. When it's posted to the network, it is bundled with other transactions that happened in the same period of time and is appended to the head of the blockchain.

## What is sharding

Sharding is a process of splitting up blockchain nodes into different groups. Each group has roughly the same number of nodes as each other, so if one shard can do ten transactions per second, all shards together can process 100 transactions per second.

Zilliqa’s consensus mechanism begins in a Proof of Work phase. During this phase, miners must confirm the blockchain by completing an individual hash. This process has finality, meaning that most nodes must agree on a mini-block before it is finalized.

## What is finality

There is a key difference between Zilliqa and blockchains like Bitcoin and Ethereum and that is finality to transactions.

Because of the fact that Bitcoin/Ethereum use PoW for consensus, every block that is mined has probabilistic finality, i.e., the probability that a block will indeed be a part of the canonical chain increases with the number of blocks that get added on top of it.

Zilliqa on the other hand uses PBFT for consensus which means that once a block is mined, it is final. Therefore you don't need to wait for any confirmation at all.

In Ethereum for instance, exchanges often wait for 30 confirmations as a newly mined block may not end up being in the canonical chain.

Also, though they are somewhat correlated, there is a difference between confirmation latency and general throughput.

## How many blocks are there per day

The blocks per day can very depending on network sharding, take note of the [statistics of the network](https://viewblock.io/zilliqa/stat/blockCountHistory) to determine this in your calculations. Divide the average blocks per day by 24 to get the hourly rate.
