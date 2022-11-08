---
tags:
  - pause
  - pattern
---

# Pattern (pause)

Once a smart contract has been deployed to the network it becomes immutable. Let's consider some usecases where pauses become useful to implement in your smart contract design.

Firstly, let's consider a vulnerability has been discovered in your contract. How do you begin to secure such a contract so that the vulnerability can't be executed?

What if you wanted to upgrade a contract to a new implementation. How do you stop users interacting with the old implementation?

Pauses and how they are implemented become an important part of smart contract security. The overall goals are to stop user interactions on the contract whilst allowing the contract owner to call critical transitions to resecure the assets.

![Docusaurus](/img/recipes/patterns/pause-diagram.png)

```ocaml
let bool_false = False

field is_paused : Bool = bool_false

procedure IsContractOwner()
    ...
end

procedure ThrowIfPaused()
  paused <- is_paused;
  match paused with
    | True =>
      err = ContractPaused;
      ThrowError err
    | False =>
  end 
end

transition OwnerSetPause(new_pause_state: Bool)
  IsContractOwner;
  is_paused := new_pause_state
end

```

Once pauses have been implemented in contract how they are called is also important. What good is writing pause functionality if the transitions don't implement it? The below transitions show a simple example of the implementation of how pause should be implemented on every user function.

:::tip
If you store assets you should be mindful of how you will migrate in the future.
:::

```ocaml
transition UserDepositInteraction()
    ThrowIfPaused
end

transition UserWithdrawInteraction()
    ThrowIfPaused
end

transition AdminOverride()
  paused <- is_paused;
  match paused with
    | True =>
        (* override implementation, send assets to their owner, contract dead ...*)
    | False =>
  end 
end
```
