# DS Reputation

This feature is to evaluate the performance of DS nodes. The underperformed DS nodes will be kicked out of DS committee first instead of the oldest DS nodes. It can push node owners to use better hardware and network for the DS nodes, so can improve the stability and the efficiency of the blockchain consensus protocol.

## Working Mechanism

1. When run DS block consensus, the performance of each DS node will be evaluated based on the rewards they got from last DS epoch until the current DS epoch.
1. At the beginning of each DS epoch, DS leader will call "DetermineByzantineNodes" to find out the underperformed DS nodes based on the criteria set in constants.xml. If found underperformed ds nodes, their pubkeys will be included in DS block announcement and send to all DS backup nodes.
1. A DS backup node receives the DS announcement, it call "VerifyRemovedByzantineNodes" to verify the proposed DS nodes going to remove from DS committee are really unerperformed. If the check pass, it will accept it and continue the consensus protocol. Otherwise, it will refuse to commit to the announcement made by the leader. If more than 1/3 of the nodes in the DS committee does not commit to the announcement, view change will be triggered.
1. After the DS block consensus is done, the selected underperformed DS nodes will be removed from the DS committee, and the shard nodes which finished DS PoW will join DS committee to replace the removed DS nodes, hence, can keep the size of the DS committee the same.

## Reference

1. [DS Reputation Proposal](https://github.com/nnamon/zilliqa-research/blob/master/ds_reputation/proposal.md)
2. [PR 1587](https://github.com/Zilliqa/Zilliqa/pull/1587)