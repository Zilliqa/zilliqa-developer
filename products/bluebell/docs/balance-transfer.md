# Contract funds

## Getting the balance owned by the contract

You can get the balance owned by the contract using the `_balance` keyword in
Scilla.

```scilla
scilla_version 0

library BalanceChecker
contract BalanceQuery()


transition CheckBalance()
  current_balance <- _balance;

  e = { _eventname : "ContractBalance"; current_balance : current_balance };
  event e
end
```

## Send funds from contract

In Scilla, sending funds from the contract involves using the `send`
instruction:

```scilla
scilla_version 0

library SendFunds

contract SendMoney(owner: ByStr20)

field balance : Uint128 = Uint128 0

transition Send(to: ByStr20, amount: Uint128)
  b <- _balance;
  can_send = builtin lt amount b;
  match can_send with
  | False =>
      e = { _eventname : "SendFailed"; reason : "InsufficientFunds" };
      event e
  | True =>
      msg = { _tag : ""; _recipient : to; _amount : amount };
      value = builtin sub b amount;
      balance := value;
      (* TODO: send msg; *)
      e = { _eventname : "SendSuccess"; recipient : to; sent_amount : amount };
      event e
  end
end
```

## Advanced example

TODO: Yet to be written
