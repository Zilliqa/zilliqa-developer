---
sidebar_position: 6
---

# Transitions

A `transition` is a way to define logic that interacts with the contract state. transitions are a part of the public interface of the contract, and may be invoked by sending a message to the contract.

:::note
Consider transitions as public encapsulation scope.
:::

Transitions are declared through keyword `transition`. A procedure declaration is followed by the name of the procedure. The below example the procedure is called `ExampleTransition`.

The arguments to be passed to the transitions are wrapped within `()`. The below example defines one parameter where `vname` is the argument name and `vtype` is its type. When multiple arguments are passed they are separated by a `,`.

Transitions indicate the end of processing by providing the keyword `end`.

```ocaml
transition ExampleTransition(vname: vtype)

end
```

## IncrementingButton

Lets continue to improve IncrementingButton by defining some transitions that will be callable by anyone with the contract address.

Remembering the scope of the brief, you stub out transitions which will contain the public logic of our contract and implement the procedures implemented previously.

Two transitions were identified for implementing. One for pressing the button and another for resetting the button. We note that the caller of the function is defined as `_sender` so the transitions do not require any input parameters.

```ocaml
transition PressTheButton()
end

transition OwnerResetButton()
end
```

Using the previously defined procedures we can start to build callable logic into the contract.

:::note
Implementing transitions before procedures can result in type or not in scope errors, typically these are defined after all procedures.
:::

```ocaml
...

(*
  @Dev: Sets 'new_clicker' as current_clicker
*)
transition PressTheButton()
  IsPreviousClicker _sender;
  SetNewClicker _sender;
  IncrementCounter
end

(*
  @Dev: Sets 'new_clicker' as current_clicker
*)
transition OwnerResetButton()
  IsContractOwner;
  ContractOwnerResetButton
end
```

## IncrementingButton so far

```ocaml
scilla_version 0
library IncrementingButton

let uint128_zero = Uint128 0
let uint128_one  = Uint128 1

(* Error exception *)
type Error =
  | NotContractOwner
  | NotUniqueClicker

let make_error =
  fun (result : Error) =>
    let result_code =
      match result with
      | NotContractOwner             => Int32 -1
      | NotUniqueClicker             => Int32 -2
      end
    in
    { _exception : "Error"; code : result_code }

contract IncrementingButton
(
  contract_owner: ByStr20
)

field current_clicker    : ByStr20 = contract_owner
field total_count_clicks : Uint128 = uint128_zero

(*
  @Dev: Emit Errors
*)
procedure ThrowError(err : Error)
  e = make_error err;
  throw e
end

(*
  @Dev : Throws an error if '_sender' is not 'contract_owner'
*)
procedure IsContractOwner()
  is_contract_owner = builtin eq contract_owner _sender;
  match is_contract_owner with
  | True =>
    (* No Operation - Continue contract execution *)
  | False =>
    err = NotContractOwner;
    ThrowError err
  end
end

(*
  Dev: Increments 'current_clicker' by 'uint128_one'
*)
procedure IncrementCounter()
  previous_click_count <- total_count_clicks;
  new_click_count = builtin add previous_click_count uint128_one;
  total_count_clicks := new_click_count
end


(*
  Dev: Throws an error if the current '_sender' is the previous 'current_clicker'
*)
procedure IsPreviousClicker(new_clicker: ByStr20)
  previous_clicker <- current_clicker;
  is_previous_clicker = builtin eq previous_clicker _sender;
  match is_previous_clicker with
  | True =>
    err = NotUniqueClicker;
    ThrowError err
  | False =>
    (* No Operation - Continue contract execution *)
  end
end

(*
  Dev: Sets 'new_clicker' as 'current_clicker'
*)
procedure SetNewClicker(new_clicker: ByStr20)
  current_clicker := new_clicker
end

(*
  Dev: Resets 'current_clicker' to 'uint128_zero'
*)
procedure ContractOwnerResetButton()
  total_count_clicks  := uint128_zero
end

(*
  Dev: Sets 'new_clicker' as current_clicker
*)
transition PressTheButton()
  IsPreviousClicker _sender;
  SetNewClicker _sender;
  IncrementCounter
end

(*
  Dev: Sets 'new_clicker' as current_clicker
*)
transition OwnerResetButton()
  IsContractOwner;
  ContractOwnerResetButton
end

```
