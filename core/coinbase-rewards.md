# Coinbase / Rewards

## Distribution

1. Each DS epoch `263698630136986000` QA is distributed. Out of which 25% is base reward and 5% is lookup reward,
rest is signature based reward.
2. Base reward is given to each node equally which does wins PoW in the beginning of the ds epoch.
3. Lookup reward is distributed to all the lookup nodes equally.
4. Rest 80% is signature based reward. This reward is given propotionally w.r.t the number of signatures
that node has in microblocks (in case of shard node) or tx blocks (in case of ds nodes).

*Note*: Guard nodes do not get reward, their reward is stored in Null Address.

## Process

1. The distribution of rewards take place in vacuous epoch.
2. The state change (subtraction from null address and addition to node's address) is reflected in the
state delta of the ds microblock.
3. The ds have consensus over the state delta and hence rewards are given.
4. Cosigs from first tx epoch (of current ds epoch) until vacuous epoch are considered for signature based rewards distribution.
5. Cosigs from shards only are considered for vacuous epoch (i.e., the tx block cosigs are excluded). That's because DS committee calculates the coinbase reward distribution, reach consensus among them and generate final block in vacuous epoch.

## Technical note

1. Coinbase reward is maintained in bit unusual way.
    `m_coinbaseRewardees[EPOCH][SHARDID]-->{Cosigs}`

    For example, Cosigs for `Epoch 5` with two shards( 0 and 1) are stored as below:
    `m_coinbaseRewardees[5][0] --> {Cosigs from Microblock proposed by shard 0}`
    `m_coinbaseRewardees[5][1] --> {Cosigs from Microblock proposed by shard 1}`
    `m_coinbaseRewardees[6][-1]--> {Cosigs from TxBlock mined by DS Committee}`
    This is because `IncreaseEpochNum` is called (inside `StoreFinalBlock`) before `SaveCoinbase` for TxBlock runs.
