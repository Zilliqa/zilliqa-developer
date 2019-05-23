# View change

This document describes the view change process in Zilliqa

<!-- TOC depthTo:2 -->

- [Description](#Description)
- [Usage](#Usage)
- [Terminology](#Description)
- [Trigger conditions](#Trigger-conditions)
- [Setup](#Setup)
- [Procedure](#Procedure)
- [Test scenario setup](#Test-scenario-setup)
- [General test scenario](#General-test-scenario)
- [Special test scenario](#Special-dddtest-scenario)


<!-- /TOC -->

## Description
In this version of view change, view change now supports random candidate leader selection and re-selection of candidate leader if it is the faulty leader. This version of view change also fixes the issue where the wrong ds leader is ejected to the back of the queue as a result of a previous hotfix that fixes view change issue after random DS leader election.

To conduct view change, the general steps are as follows:
1. A stall in consensus must have happened
2. Network enters into view change state
3. Candidate leader lead the view change consensus using PBFT
4. Backup validate the announcement
5, View change consensus is reached
6. Re-run the stalled consensus 


## Usage
Allows election of a new leader when the network cannot reach an agreement of the next state and stalled the consensus process


## Terminology
1. Candidate leader: Known as the proposed leader, the candidate leader will lead view change consensus round
2. Faulty leader(s): The current or previous DS leader(s) that is deemed to be faulty
3. Ejection: Reject the faulty leader(s) to the back of the ds committee. It will then be fully kicked out of the DS committee after the next DS consensus

## Trigger conditions
1. Node entered “RunConsensusOnDSBlock” but DS block consensus did not complete within the time stipulated
2. Node entered “RunConsensusOnFinalBlock” but final consensus did not complete within the time stipulated
3. Node entered “RunConsensusOnViewChange” but view change consensus did not complete within the time stipulated

## Setup
1. `[VC Block header]` Removal of candidate leader index as the index will be adjusted after view change and will be obsolete
2. `[VC Block header]` Addition of `vector<pair<PubKey, Peer>>` for tracking all the faulty leaders 
3. `[Macro]` Add the related test scenario macro. Refer to test macros section.

## Procedure:
1. Consensus stalled during DS consensus or Final block consensus
2. View change condition variable is triggered
3. Enter view change consensus
4. `[Precheck]` Enter the precheck phase. DS nodes contact lookup to ask for new blocks
5. `[Precheck]` If no new blocks (DS and FB) is obtained, proceeds to do view change
6. `[Precheck]` Else, rejoin as a DS node
7. All nodes calculate the new candidate leader index using `CalculateNewLeaderIndex()`
8. `CalculateNewLeaderIndex()` calculates candidate leader index using 
```
H((finalblock or vc block hash, vc counter) % size

If a previous vc block (for current consensus) exists, use vc block hash. Else use final block hash. If new candidate leader index is current faulty leader, re-calculate using
H(H((finalblock or vc block hash, vc counter)) repeatedly till an index is not the current faulty leader. 
```
9. Candidate leader lead the consensus round 
10. Backup validate leader announcement
11. View change consensus completed/Stalled
a. If stalled, wait for timeout and re-run view change consensus with a new candidate leader
12. Remove faulty leaders (found in Faulty leader vector) from DS Committee
13. Add faulty leaders (found in Faulty leader vector) to the back DS Committee
14. Recalculate `m_consensusMyID` by searching for own node inside the DS committee
15. Set new DS `m_consensusLeaderID`
16. Store VC block to persistent storage
17. If stalled consensus is at final block consensus, send the VC block to the lookup and shard nodes. Lookups and shard nodes update the ds composition respectively
18. If stalled consensus is at DS block consensus, hold and collect VC block(s) to the lookup and shard nodes.
19. Re-run stalled consensus (DS block or final block consensus)
20. Consensus completed
21. If DS block consensus, concatenate the DS block with VC block(s) and send to lookup and shard nodes
22. Lookup and shard nodes will process VC block(s) linearly followed by DS block


## Test scenario setup
A total of 6 general view change tests is built into the codebase as macro. To perform the test, 
1. Remove the build folder 
2. For a single test scenario
```bash
./build.sh vc<1-6>
```
3. For multiple test scenario
```bash
./build.sh vc<1-6> vc<1-6>
```
4. Build twice as the `ccache` may be hindering the macros 


## General test scenario

### Single failure
1. `vc1` - DS leader stalled at DS block consensus
2. `vc3` - DS leader stalled at Final block consensus

### Multiple failures (after view change is completed)
1. `vc2` - DS leader stalls at DS block consensus and 2 candidate leaders stall at DS block consensus 
2. `vc4` - DS leader stalls at Final block consensus and 2 candidate leaders stall at Final block consensus 

### Multiple failures (with view change consensus failure)
1. `vc1 vc5` - DS leader stalls at DS block consensus and candidate leaders stall at View Change consensus
2. `vc3 vc5` - DS leader stalls at Final block consensus and candidate leader stall at View Change consensus
3. `vc1 vc6` -` DS leader stalls at DS block consensus and 2 candidate leaders stall at View Change consensus
4. `vc3 vc6` - DS leader stalls at Final block consensus and 2 candidate leaders stall at View Change consensus

### VC Pre-check failed
1. `vc7` - When a DS backup is lagged (ds epoch) and the whole network did not enter into view change, check whether the node will rejoin as DS or not. Node with `consensusMyID` 3 will stall for 45s and enter view change to simulate node lagging behind.
2. `vc8` - When a DS backup is lagged (tx epoch) and the whole network did not enter into view change, check whether the node will rejoin as DS or not. Node with `consensusMyID` 3 will stall for 45s and enter view change to simulate node lagging behind.


## Special test scenario

Test plan for merging DS Microblock into FinalBlock consensus

1. Objective: Check fetching missing txn
   Scenario : DS leader has some txn that one of the backups doesn't have
   Adoption : Letting one of the backups accept fewer txns from lookup comparing to the others

2. Objective: Check View Change due to dsblock check failure within FinalBlock consensus
   Scenario : DS leader has some txn that all of backups don't have
   Adoption : Letting all of the backups accept fewer txns from lookup comparing to the leader

3. Objective: Check fetching missing microblock
   Scenario : DS leader has more microblock received than one of the backups
   Adoption : Letting one of the backups refuse some Microblock submission
4. Objective: Check View Change after fetching missing microblock
   Scenario : DS leader has more microblock received than all of the backups
   Adoption : Letting all of the backups refuse some Microblock submission

5. Objective: Check View Change due to TxBlock check failure within FinalBlock consensus
   Scenario : DS leader composed invalid TxBlock
   Adoption : Done by composing wrong state root hash

## Test macro
1. `vc1` - stall at the start of ds consensus for 1 time
2. `vc2` - stall at the start of ds consensus for 3 times
3. `vc3` - stall at the start of final consensus for 1 time
4. `vc4` - stall at the start of final consensus for 3 times
5. `vc5` - stall at the start of vc consensus for 1 time
6. `vc6` - stall at the start of vc consensus for 2 times 
7. `dm1` - letting one of the backups accept fewer txns from lookup comparing to the others
8. `dm2` - letting all of the backups accept fewer txns from lookup comparing to the leader
9. `dm3` - letting one of the backups refuse some Microblock submission
10. `dm4` - letting all of the backups refuse some Microblock submission
11. `dm5` - compose the wrong TxBlock, done by composing wrong state root hash in the TxBlock
12. `dm6` - compose the wrong DSMicroBlock, done by composing wrong tranhashes in the DSMicroBlock
13. `dm7` - letting the ds leader accept fewer txns from lookup comparing to the others
14. `dm8` - letting the ds leader and half of the ds committee members accept fewer txns from lookup comparing to the others
15. `dm9` - letting the ds leader and half of the ds committee members refuse some MicroBlock submission

