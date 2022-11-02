---
sidebar_position: 4
---

# Procedures

A ```procedure``` is a way to define logic that interacts with the contract state. Procedures are not part of the public interface of the contract, and may not be invoked by sending a message to the contract. The only way to invoke a procedure is to call it from an internal ```transition``` or from another internal ```procedure```.

:::note
Consider procedures as private encapsulation scope.
:::

Procedures in a contract are declared through keyword ```procedure```. A procedure declaration is followed by the name of the procedure. The below example the procedure is called ```ExampleProcedure```.

:::tip
The standard convention to writing procedure names is in PascalCase and snake_case for constants or state
:::

The arguments to be passed to the procedure are wrapped within  ```()```. The below example defines one parameter where ```vname``` is the argument name and ```vtype``` is its type.  Multiple parameters are delimited with a  ```,```. Procedures indicate the end of processing by providing the keyword ```end```.

```ocaml
procedure ExampleProcedure(vname: vtype)

end
```

## IncrementingButton

Lets improve ```IncrementingButton``` by defining some ```procedures``` that will handle the logic of our contract.

Remembering the scope of the brief, you stub out procedures which will define the logic for each requirement. We will use these procedures as atomic code blocks later in the public transitions. Atomic code helps reduce repeating code when building complex behaviour.

:::tip
Typically procedures with 'Is XXX' throw errors, allowing contracts to stop execution while procedures defined as 'DoSomething' performs an action.
:::

```ocaml
procedure IsContractOwner()
end

procedure IncrementCounter()
end 

procedure IsPreviousClicker(new_clicker: ByStr20)
end

procedure SetNewClicker(new_clicker: ByStr20)
end

procedure ContractOwnerResetButton()
end

```

### IsContractOwner

```IsContractOwner``` will check the calling address against what was deployed out with the contract. If these addresses are not equal then it will raise an error.
```_sender```  is a builtin variable which captures the calling addresses value. It's more secure to use ```_sender``` than passing an address that can be defined with a transition call.

```IsContractOwner``` uses ```builtin eq``` to compare ```contract_owner``` to ```_sender``` and returns the Bool value to ```is_contract_owner```.

Using pattern matching, we can write a basic function with the pattern ```match type_variable with | type =>```. We can now branch depending if that is true or false.

For now, we will leave these empty, but know we will want to throw an error later for the false path.

:::tip
Writing code comments helps others easily understand what is happening in your contract
:::

```ocaml
(*
  @Dev : Throws an error if '_sender' is not 'contract_owner'
*)
procedure IsContractOwner()
  is_contract_owner = builtin eq contract_owner _sender;
  match is_contract_owner with
  | True => 
    (* Continue contract execution *)
  | False =>
    (* Throw error - Stop execution *)
  end
end
```

### IncrementCounter

```IncrementCounter``` will increment the counter by one everytime its called. We plan to only use this procedure after we've verified the caller is unique.

We read the value of our mutable counter with the similar syntax as above, using the ```<-``` returning a new local variable. We then use the ```builtin add``` command to then add ```uint128_one``` and ```total_count_clicks``` together. This value is returned as a new variable called ```new_click_count```. Lastly, using the pattern ```state_type := new_state``` the state of the contract can be written to, in our case the state is updated with count + 1.

:::caution
Take note how the last statement line of the statement is not terminated with a semi-colon.

If this line ended with a semi-colon, the contract would fail to compile with the error ```What follows the statement was unexpected, for example possibly a statement or termination is expected.```
:::

```ocaml
(*  
  Dev: Increments 'current_clicker' by 'uint128_one'
*)
procedure IncrementCounter()
  previous_click_count <- total_count_clicks; (* get the state *)
  new_click_count = builtin add previous_click_count uint128_one; (* add one *)
  total_count_clicks := new_click_count (* set the state *)
end 
```

### IsPreviousClicker

```IsPreviousClicker``` defines the logic that checks if the current caller was the last caller.  If these addresses are equal to each other then we should throw an error, otherwise it's a unique address. For now, we will define these as empty, but know we will want to throw an error later for the true path.

```ocaml
(*  
  Dev: Throws an error if the current '_sender' is the previous 'current_clicker'
*)
procedure IsPreviousClicker(new_clicker: ByStr20)
  previous_clicker <- current_clicker;
  is_previous_clicker = builtin eq previous_clicker _sender;
  match is_previous_clicker with
  | True => 
    (* Throw error - Stop execution *)
  | False =>
    (* Continue contract execution *)
  end
end

```

### SetNewClicker

```SetNewClicker``` takes an address called ```new_clicker``` and sets it to the current clicker. We could have used ```_sender``` as another valid alternative to ```new_clicker```.

```ocaml
(*  
  Dev: Sets 'new_clicker' as 'current_clicker'
*)
procedure SetNewClicker(new_clicker: ByStr20)
  current_clicker := new_clicker
end
```

### ContractOwnerResetButton

```ContractOwnerResetButton``` sets the mutable state ```total_count_click``` variable to zero.

:::danger
When procedures define actions that change the state of the contract, developers should be extra mindful about unauthorised permissions. What if anyone could call ```ContractOwnerResetButton```?
:::

```ocaml
(*  
  Dev: Resets 'current_clicker' to 'uint128_zero'
*)
procedure ContractOwnerResetButton()
  total_count_clicks  := uint128_zero
end
```

### IncrementingButton review

In this section we have defined internal procedure logic to interact with our state parameters. Since procedures only allow for procedures and transitions to call them and we have none, this contract does not allow for the button to be pressed - though we have the parts to create the button.

In the next section we will write some additional logic to handle the two errors we've identified we want to throw.

```ocaml
scilla_version 0
library IncrementingButton

let uint128_zero = Uint128 0
let uint128_one  = Uint128 1

contract IncrementingButton
(
  contract_owner: ByStr20
)

field current_clicker    : ByStr20 = contract_owner
field total_count_clicks : Uint128 = uint128_zero

(*
  @Dev : Throws an error if '_sender' is not 'contract_owner'
*)
procedure IsContractOwner()
  is_contract_owner = builtin eq contract_owner _sender;
  match is_contract_owner with
  | True => 
  | False =>
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
  | False =>
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
```
