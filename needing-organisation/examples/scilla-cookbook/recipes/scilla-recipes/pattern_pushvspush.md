---
tags:
  - proxy
  - push
  - pull
  - merkel tree
---

# Pattern (push vs pull)

Let's consider the scenario where we have some funds in a contract or a wallet and we are trying to distribute these to multiple addresses. You could choose to send these manually, one-by-one or you could choose to automate batch sending these. In either case the token owner is sending `n` transactions to addresses.

![Docusaurus](/img/recipes/patterns/push-diagram.png)

Compare this to the Pull model. First, the entity responsible for the initiation of the transfer (e.g. the owner of a contract, or the contract itself) starts the process. Secondly, the smart contract has the responsibility of keeping track of all balances. The third participant is the receiver, who will not simply receive his funds via a transaction, but has to actively request a withdrawal, in order to isolate the process from other payout and contract logic.

![Docusaurus](/img/recipes/patterns/pull-diagram.png)

Comparatively the responsibility has switched from the contract owner to send directly to many addresses - to the contract owner sending a batch transaction to the contract containing all the accounts and their associated rewards. Users can now claim these directly from the contract.

So we've moved from pushing transactions to users, to sending one transaction to a smart contract that allows the specific user to pull an amount from. How would we improve this further?

A Merkel Tree is a data structure that takes a set of data. In our case this is a tuple of account and amount, and will hash each node with another to from a resulting node (as this is a tree, two nodes can be related to a parent) - those result nodes are hashed together also, until one master (root) result hash is present at the top of the tree. For a given user address at the bottom of the structure, you only need to know a certain amount of 'proof' nodes to calculate the master (root) hash.

Given that you have the data and can present an API to a user which for a given address and an epoch period can firstly tell you if you was present, and secondly generate a set of proofs for that address - a Merkel tree contract can be created. The developer would send 1 transaction with the master (root) to the contract instead of N - which is a vast improvement! The user sends the proofs to the contract which validates they are present and the proof plus their node equally matched the root hash.

## Further reading

[Token distributors in Scilla](https://medium.com/builders-of-zilliqa/token-distributors-in-scilla-b37241f7466a)

[Merkel tree and proofs for distributor](https://medium.com/builders-of-zilliqa/merkle-tree-and-proofs-for-distributor-e9c54f737e9)

[ZAP-API](https://github.com/Switcheo/zap-api)

[ZIL/Token distributor contract](https://github.com/Switcheo/zwap-token/tree/master/contracts)
