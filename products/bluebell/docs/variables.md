# Variables

Variables in Scilla can be declared and used in the contract body and
transitions. Here's a basic example:

```scilla
scilla_version 0

library VariableExample

(* A basic variable declaration *)
let someVariable = Uint128 5

contract VariableContract()
```

## Integers

Integers in Scilla can be both signed and unsigned. The size can range from 32
bits to 256 bits.

```scilla
scilla_version 0

library IntegerExample

(* Unsigned Integer of 128 bits *)
let uintVal = Uint128 10

(* Signed Integer of 128 bits *)
let intVal = Int128 -10

contract IntegerContract()
```

## 32 byte unsigned integer

Uint256 can be used to declare a 32-byte unsigned integer:

```scilla
scilla_version 0

library Uint256Example

(* Unsigned 32-byte integer *)
let bigValue = Uint256 99999999999999999999999999999999999999

contract Uint256Contract()
```

## Boolean

Boolean values can either be `True` or `False`:

```scilla
scilla_version 0

library BooleanExample

let isTrue = True
let isFalse = False

contract BooleanContract()
```

## Lists

Lists or Lists can be used to store a collection of items:

```scilla
scilla_version 0

library ArrayExample

(* TODO: Not working *)
let numberList = [Uint128 1, Uint128 2, Uint128 3]

contract ArrayContract()

```

## Maps

Maps can be used to create key-value storage:

```scilla
scilla_version 0

library MapExample

contract MapContract()

field storedMap : Map Uint128 String = Emp Uint128 String

(* To add to the map *)
transition AddToMap(key : Uint128, value : String)
  storedMap[key] := value
end
```

## Events

Events can be emitted for external observers:

```scilla
scilla_version 0

library EventExample

contract EventContract()

(* event Notify(message: String) *)

transition EmitEvent()
  e = { _eventname : "Notify"; message : "Event emitted!" };
  event e
end
```

## State

State is used to maintain contract storage:

```scilla
scilla_version 0

library StateExample

contract StateContract()

field count : Uint128 = Uint128 0

transition Increment()
  current <- count;
  one = Uint128 1;
  newCount = builtin add current one;
  count := newCount
end
```

## Address

Addresses are used to identify contracts or users:

```scilla
scilla_version 0

library AddressExample

let someAddress = 0x1234567890123456789012345678901234567890

contract AddressContract()
```

## Type casting

Converting one data type to another:

```scilla
scilla_version 0

library TypeCastExample
contract TypeCastContract()

(* Fields to store the results *)
field uintVal : Uint128 = Uint128 0

(* Transition to demonstrate type "casting" *)
transition CastIntToUint(input : Int128)
  optUintResult = builtin to_uint128 input;
  match optUintResult with
  | Some uintResult =>
    uintVal := uintResult;
    e = { _eventname : "Casted"; uintValue : uintResult };
    event e
  | None =>
    e = { _eventname : "Error"; message : "Failed to cast Int128 to Uint128" };
    event e
  end
end
```

## Constants

Constants are values that cannot be changed:

```scilla
scilla_version 0

library ConstantExample

(* Scilla doesn't have a separate "constant" keyword, but you can use 'let' bindings as constants *)
let pi = Uint256 3141592653589793238

contract ConstantContract()
```

## Default values

Default values can be set for fields:

```scilla
scilla_version 0

library DefaultValueExample

contract DefaultValueContract()

field name : String = "DefaultName"
```
