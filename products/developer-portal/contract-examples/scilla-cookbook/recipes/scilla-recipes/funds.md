---
tags:
  - funds
  - accept
  - zil
  - token
---

# Funds

One unit of ZIL as a native token is equal to 1000000000000 QA. (1 ZIL = 10^12 QA). When we deal with ZIL as a unit in Scilla, it is represented as an ```Uint128```. The below example shows how to accept/send funds to/from a smart contract.

:::danger
The example does not implement ownership logic, therefore anyone can withdraw funds.
Do not use this contract.
:::

## Accept ZIL contract

```Deposit``` takes any ZIL presented to it with the ```accept``` keyword.

```Withdraw``` checks the amount wanting to be withdrawn against what the contract currently has.

```Empty``` sends all of the contract funds to the ```_sender``` of the function.

```ocaml
scilla_version 0
(*********************************************************************************)
(* Receive, store and send funds [QA = 10^{-12} ZIL, LI = 10^{-6} ZIL = 10^6 QA] *)
(* in a smart contract                                                           *)
(*********************************************************************************)
import IntUtils

library Funds

let one_msg = (* Wrap single message into singleton list *)
  fun (msg : Message) =>
    let nil_msg = Nil {Message} in
    Cons {Message} msg nil_msg

let msg_as_list_wo_tag = (* Create transaction message as singleton list without a tag *)
  fun (recipient : ByStr20) =>
  fun (amount : Uint128) =>
    let msg = {_tag : ""; _recipient : recipient; _amount : amount } in
    one_msg msg

let is_positive = (* check if a uint is > 0 *)
  fun (n : Uint128) =>
    let zero = Uint128 0 in
    uint128_gt n zero

contract Funds
()

procedure TransferTo(to : ByStr20, amount : Uint128)
  msgs = msg_as_list_wo_tag to amount;
  send msgs
end

transition Deposit() (* send QA to contract to receive and store it in contract *)
  received = is_positive _amount;
  match received with
  |True  =>
    accept;
    b <- _balance;
    ev = {_eventname: "DepositSuccess"; amount_received: _amount; new_balance: b};
    event ev
  |False =>
  end
end

transition Withdraw(amount : Uint128) (* withdraw an amount from the contract *)
  b <- _balance;
  is_more_than_balance = uint128_gt amount b;
  match is_more_than_balance with
  |True => (* requested more than the balance, do not fullfil request *)
    ev = {_eventname: "WithdrawFailue"; amount_requested: amount; new_balance: b};
    event ev
  |False =>
    new_b = builtin sub b amount;
    TransferTo _sender amount;
    ev = {_eventname: "WithdrawSuccess"; amount_withdrawn: amount; new_balance: new_b};
    event ev
  end
end

transition Empty() (* withdraw everything *)
  b <- _balance;
  zero = Uint128 0;
  TransferTo _sender b;
  new_b <- _balance;
  ev = {_eventname: "EmptySuccess"; amount_withdrawn: b; new_balance: new_b};
  event ev
end
```

## Refund mechanism

This function checks the ```_amount``` against a passed parameter ```cap```. If the amount sent is less than cap, the procedure does nothing, but in the case where you send more than ```cap```, it can calculate the difference equal to where you send exactly the right amount and returns the excess amount to the user.

```ocaml
procedure AcceptWithCap (cap : Uint128)
  sent_more_than_necessary = builtin lt cap _amount;
  match sent_more_than_necessary with
  | True =>
      amount_to_refund = builtin sub _amount cap;
      accept;
      msg = { _tag : ""; _recipient: _sender; _amount: amount_to_refund };
      msgs = one_msg msg;
      send msgs
  | False =>
  end
end
```

## Further reading

[Scilla Documentation - Money Idioms](https://scilla.readthedocs.io/en/latest/scilla-tips-and-tricks.html?highlight=funds#money-idioms)

[Scilla Documentation - Units](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=funds#mutable-fields)
