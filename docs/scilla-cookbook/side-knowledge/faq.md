---
tags:
  - FAQ
---

# FAQ

A list of common FAQ's developers might have.

## Why use Scilla over Solidity or other languages?

We believe in moving towards a language or a subset of an existing language grammar on which we
can prove safety properties about a contract. This will avoid most of the issues that existing contracts
are facing, e.g., DAO and Parity. Without any formal proofs about the contracts, such situations are
unavoidable.

Unlike Solidity, Scilla won’t be Turing complete. The computation model will be based on communicating I/O automata (I/O Automata theory by Lynch and Tuttle (’81) with CPS style return of values. The front-end language can be close to Solidity. The rationale behind the choice is the following: not all applications require a Turing complete language.

Moreover, Turing complete languages are hard to reason about and hence prone to bugs. A non Turing-complete language becomes amenable to formal methods-based verification because of its simplicity. In more concrete terms, it becomes possible to prove interesting safety and liveness properties about a non-Turing complete program such as the funds never get locked, etc.

## How is Zilliqa's mining different compared to Bitcoin POW?

Unlike Bitcoin, the mining process in Zilliqa is not directly based on PoW. Every Zilliqa node first does a PoW at the start of what is called a DS epoch. Once, a valid PoW solution is submitted to the network, each node will then have to participate in the PBFT consensus protocol. Consider the consensus protocol as a simple voting. If a super-majority of nodes vote for it, then the block will be considered valid and can be committed to the blockchain. Once a node has done PoW, it can vote for a certain number of blocks. Every block that gets committed to the blockchain will yield some reward.

The difference with Bitcoin is that in Bitcoin nodes do a PoW for every new block. For Zilliqa, a node will do a PoW for every 100 blocks. This also means that the energy footprint associated with PoW in Zilliqa is considerably lower.

## How does Zilliqa core protocol work?

Please see the [core protocol design](https://dev.zilliqa.com/docs/contributors/core-node-operation/) documentation.

## Further reading

[PDF Scilla Docs](https://scilla.readthedocs.io/_/downloads/en/latest/pdf/)

[Developer FAQ](https://docs.zilliqa.com/techfaq.pdf)

[Technical FAQ](https://docs.zilliqa.com/techfaq.pdf)

[Scilla Design Story](https://blog.zilliqa.com/scilla-design-story-piece-by-piece-part-1-why-do-we-need-a-new-language-27d5f14ae661)

[Core protocol design](https://dev.zilliqa.com/docs/contributors/core-node-operation/)
