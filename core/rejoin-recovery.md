# Rejoin / Recovery

This document will explain the concept of `rejoin` and `recover`.

## Rejoin

When following scenarios happened, `rejoin` process will be applied.

- A node is kick-out from network for some reason (e.g., Lose POW, recover)
- A new node (e.g., community node, `new`/`newlookup` node) want to join to network
- A shard node doesn't receive final block

Basically, the `rejoin` will fetch persistence as much as possible from AWS S3 buckets (`incremental`/`statedelta`) first. Then, if lagging behind, fetch the lacked information (DS info, DS block, TX block, statedelta...) from a random-selected lookup/level2lookup node, until vacuous epoch. After a new DS epoch, this node may successfully join back to network, or keep trying to rejoin in next DS epoch. Following is the brief flow chart of this idea:

![rejoin](images/features/rejoin-recovery/rejoin.jpg)

Here is more detail steps for `normal`, `DS`, and `lookup` nodes.

### Normal node

1. Download persistence from AWS S3
2. Clean variables in Node class
3. Retrieve Persistence Storage
4. Set SyncType to be NORMAL, thus to block some messages that will be received as a healthy normal node
5. Send Request A: Fetch if any lookup nodes are offline
6. Wait Request A feedback with conditional variable CV1.
7. While Loop until SyncType becomes NO_SYNC:
8. Send Request B: Fetch Latest DSBlocks
9. Send Request C: Fetch Latest TxBlocks
10. If PoW2 Started, sleep for BACKUP_POW2_WINDOW_IN_SECONDS, otherwise only NEW_NODE_SYNC_INTERVAL seconds.
11. If the feedback of Request C contains a new TxBlock, check whether it is a vacuous block, if so, send Request D: Fetch latest Account States.
12. If the feedback of Request B contains a new DSBlock, Send Request F: Fetch Latest DS Committee Info.
13. Wait until Request D got feedback, check fetchedDSInfo, if true, start PoW2, otherwise set a conditional variable CV2.
14. Wait until Request F got feedback, if it is not the first time get the Request F feedback && the Request D Started to listen DSInfo Updating && check if the current epoch is still the epoch when the DS committee changed, then notify CV2.
15. When CV2 get notified, start PoW2.
16. Init Mining and submit PoW2.
17. If received sharding information, change SyncType to NO_SYNC. Stop blocking messages. The normal node now successfully joined the network.

### DS node

1. Download persistence from AWS S3
2. If the DS Node was a DS Leader, it will do view change rather than do recovery as a DS Node. It the process of the DS Leader was killed, it will start joining as a Normal Node if triggered by the Daemon.
3. The steps for DS Node are the same as Normal Node until step 8, besides:
4. The step 2 is to clean the variables in DirectoryService class.
5. The step 3 is to set SyncType to be DS_SYNC.
6. The following is the step after step 8.
7. Wait until Request F got feedback, check isFirstLoop: if true, set to false; if false, mark the currDSExpired as true.
8. Wait until Request D got feedback, check if the currDSExpired, if false, change the SyncType to NO_SYNC, reset isFirstLoop to true, start RunConsensuOnDSBlock with no PoW1 submission.
9. This is to make sure the DS Node will declare its success of joining before the new DS Committee generated, then it’s legible to participant the DS Committee Consensus.

### Lookup node

1. Most of the steps for Lookup Node are the same as DS Node excludes:
2. At the beginning, the lookup node will tell the other lookup nodes that it will be offline.
3. The last step is to call RSync to get the latest TxBodies.Then set the SyncType to NO_SYNC, and tell the other lookup nodes it will be online.

## Recover

TBD