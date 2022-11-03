---
sidebar_position: 2
---

# Constructor name

A contract is declared using the ```contract``` keyword that starts the scope of the contract.
The next keyword passed is the contract name. Only one constructor is able to be defined per contract.
When deploying a contract to the network, developers will be asked to define the variables passed into the constructor.

:::caution
A constructor (**immutable**) variable cannot be changed after deployment.
:::

This example contract name is defined as ```IncrementingButton```. A contract declaration is followed by the declaration of its immutable parameters, the scope of which is defined by ```()```.

```IncrementingButton``` currently does not have any immutable variables defined.

```ocaml
contract IncrementingButton()
```

We will improve our contract by providing an immutable variable called ```contract_owner```

Each immutable parameter is declared in the following pattern ```vname: vtype``` where ```vname``` is the parameter name and ```vtype``` is the parameter type. Multiple parameters are delimited with a  ```,```.

```contract_owner``` defines the owner of the contract. There will be transitions defined later that only the ```contract_owner``` can access.

```ocaml {3}
contract IncrementingButton
(
  contract_owner: ByStr20
)
```

These initialisation parameters can be viewed publicly from the ```state``` of the contract.

### Checking the immutable variables
It is possible to check that the immutable variables satisfy one or more conditions. For example, let's assume for a moment that our `IncrementingButton` contract has a second immutable variable `max_num_clicks: Uint32` that limits the number of clicks allowed, and that we want this variable to be positive. In other words, the deployment should fail if the variable is set to 0. This is called a "contract constraint" and is put in a ```with ... =>``` block right after the definition of the immutable variables. 

```ocaml
contract IncrementingButton
(
  contract_owner: ByStr20,
  max_num_clicks: Uint32
)
with
  let zero = Uint32 0 in (* check max_num_clicks > 0 *)
  uint32_gt max_num_clicks zero
=> 
```
More examples can be found in the [oSCILLAtor repo](https://github.com/TheDrBee/oSCILLAtor#initparams) under "InitParams".


We are still missing required fields that stop our contract being compiled correctly. See the next topic about Scilla Versions.
