---
id: core-guard-mode
title: Guard Mode
keywords:
  - core
  - guard
  - mode
description: Core protocol design - guard mode.
---

---

Guard mode is a special operating mode in Zilliqa that can be used to maintain stability of the Mainnet until the protocol has been made perfectly robust. Guard mode ensures the following:

- A maximum of `n` nodes (e.g., 2/3) in the DS committee are nodes operated by Zilliqa Research
- A maximum of `n` nodes (e.g., 1/3) across all shards are nodes operated by Zilliqa Research
- DS leader selection (in either normal or view change situations) will only include nodes operated by Zilliqa Research

!!! note

    Guard mode is designed to be toggleable and does not interfere with the standard protocol whether or not it is enabled.

## Terminology

- DS guard - DS node operated by Zilliqa Research
- Shard guard - Shard node operated by Zilliqa Research

## Configuration

1. To enable guard mode, set `GUARD_MODE` to `true` in `constants.xml`
1. Add `n` DS guard public keys to the `ds_guard.DSPUBKEY` section in `constants.xml`
1. Add `n` shard guard public keys to the `shard_guard.SHARDPUBKEY` section in `constants.xml`
1. Adjust `SHARD_GUARD_TOL` in `constants.xml` to control the maximum percentage of shard guards in each shard

## Normal Operation

A DS guard is designed to be statically placed inside the DS committee. Given `n` DS guards, the first `n` slots in the DS committee will be allocated for those DS guards. While in guard mode, these positions do not change or shift during each DS consensus or view change.

<table>
  <tr>
    <th colspan="2">DS Committee</th>
  </tr>
  <tr>
    <td>1 ... n = DS guards (operated by Zilliqa Research)</td>
    <td>n+1 ... m = non-guard nodes</td>
  </tr>
</table>

The DS leader is selected from these DS guards by doing `mod n` rather than `mod m`.

A non-guard node joins the DS committee via [PoW](../mining/core-pow.md) as usual. If selected, it is inserted in the committee starting at index `n+1`. Following the [DS MIMO](../directory-service/core-ds-mimo.md) convention, the last few DS nodes (non-guards) are ejected from the DS committee to preserve the committee size.

!!! note

    The DS reputation feature (starting Zilliqa version 5.0.0) also impacts DS committee member placement. Please refer to both [DS MIMO](../directory-service/core-ds-mimo.md) and [DS Reputation](../directory-service/core-ds-reputation.md) sections for more information on how the DS committee membership is managed.

## View Change Operation

When a view change occurs, it is likely that the DS leader (a DS guard) is faulty or the network failed to agree with what the DS leader proposed. In such a case, the view change candidate leader will be selected from among the `n` DS guards by doing `mod n` rather than `mod m`.

Upon view change completion, there is no shifting of the DS guard nodes, i.e., the DS guards stay in place (even the faulty ones). The shard nodes who receive the generated VC block will also not adjust these nodes in their own view of the DS committee.

After the view change, the DS committee updates their `m_consensusLeaderID` to the new leader and the protocol resumes.

## Shard Guard Design

Shard guards are placed within shards in a manner such that there is a sufficient number of these Zilliqa-operated nodes in every shard. Shard guards are special as:

- They only do PoW with difficulty 1
- They cannot join the DS committee (hence, they only perform PoW to enter a shard)
- Their PoW submissions are given priority by the DS committee over normal shard nodes' submissions

After the PoW window is over, the DS committee will begin to compose the sharding structure. The DS leader, as per the protocol, will trim the list of nodes such that each shard will be expected to have exactly `COMM_SIZE` number of nodes. In guard mode, shard guards are given priority during the trimming, which means non-guard nodes are trimmed away first. With the trimmed list, the DS leader will then randomly assign each node (shard guard and non-shard guard) to its respective shard.

## Shard Rebalancing

When determining the shard composition - particularly in the event that the number of shards in the new DS epoch is lower than in the previous one - we must ensure that the newly composed shards will not be entirely made up of guards.

To do this, we trim the overall number of shard guards to 1/3 of the expected population (e.g., 600 out of 1800), and then complete the count with non-shard guards. However, in the event when there is not enough nodes to make up the EXPECTED_SHARD_NODE_NUM, the additional shard guards will fill up the gaps.

Keywords to look for in the logs:
