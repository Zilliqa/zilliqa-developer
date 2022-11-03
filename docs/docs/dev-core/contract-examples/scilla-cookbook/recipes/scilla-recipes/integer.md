---
tags:
  - int
  - uint
  - integers
---

# Integer

Scilla defines signed and unsigned integer types of 32, 64, 128, and 256 bits.

These integer types can be specified with the keywords ```IntX``` and ```UintX``` where ```X``` can be ```32```, ```64```, ```128```, or ```256```.

For example, the type of an unsigned integer of 32 bits is ```Uint32```.

The following code snippet declares a variable of type Uint32:

```ocaml
scilla_version 0

import IntUtils

let zero_int32 = Int32 0
let zero_int64 = Int64 0
let zero_int128 = Int128 0
let zero_int256 = Int256 0

let zero_uint32 = Uint32 0
let zero_uint64 = Uint64 0
let zero_uint128 = Uint128 0
let zero_uint256 = Uint256 0

contract Integers()

transition IsGreaterThan(a: Int32, b: Int32)
  is_gt = int32_gt a b;
  ev = {_eventname : "IsGreaterThan"; a: a; b: b; is_a_greater_b: is_gt};
  event ev
end

transition Example(a: Uint128, b: Uint128)
  c = builtin eq a b;  (*Is a equal to b? Returns a Bool.*)

  d = builtin add a b; (*Add integer values a and b. Returns an integer of the same type.*)

  e = builtin sub a b; (*Subtract b from a. Returns an integer of the same type.*)

  f = builtin mul a b; (*Integer product of a and b. Returns an integer of the same type.*)

  g = builtin div a b; (*Integer division of a by b. Returns an integer of the same type.*)

  h = builtin rem a b; (*The remainder of integer division of a by b. Returns an integer of the same type.*)

  i = builtin uint128_lt a b; (*Is a less than b? Returns a Bool.*)

  j = builtin uint128_le a b; (*Is a less or equal to b? Returns a Bool.*)

  k = builtin uint128_gt a b; (*Is a greater than b? Returns a Bool.*)

  l = builtin uint128_ge a b; (*Is a greater or equal than b? Returns a Bool.*)

  m = builtin uint128_pow a b; (*a raised to the power of b. Returns an integer of the same type as a.*)

  p = builtin uint128_isqrt a; (*Computes the integer square root of i, i.e. the largest integer j such that j * j <= i. Returns an integer of the same type as i.*)

  q = builtin uint128_to_nat a; (*Convert a value of type Uint32 to the equivalent value of type Nat.*)

  r = builtin to_(u)int32/64/128/256 (*Convert a UintX / IntX or a String (that represents a decimal number) value to the result of Option UintX or Option IntX type. Returns Some res if the conversion succeeded and None otherwise. The conversion may fail when*)
end
```

## Further reading

[Scilla Documentation - Integers](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html#integer-types)
