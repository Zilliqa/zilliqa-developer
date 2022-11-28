# Proof Of Reputation

Set the node with successive co-signature to a higher priority. When the network is full, the PoW submission of nodes with higher priority will be processed first. It can prevent attackers using mass malicious nodes to join the network, so can improve the security of the blockchain.

## Working Mechanism

1. This machanism only take effect when the number of PoW submissions exceed MAX_SHARD_NODE_NUM defined in constants.xml.
1. When we bootstrap the system, the reputation of every node is 0.
1. For every consensus micro or final block, if a node signs a signature as ⅔ of nodes included in the co-signature, its reputation is incremented by one. The maximum reputation is capped at 4096.
1. If in any DS epoch, a node fails to join the network, its reputation will be reset to 0.
1. At the beginning of each DS epoch, DS leader will call “CalculateNodePriority” function to calculate the node priority based on the node reputation. The nodes with higher priority will be choosed first to add to sharding structure.
1. A DS backup node receives the DS announcement, it call "VerifyNodePriority" to calculates the node priority, and verify the nodes in sharding structure have meet the minimum priority requirement. If the check pass, it will accept it and continue the consensus protocol. Otherwise, it will refuse to commit to the announcement made by the leader. If more than 1/3 of the nodes in the DS committee does not commit to the announcement, view change will be triggered.
1. When a new DS leader is selected, the sharding structure is sent to it. The new DS leader can get the reputation of each node from the sharding structure.

## Reference

1. [PoR](https://drive.google.com/file/d/1hU4c8RUkRL5AJu7BwExqakXQpOPUR92D/view?usp=sharing)
