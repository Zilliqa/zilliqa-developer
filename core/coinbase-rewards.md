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

1. The distribution of rewards take place in vacous epoch.
2. The state change (subtraction from null address and addition to node's address) is reflected in the
state delta of the ds microblock.
3. The ds have consensus over the state delta and hence rewards are given.