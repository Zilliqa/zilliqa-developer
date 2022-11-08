---
tags:
  - random
  - randomisation
---

# Pseudo-random

The difference between true random number generators(TRNGs) and pseudo-random number generators(PRNGs) is that TRNGs use an unpredictable physical means to generate numbers (like atmospheric noise), and pseudo-random numbers are sets of algorithms that utilize mathematical formulas to produce a certain sequence of numbers that will appear random, or at least will have the effect of randomness.

If you listed down the results of pseudo-randomness mimicking dice rolls the numbers will really appear as if they are random. But statistical analysis will prove that the numbers produced by a pseudo-random algorithm is not really random but is rather predetermined. Thus its results can effectively calculated and guessed.

# Implementation of pseudo random

The below function is an example of how pseudo-random can be implemented using keccakhash. The result of which could be predetermined and guessed ahead of time.

```ocaml
let random =
  fun (entropy: Uint256) =>
  fun (block_number: BNum) =>
  fun (addr: ByStr20) =>
    let addr_hash = builtin keccak256hash addr in
    let entropy_hash = builtin keccak256hash entropy in
    let blockhash = builtin keccak256hash block_number in
    let ehash = builtin concat entropy_hash blockhash in
    let ahash = builtin concat ehash addr_hash in
    let last_hash = builtin concat ehash ahash in
    let hash = builtin keccak256hash last_hash in
      builtin to_uint256 hash
```
