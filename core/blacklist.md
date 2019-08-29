# Blacklist

Zilliqa has a blacklisting feature implemented in `libNetwork`. The idea is to keep track of IP addresses of peers that, for conditions listed below, can potentially disrupt the operation of the node. Once blacklisted, the peer is effectively excluded from further interactions with the node.

## Blacklisting conditions

- Socket write failure (according to `P2PComm::IsHostHavingNetworkIssue`)
- Socket connect failure (according to `P2PComm::IsHostHavingNetworkIssue`)
- Gossip message from peer exceeds `MAX_GOSSIP_MSG_SIZE_IN_BYTES`
- Bytes read from peer exceeds `MAX_READ_WATERMARK_IN_BYTES`

## Blacklist checking

Outgoing

- `Lookup::SendMessageToRandomSeedNode`
- `P2PComm::SendMessageNoQueue`
- `SendJob::SendMessageCore`
- `SendJobPeer::DoSend`
- `SendJobPeers<T>::DoSend`

Incoming

- `P2PComm::AcceptConnectionCallback`

## Blacklist exemptions

Adding exclusion privilege

1. DS guards
   - When `NEWDSGUARDNETWORKINFO` is received (new IP)
   - Whenever DS committee is updated
1. Lookup or seed nodes
   - Every time a message is sent out
1. Manual addition of an IP using `miner_info.py whitelist_add`

Removing exclusion privilege

1. DS guards
   - When `NEWDSGUARDNETWORKINFO` is received (old IP)
1. Manual removal of an IP using `miner_info.py whitelist_remove`

## Blacklist removal and clearing

- Non-lookup nodes remove `BLACKLIST_NUM_TO_POP` number of peers from the blacklist at the start of the DS epoch
- Lookup nodes clear the entire blacklist upon receiving the DS Block

## Blacklist enabling

Blacklist is enabled by default, and is only temporarily disabled when doing node recovery (`RECOVERY_ALL_SYNC`). In that situation, the blacklist is re-enabled once the final block is processed.