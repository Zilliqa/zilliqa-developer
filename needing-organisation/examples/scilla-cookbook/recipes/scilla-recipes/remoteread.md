---
tags:
  - Remote
  - State
  - Reads
  - RSR
---

# Remote state reads

A remote state read is the mechanism to read another contract state from any another contract. These remote fetches can be implemented in a number of ways.

:::tip
Remote state reads only allow fetching of mutable fields, not immutable fields.
:::

## Address Subtyping

`ByStr20 with end`: Refers to an address that is in use. An address is in use if it either of the conditions is fulfilled

- balance greater than 0
- nonce of address greater than 0

`ByStr20 with contract end`: Refers to a contract address

`ByStr20 with contract field f1 : t1, field f2 : t2, ... end`: Refers to the address of a contract containing the mutable fields f1 of type t1, f2 of type t2, and so on. The contract in question may define more fields than the ones specified in the type, but the fields specified in the type must be defined in the contract.

## Inline remote state read

To perform a remote state read inline, the syntax `x <- & c.f` is used.

The type of c must be some contract address type declaring the field f. For instance, if c has the type ByStr20 with contract field paused : Bool end, then the value of the field paused at address c can be fetched using the statement `x <- & c.paused`.

## Example contract

```ocaml
scilla_version 0

contract RemoteRead
()

(* read a value from a contract that has a field value of type Uint128  *)
transition ReadValueFromSetGet(c: ByStr20 with contract field value: Uint128 end)
  value <- & c.value;
  ev = {_eventname : "ReadValueFromSetGet"; value: value};
  event ev
end

(* read a value from a contract that has afield value of type Uint128   *)
(* version #2: do not define the address type as a transition parameter *)
(*              but only inside the transition: Address type cast       *)
transition ReadValueFromSetGet2(addr: ByStr20)
  contract_opt <- &addr as ByStr20 with contract field value: Uint128 end;
  match contract_opt with
  | Some c =>
    value <- &c.value;
    ev = {_eventname : "ReadValueFromSetGet2"; value: value};
    event ev
  | None =>
    ev = {_eventname : "ReadValueFromSetGet2Failure"};
    event ev
  end
end
```

## Example - immutable Parameters

Users must declare the remote contract mutable fields for which they are trying to access. If the contract has more mutable fields than the user requires to read, they can be omitted.

```ocaml
(* Immutable parameter defined as 'An address on the network' which has fields value1, value2 *)
contract RemoteRead2
(
  remote_contract_addr: ByStr20 with contract
    field value1: Uint128,
    field value2: String
  end
)

(* Remote read the contract at 'remote_contract_addr' and fetch the values into value1, value2 *)
transition RemoteStateRead()
  value1 <-& remote_contract_addr.value1;
  value2 <-& remote_contract_addr.value2;
  ev = {_eventname : "RemoteStateRead"; value1: value1; value2: value2};
  event ev
end
```

### Remote State Read - migration of Map

This is an example how how migrate a map from contract A to contract B.
Because this implementation contains no batching functionality, if the map becomes too large, the read and write operations will exceed the maximum amount of gas allowed for a transaction to consume, rendering this function effectively broken.

:::tip
Can you improve the example to then batch on a selection of passed keys?
:::

```ocaml
(* A : Old contract, has state read by contract(b) *)
transition ProxyReadFromMap(old_contract: ByStr20)
  msg_to_new_contract = {
    _tag: "ReadFromMap";
    _recipient: new_contract;
    _amount: zero;
    old_contract: old_contract
  };
  msgs = one_msg msg_to_new_contract;
  send msgs
end

(* B : New contract, call this first with the old contract(a) *)
transition ReadFromMap (
  old_contract: ByStr20 with contract
    field map_a: Map ByStr20 Uint128
  end
)
  temp_new_a <- & old_contract.map_a;
  new_map_a := temp_new_a
end
```

## Further reading

[Remote state read Implementation](https://github.com/Zilliqa/scilla/pull/1014/files)
