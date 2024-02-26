# Syntax

Scilla's syntax is concise and functional, resembling OCaml and other ML-family
languages.

## Declarations

Scilla provides a way to declare constants, mutable fields (contract state), and
types.

### Constants

Declare a constant using the `let` keyword.

```scilla
let x = Uint128 5
```

### Fields

Fields represent the mutable state of a contract.

```scilla
field totalSupply : Uint128 = Uint128 1000000
```

Full example:

```scilla
scilla_version 0

library SimpleToken

contract SimpleToken()
field totalSupply : Uint128 = Uint128 1000000
```

### Maps

Maps are key-value storage constructs.

```scilla
field balances : Map ByStr20 Uint128 = Emp ByStr20 Uint128
```

Full example:

```scilla
scilla_version 0

library TokenWithBalances

contract TokenWithBalances()
field balances : Map ByStr20 Uint128 = Emp ByStr20 Uint128
```

### User-Defined ADT

You can also declare custom ADTs (Algebraic Data Types).

```scilla
type color = | Red | Blue | Green
```

Full example:

```scilla
scilla_version 0

library ColorContract

type color = | Red | Blue | Green

contract ColorContract()
field selectedColor : color = Red
```

## Procedures

Procedures are like transitions but can't change the contract's state or invoke
events.

```scilla
procedure CalculateSum(x : Uint128, y : Uint128)
  var sum : Uint128;
  sum := x + y;
end
```

## Transitions

Transitions are the primary way to interact with a contract. They can modify the
contract's state and invoke events.

```scilla
transition UpdateBalance(addr : ByStr20, value : Uint128)
  balances[addr] := value;
  e = { _eventname : "BalanceUpdated"; address : addr; new_balance : value };
  event e
end
```

Full example:

```scilla
scilla_version 0

library BalanceUpdater

contract BalanceUpdater()

field balances : Map ByStr20 Uint128 = Emp ByStr20 Uint128

transition UpdateBalance(addr : ByStr20, value : Uint128)
  balances[addr] := value;
  e = { _eventname : "BalanceUpdated"; address : addr; new_balance : value };
  event e
end

```

## Keywords

Here is a current list of Scilla keywords:

| Keyword name | Description                              |
| ------------ | ---------------------------------------- |
| `let`        | Declare a constant                       |
| `transition` | Define a contract transition             |
| `procedure`  | Define a contract procedure              |
| `field`      | Declare a mutable contract field (state) |
| `map`        | Key-value storage construct              |
| `type`       | Type definition                          |
| `end`        | End of a transition or procedure         |
| `event`      | Emit an event                            |
| `fun`        | Declares a function                      |
| `accept`     | Accept incoming funds to the contract    |
| `delete`     | Deletes entry in map                     |

## Comments

In Scilla, comments are denoted by `(*` to start the comment and `*)` to close
it.

```scilla
(* This is a comment *)
(*
 This is a multi-line comment
 *)
(* Nested (* Comments *) also work *)
```
