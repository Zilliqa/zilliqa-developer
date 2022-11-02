---
sidebar_position: 4
---
# State and constants

## Mutable state

Mutable fields in a contract are declared through the keyword ```field```.

Each mutable field is declared in the following pattern ```field vname : vtype = init_val``` where ```vname``` is the field name, ```vtype``` is its type and ```init_val``` is the value to which the field has to be initialized.

Mutable fields hold the state of our contract. Unlike immutable fields, they are able to be changed and as such become useful to read and write from.

:::note
Think about the state you want to track. Does this change or is it a static value? What about in the future? Make anything that might change a mutable parameter.
:::

## Global constants

We can take advantage of defining a global definition of a constant through the keyword ```let```. The value defined cannot be changed throughout the life of the contract.

Each mutable field is declared in the following pattern ```field vname : vtype = init_val``` where ```vname``` is the field name, ```vtype``` is its type and ```init_val``` is the value to which the field has to be initialized.
:::note
Need to define a single instance of a type more than once in code? Global constants are tidier than mutable parameters.
:::

## IncrementingButton

Back to our example ```IncrementingButton```. let's define two immutable state variables, the first will track the changing instance of who the current owner of the contract is, and the second will be the counter that counts the amount of times this has changed. We need to define a constant value to what this counter is starting at also.

```current_clicker``` is a mutable field of type ByStr20. It will store the current clicker. When the contract is deployed, it takes whatever the initialisation parameter value of ```contract_owner``` is and copies it into ```current_owner``` when the contract is initialised.

```total_count_clicks``` is a mutable field of type Uint128 and will increment and store the amount of times the value ```current_clicker``` has been changed. It is set to the constant value of ```uint128_zero``` when the contract is initialised.

```ocaml {5,12,13}
scilla_version 0

library IncrementingButton

let uint128_zero = Uint128 0

contract IncrementingButton
(
  contract_owner: ByStr20
)

field current_clicker : ByStr20 = contract_owner
field total_count_clicks  : Uint128 = uint128_zero
```

:::note
We can set mutable fields in our contract to immutable parameters at deploy-time or global constants.
:::

In the next section, we will create internal contract logic that interacts with our mutable parameters to change the owner and increment the counter.
