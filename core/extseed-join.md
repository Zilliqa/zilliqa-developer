# ExtSeed Join

This document will explain the workflow for external seed node which opted for syncing via level2lookups instead of multiplier.

## Motivation

Usual way of syncing via multiplier needs each seed node to get its `IP-Port` registered with multiplier.
This would mean that every time IP or Port is changed, zilliqa support team needs to deregister/register them again with multiplier.
So, instead we will let seed node sync via level2lookups.

Hereby, we will call such seed node as `extseed`.

## Registering process

Every exchange or external party willing to host their seed nodes under this option will need to generate the key pair and share the pubkey with zilliqa team.
This pubkey will be whitelisted on all level2lookups using command below.

`./testnet.sh register-extseed <<PUB_KEY>>`

If any party or exchange hosting any extseed nodes is off boarded, we would remove its pubkey from all level2lookup using command below.

`./testnet.sh deregister-extseed <<PUB_KEY>>`

The whitelisted pubkeys will be stored in persistence storage DB `extSeedPubKeys`.
This would enable level2lookup to repopulate whitelisted extseed pubkeys after being recovered for any maintenance reasons.

## Starting extseed node

Ext seed nodes will continue to be launched either by launch.sh/launch_docker.sh.
However, there will be additional options to be provided if its willing to use this new syncing feature.

    Assign a name to your container (default: zilliqa):
    [Press Enter to skip if using default]
    Enter your IP address ('NAT' or *.*.*.*):
    [Key in your IP address as found in step 5 OR NAT if you chose Option 1b during Network setup]
    Enter your listening port (default: 33133):
    [Press Enter to skip if using default]
  
    Use IP whitelisting registration approach (default: Y):
    [Press Enter to skip if using default else provide N]

    Enter the private key (32-byte hex string) to be used by this node and whitelisted by upper seeds: 

Note that extseed private key is not the one from seed node keys (mykeys.txt). It’s the key which will be shared by all seed nodes hosted by single external party.
Ex: All seed nodes hosted by single exchange will share same extseed private key.

Zilliqa process starts with additional optional parameter i.e. `--extseedprivk` so as to sign the request messages to be sent to level2lookup. More details at section "Authentication of extseed by level2lookup"

## Extseed lifecycle

1. ExtSeed will first fetch historical data from S3.

2. It fetches next pieces of data until current block from random level2lookups and will be marked as `SYNC`.

3. ExtSeed will continue to fetch following data forever from random level2lookups in given sequence.

The data to be pulled will depend on the current epoch.

a) If it is `non-vacuous` epoch, it will send a request for any next `vcfinal` block to random level2lookup.
Level2lookup will check if there is any corresponding `vcblock` being mined for requested vcfinal block.
And level2lookup will send the `vcfinal` block message back to a seed node.

Only after receiving the requested vcfinal block, it consider requesting another two types of data, namely `MBNFORWARDTRANSACTION` and `PENDIGNTXN`.
-It will identify the non empty microblocks and send requests for `MBNFORWARDTRANSACTION` messages for those shard num to random level2lookup.
-It will send request for `PENDINGTXN` messages for each shard and current txblock num to random level2lookup.

Level2lookup will check if the corresponding message exists for requested block num and shard num and send it back to the seed node.

b) If it is a vacuous epoch, it will send a request for a next ds block to random level2lookup.
Level2lookup will check if there is requested ds block. If so, it will send the ds block message back to seed.

## Level2lookup's role in extseed lifecycle

1. Level2lookup continue's to recieve data from multiplier.
2. Level2lookup will store the data received from multiplier in raw format in local memory store.
3. For every request received from extseed, it fetches the raw message from local store and send it back.

Following are type of messages stored:
-`VCDSBlock` message: After receiving and successfully processing this message, it stores it in `m_vcDSBlockStore` against `txblock` number.
-`VCBlock` message: After receiving and successfully processing this message, it stores it in `m_vcBlockStore`
-`VCFinalBlock` message: After receiving and successfully processing `Finalblock` message, it checks if there is any VC blocks in `m_vcBlockStore` and recreate new raw message of type `VCFINALBLOCK` (encapsulate vcblocks and final block together) and stores it in `m_vcFinalBlockStore` against `txblock` number.
-`MBnForwardTxn` message: After receiving and successfully processing this message, it stores it in `m_mbnForwardedTxnStore` against `txblock` number and `shardId`.
-`PendingTxn` message: After receiving and successfully processing this message, it stores it in `m_pendingTxnStore` against `txblock` number and `shardId`.

Entries in above local stores older than previous dsepoch are cleared every ds epoch.

## what if level2lookup is recovered

- If level2lookup is recovered for any reason, then local stores will be empty.
- In such case, if request for any message is received from extseed then it recreates the raw message with help of persistence storage, stores it in local store and send the raw message back to extseed.
- Any further request for same message will be served from local store itself.

## Authentication of extseed by level2lookup

- ExtSeed always includes its `PeerInfo` in request and signs the request being send out to level2lookup using the `extseed private key`.
- Level2lookup perform following 3 verification in given order:
  - Verify `signature` of request message
  - Verify if `PeerInfo` in request matches the actual sender.
  - Verify if `PubKey` in request is whitelisted pubkey.
- Only if above checks passes, the request will be further processed by level2lookup.
