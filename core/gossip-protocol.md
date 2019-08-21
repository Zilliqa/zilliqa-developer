# Rumor Manager

This document describes the rumor manager which enforces Gossip protocol for messaging in Zilliqa.


## Description

- The goal is to provide an efficient replacement to the existing broadcasting mechanism.
- The current broadcasting mechanism in `P2PComm::SendBroadcastMessage` is resource hungry; it sends O(n^2) messages, requires a lot of threads and opens a lot of TCP connections.
- The gossip algorithm, described in detail in this [link](https://zoo.cs.yale.edu/classes/cs426/2013/bib/karp00randomized.pdf) paper, provides a method to spread a message in O(logn) rounds and O(ln(ln(n))) messages,
where n is the number of peers participating in the gossip.
- RumorManager plays a role of managing all the gossips/rumors and their states.

## Interfaces

Following interfaces are exposed for node to enable gossiping messages in network.

### InitializeRumorManager

Every node in network intializes the RumorManager with the peers from their shard or DSCommittee at start of new DS epoch or after successful view change.

Initialization involves following: 
- Storing peer-list 
- Storing pubkeys of peers from peer-list, DSCommitte members and lookup nodes
- Storing self peer-info and pub/priv key
- Starting of Rounds - that runs loop every `ROUND_TIME_IN_MS` ms.
	- Checks the state of every rumor in RumorHolder (More on RumorHolder later) and sends to `MAX_NEIGHBORS_PER_ROUND` random peers if not old enough.
	- RumorHolder monitors/changes state of each rumor it holds using Median Counter algorithm as explained in paper ( section 3 ) for every round.

### SpreadRumor

Enables node to initiate the rumor to be gossipped with his peer-list. It will basically add the rumor to RumorHolder which in turn manages it states and further gossiping.
		
### SpreadForeignRumor

Enables node to initiate spreading out rumor received from node not part of his peer-list ( hence foreign ).
It verifies the sender node against all the pubkeys stored during initialization of RumorManager

### StopRounds

Stops the Round. Thereby stops gossiping rumors to peers.
  
  
## Rumor State Machine

   Every rumor will be in one of following state at any time
	
   - `NEW` : the peer `v` knows `r` and `counter(v,r) = m` (age/round)
   - `KNOWN` : cooling state, stay in this state for a `m_maxRounds` rounds, participating in rumor spreading
   - `OLD` : final state, member stops participating in rumor spreading
	
Every rumor starts with NEW. It either stay in same state or move on to KNOWN /OLD state immediately or in successive rounds based on algo mentioned in whitepaper. Every rumor is tied up with round ( consider it as rumor age).
	
Rumor is configured to stay in NEW and KNOWN state for max `<MAX_ROUNDS_IN_BSTATE>` and `<MAX_ROUNDS_IN_CSTATE>` respectively.
And to brutefully mark rumor as OLD, total rounds is limited to not exceed `<MAX_TOTAL_ROUNDS>`.
```
	     <gossip_custom_rounds>
            <MAX_ROUNDS_IN_BSTATE>2</MAX_ROUNDS_IN_BSTATE>
            <MAX_ROUNDS_IN_CSTATE>3</MAX_ROUNDS_IN_CSTATE>
            <MAX_TOTAL_ROUNDS>6</MAX_TOTAL_ROUNDS>
        </gossip_custom_rounds>
```

Rumor State Machine is managed by `RumorHolder`

## Gossip Message Format

| START_BYTE_GOSSIP (0X33) | HDR | GOSSIP_MSGTYPE | GOSSIP_ROUND | GOSSIP_SNDR_PORT | PUB_KEY_SIZE | SIGNATURE | Payload Message |
|--------------------------|-----|----------------|--------------|------------------|--------------|-----------|-----------------|

## Optimization with Pull-Push Mechanism

Following are the GOSSIP_MSGTYPE :

- `PUSH` = 0x01
		Indicates response to PULL request and payload contains real raw message and send out to requesting peer

- `PULL` = 0x02
		Indicates request for real raw message of given hash and payload contains hash. Its being send out to sender in response to LAZY_PUSH or LAZY_PULL

- `EMPTY_PUSH` = 0x03
		Being send out every round to random neighbors if it has not active rumor in it store. Indicates asking for any rumors from neighbors. Payload contains unused dummy data.

- `EMPTY_PULL` = 0x04	
		Being send out to sender of EMPTY_PULL or LAZY_PULL that it don't have any active rumors either. Payload contains unused dummy data.

- `FORWARD` = 0x05
		Special type that indicates that message being send out is from foreign peer. This would mean sender is from another shard or is lookup node.
	This would mean message is send out from 
	- 	lookup node to shard node or DSCommitte node
	-	shard node to DSCommitte node and vice-versa
			
- `LAZY_PUSH` = 0x06
		Being send out every round to random neighbors for each active rumor in it store. Its payload contains the hash of real raw message intented to be gossiped eventually.

- `LAZY_PULL` = 0x07
		Indicates the response to the sender if it is the first time 'sender' have sent a LAZY_PUSH/EMPTY_PUSH message in this round. Its payload contains the hash of real raw message
	
**(Note: Every gossip message is verified for signature before being accepted.)**
	
As mentioned above, Standard Push Pull mechanism is optimized further to gossip the hashes using EMPTY_* and LAZY_* and fetching the real messages using PUSH and PULL. 
So, LAZY_PUSH and LAZY_PULL are the backbone for gossiping of hashes and are only ones which has valid `GOSSIP_ROUND` for underlying rumor (hash in our case).
For rest of message type, GOSSIP_ROUND is just set to -1 since it's not of any use.


## Further optimization:

Due to nature of quick gossip, its possible that node might not have real message and only hash at some point of time. In such case, if node receives `PULL` message for that hash it adds that node to `subscription list`. As soon as nodes receives real message for that hash, it send it all peers in its subscription list.
