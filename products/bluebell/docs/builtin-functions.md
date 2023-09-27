# Builtin functions

## Builtin example usage

The `eq` builtin in Scilla is used to check the equality of two values. In this
example, we demonstrate how to check the equality of two Uint128 values:

```scilla
scilla_version 0

library EqualityCheck

contract EqualityChecker()

transition CheckEquality(a : Uint128, b : Uint128)
  (* Use builtin eq to check equality *)
  isEqual = builtin eq a b;
  e = { _eventname : "CheckEqualityResult"; result : isEqual };
  event e
end
```

The `concat` builtin in Scilla is used to concatenate two strings. In the
following example, we demonstrate how to concatenate two strings:

```scilla
scilla_version 0

library StringConcatContract

contract StringConcat()

(* Fields to store the strings and the result *)
field str1 : String = ""
field str2 : String = ""
field result : String = ""

(* Transition to set the strings *)
transition SetStrings(s1: String, s2: String)
  str1 := s1;
  str2 := s2;
  e = { _eventname : "StringsSet"; s1 : s1; s2 : s2 };
  event e
end

(* Transition to concatenate the stored strings *)
transition ConcatenateStrings()
  s1 <- str1;
  s2 <- str2;
  concatenated = builtin concat s1 s2;
  result := concatenated;
  e = { _eventname : "StringsConcatenated"; concatenated : concatenated };
  event e
end
```

With the above contracts, users can check the equality of two `Uint128` values
and concatenate two strings, respectively.

## Builtin summary

Below is the summarized table of builtin functions:

| Function name        | Inputs                                                           | Outputs               | Comments                                                                                 |
| -------------------- | ---------------------------------------------------------------- | --------------------- | ---------------------------------------------------------------------------------------- |
| `builtin eq`         | `i1: IntX / UintX, i2: IntX / UintX` OR `s1: String, s2: String` | `Bool`                | Checks equality between two integers or strings.                                         |
| `builtin add`        | `i1: IntX / UintX, i2: IntX / UintX`                             | `IntX / UintX`        | Adds two integer values.                                                                 |
| `builtin sub`        | `i1: IntX / UintX, i2: IntX / UintX`                             | `IntX / UintX`        | Subtracts the second integer from the first.                                             |
| `builtin mul`        | `i1: IntX / UintX, i2: IntX / UintX`                             | `IntX / UintX`        | Multiplies two integers.                                                                 |
| `builtin div`        | `i1: IntX / UintX, i2: IntX / UintX`                             | `IntX / UintX`        | Integer division.                                                                        |
| `builtin rem`        | `i1: IntX / UintX, i2: IntX / UintX`                             | `IntX / UintX`        | Provides the remainder after division.                                                   |
| `builtin lt`         | `i1: IntX / UintX, i2: IntX / UintX`                             | `Bool`                | Checks if the first integer is less than the second.                                     |
| `builtin pow`        | `i1: IntX / UintX, i2: Uint32`                                   | `IntX / UintX`        | Raises the first integer to the power of the second.                                     |
| `builtin isqrt`      | `i: UintX`                                                       | `UintX`               | Computes the integer square root.                                                        |
| `builtin to_nat`     | `i1: Uint32`                                                     | `Nat`                 | Converts a Uint32 value to type Nat.                                                     |
| `builtin to_(u)intX` | `UintX / IntX or String`                                         | `Option UintX / IntX` | Converts a value to a specified integer type. Can fail in certain cases.                 |
| `builtin concat`     | `s1: String, s2: String` OR `h1: ByStr(X/Y), h2: ByStr(X/Y)`     | `String / ByStr`      | Concatenates two strings or byte strings.                                                |
| `builtin substr`     | `s: String, idx: Uint32, len: Uint32`                            | `String`              | Extracts a substring from a given string.                                                |
| `builtin to_string`  | `x: IntX, UintX, ByStrX, ByStr`                                  | `String`              | Converts various types to a string literal.                                              |
| `builtin strlen`     | `s: String` OR `h: ByStr`                                        | `Uint32`              | Calculates the length of a string or byte string.                                        |
| `builtin strrev`     | `s: String`                                                      | `String`              | Returns the reversed version of a string.                                                |
| `builtin to_ascii`   | `h: ByStr or ByStrX`                                             | `String`              | Converts a byte string to an ASCII string. Raises an error for non-printable characters. |
| `builtin to_bystr`   | `h: ByStrX`                                                      | `ByStr`               | Converts a fixed size byte string to one of arbitrary length.                            |
| `builtin to_bystrX`  | `h: ByStr OR Uint(X)`                                            | `Option ByStrX`       | Converts an arbitrary size byte string or unsigned integer to a fixed size byte string.  |
| `builtin to_uintX`   | `h: ByStrX`                                                      | `Uint(X)`             | Converts a fixed sized byte string to an equivalent unsigned integer value.              |

Note: In the table, "X" and "Y" represent placeholder values, meaning you would
replace them with actual numeric values (32, 64, 128, 256) as appropriate for
the function's usage.
