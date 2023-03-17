---
tags:
  - ownership
---

# Pattern (ownership)

If your contract has an owner it typically means that address has admin powers meaning they have the privilege to adding/remove critical data or pause/unpause the contract.

It's important to keep your address keys safe, but if for any reason a `contract_owner` needs changing, there needs to be transitions pre-baked into your contract to handle this case.

## Transferring ownership

Note the below example. The `owner` field is initially set to `owner_at_deployment`, but can then be changed at a later date. You should consider an ownership pattern when writing contracts.

It shows three ways of how to change the owner of the contract:

`transition ChangeOwner(new_owner : ByStr20)`

ChangeOwner allows anybody to change contract_owner to any address, which is dangerous

`transition ChangeOwnerByOwnerOnly(new_owner : ByStr20)`

Here only the owner can change the ownership. This is better, yet still problematic as the new_owner might be wrong (a typo is enough), and thus the contract will have either a wrong owner, or even worse a non-existing owner.

The suggested way of transferring ownership, see Scilla Documentation -- use this in practice.
The current owner proposes (stages) a new owner

`transition RequestOwnershipTransfer(new_owner : ByStr20)`

The proposed new owner accepts the ownership

`transition ConfirmOwnershipTransfer()`

This allows a recall of ownership in the case the address was mistyped.

```ocaml
scilla_version 0

contract Ownership
(owner_at_deployment: ByStr20) (* immutable *)

(* mutable fields declarations *)
field owner : ByStr20 = owner_at_deployment (* the current owner *)
field pending_owner: Option ByStr20 = None {ByStr20} (* a new owner has been proposed *)

(* the safe way of tranfering owner ship: current owner proposes a new owner    *)
(* and the new onwer collects/accpets the owner ship                            *)
transition RequestOwnershipTransfer(new_owner : ByStr20)
  current <- owner;
  is_owner = builtin eq _origin current;
  match is_owner with
  | False => (* do not accept proposal as _origin is not current owner *)
    ev = {_eventname: "RequestOwnershipTransferFailureSameOwner"};
    event ev
  | True =>
    proposed = Some {ByStr20} new_owner;
    pending_owner := proposed;
    ev = {_eventname: "RequestOwnershipTransferSuccess"};
    event ev
  end
end

transition ConfirmOwnershipTransfer()
  proposed_option <- pending_owner;
  match proposed_option with
  | None => (* ownership transfer is not in-progress, do nothing *)
    ev = {_eventname: "ConfirmOwnershipTransferFailureNoProposedNewOwner"};
    event ev
  | Some proposed_owner =>
    caller_is_new_owner = builtin eq _origin proposed_owner;
    match caller_is_new_owner with
    | False => (* the caller is not the new owner, do nothing *)
      ev = {_eventname: "ConfirmOwnershipTransferFailureNotCalledByProposedNewOwner"};
      event ev
    | True => (* transfer ownership *)
      owner := proposed_owner;
      none = None {ByStr20};
      pending_owner := none;
      ev = {_eventname: "ConfirmOwnershipTransferSuccess"};
      event ev
    end
  end
end

```

## Further reading

[Scilla Documentation - Ownership](https://scilla.readthedocs.io/en/latest/scilla-tips-and-tricks.html?highlight=ownership#transfer-contract-ownership-1)

[Ownership.Scilla](https://github.com/TheDrBee/oSCILLAtor/blob/079f2400cfa1e6fdc7a7b0449bd65406186a1f3e/contracts/Ownership.scilla)
