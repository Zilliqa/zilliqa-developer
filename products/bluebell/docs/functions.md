# Functions

## Writing a function

In Scilla, functions are essential constructs that allow us to encapsulate
logic. A function can be defined using the `fun` keyword, and it has a set
format: `fun (parameter_name : parameter_type) => expression`.

Here's an example of a simple function that takes an `Int128` and checks if it's
less than 3:

```scilla
scilla_version 0

library SimpleFunctionLibrary

let f =
  fun (a : Int128) =>
    let three = Int128 3 in
    builtin lt a three

contract SimpleContract()
```

## Parameter and return types

Each function in Scilla must specify its parameter's type and will also have an
associated return type. The return type is inferred from the expression of the
function.

For instance, here's a function that checks if two `Uint128` numbers are equal
and if so, adds them:

```scilla
scilla_version 0

library CheckAndAddLibrary

let add_if_equal =
  fun (a : Uint128) => fun (b : Uint128) =>
    let eq = builtin eq a b in
    match eq with
    | True => builtin add a b
    | False => Uint128 0
    end

contract ContractName()
```

## Generics

Scilla allows for generic or parametric polymorphism through the use of type
functions (`tfun`). This means you can write code in a type-agnostic manner, and
then instantiate the type as required.

Here's a function that returns the first element of a pair, regardless of the
type of elements the pair holds:

```scilla
scilla_version 0

library GenericsLibrary

let fst =
  tfun 'A =>
  tfun 'B =>
  fun (p : Pair ('A) ('B)) =>
    match p with
    | Pair a b => a
    end

contract ContractName()
```

## Transitions and procedures

Transitions are the primary way by which a Scilla contract's state can be
modified. They are analogous to methods in object-oriented languages.
Procedures, on the other hand, are akin to transitions but cannot change the
contract's state.

Here's a contract with a transition to set an integer value:

```scilla
scilla_version 0

library SetValueLibrary

contract SetValueContract()

field stored_value : Int128 = Int128 0

transition Set(value : Int128)
  stored_value := value;
  e = { _eventname : "ValueSet"; new_value : value };
  event e
end
```

## Using functions in transitions

Functions and transitions often work hand-in-hand. Here's a contract that uses
the previously defined `add_if_equal` function in a transition:

```scilla
scilla_version 0

library UseFunctionInTransitionLibrary

let add_if_equal =
  fun (a : Uint128) => fun (b : Uint128) =>
    let eq = builtin eq a b in
    match eq with
    | True => builtin add a b
    | False => Uint128 0
    end

contract UseFunctionContract()

field result : Uint128 = Uint128 0

transition ComputeAndStore(a : Uint128, b : Uint128)
  r = add_if_equal a b;
  result := r;
  e = { _eventname : "ComputedResult"; value : r };
  event e
end
```

These examples give an overview of defining and using functions, transitions,
and procedures in Scilla.
