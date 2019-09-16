# LOCAL APIS

This API server run on port 4301 on a node locally (i.e. cannot be accessed from outside) by default.

## AddToBlacklistExclusion

Can be used to add an API to the blacklist exclusion list (or whitelist).

## RemoveFromBlacklistExclusion

Can be used to remove an API from the blacklist exclustion list.

## GetNodeState

Used to get the state of the node. for eg. POW, COMMIT_DONE etc.

## GetEpochFin

Tells the epoch number for the lookup for which the microblocks and txns have been recieved.

## GetDSCommittee

Returns the IP and PubKey of the current DS Committee.

## IsTxnInMemPool

Used to query local mempool of the nodes. Can tell, given a particular txnhash, if it is in mempool and why (Nonce too high or Gas price low).
