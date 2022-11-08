---
sidebar_position: 5
---

# Error handling

In the below example we implement the ability to raise basic errors and stop execution.

Firstly, a Composite Type (ADT) Type ```Error``` is defined with the constructor values we want to raise as errors.

Secondly, a library function called ```make_error``` is defined. The function takes a variable called result of type ```Error``` and matches that to an error code. This is wrapped in a JSON object storing both the error name and error code.

Lastly, we define a procedure ```ThrowError``` which takes an instance of a variable with type ```Error```. It then calls the library function and returns the error. The keyword ```throw``` stops the execution of the smart contract with the specific error passed.

:::tip
Throwing errors with descriptive names helps users of your smart contracts debug what's wrong.
:::

```ocaml {8,9,10,12,13,14,15,16,17,18,19,20,33,34,35,36}
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
  Dev: Emit Errors 
*)
procedure ThrowError(err : Error)
  e = make_error err;
  throw e
end
```

## IncrementingButton

In the previous section, we defined a pattern matching syntax that allowed branching depending on what the type was. For some paths of execution we want the logic to fail and stop executing. The procedures ```IsContractOwner``` and ```IsPreviousClicker``` should throw an error in some cases.

We define wrapping of a new error variable by referencing the ADT type of error ```err = NotContractOwner``` and then calling the ```ThrowError``` procedure with the ```err```.
```ThrowError``` calls to the library function ```make_error``` to wrap the error and then finally the error is thrown, stopping the execution.

```ocaml {3,14,15,24,25}
...

procedure ThrowError(err : Error)
  e = make_error err;
  throw e
end

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

```

In the next section, we will create logic that users can interact with to press the button, we will be concerned with the functionality and security of our button, combining the atomic logical blocks we have built with procedures.
