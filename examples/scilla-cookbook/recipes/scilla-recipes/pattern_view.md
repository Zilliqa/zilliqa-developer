---
tags:
  - view
  - viewer
  - pattern
---

# Pattern (viewer)

The viewer pattern is a standardised way of implementing functions that return a value without using callbacks.

Contract A is the calling contract. User calls ```Call_and_read_return_value``` which does two things. Firstly, it calls a transition on contract B called ```View_function``` which then sets some state. Secondly, control is passed back to contract A, and then calls a transition on itself called ```Read_Return_value``` which then can access the state field amended on contract B to use for onwards processing.

![Docusaurus](/img/recipes/patterns/viewer-diagram.png)

```ocaml
scilla_version 0

library Lib

let compute_the_value =
  fun (input_1 : ...) =>
  fun (input_2 : ...) =>
    ...
    =>
    ...

contract ContractWithReturnValues()

field state_field_1 : Uint128 = ...
field state_field_2 : String = ...
...

field return_value_1 : ... = ...
field return_value_2 : ... = ...

(* Perform a computation and state updates, and store the result in the field return_value_1 for the caller to read *)
transition Function_with_return_value(param1 : ..., param2 : ..., ...)
  <compute whatever needs to be computed, including state updates>;
  res = <the result that needs to be returned>;
  return_value_1 := res
end

(* Perform a computation without state updates, and store the result in the field return_value_2 for the caller to read *)
transition View_function(param1 : ..., param2 : ..., ...)
  (* A view function always follows this exact pattern *)
  (* Read the state fields needed to do the computation*)
  s1 <- state_field_1;
  s2 <- state_field_2;
  ...
  sn <- state_field_n;
  res = compute_the_value param1 param2 ... s1 s2 ...;
  return_value_2 := res
end

(* *********************************************************************************************** *)

scilla_version 0

contract CallingContract(contract_with_return_values : ByStr20 with contract field return_value_2 : ... end)

transition Call_and_read_return_value(...)
  remote <- contract_with_return_values;
  msg1 = { _recipient : remote; _tag : "View_function"; param1 : ... ; param2 : ...; ...};
  send msg1;
  msg2 = { _recipient : _this_address; _tag : "Read_Return_value"; ...};
  send msg2
end

transition Read_Return_value(...)
  return_value <-& contract_with_return_values.return_value_2;
  <use the returned value for further computation>
end
```
