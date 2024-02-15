Here is a list of all builtins. Update according to
https://scilla.readthedocs.io/en/latest/scilla-in-depth.html#primitive-data-types-operations

| Function name    | Inputs                      | Outputs            | Comments                                 | Example                                                    |
| ---------------- | --------------------------- | ------------------ | ---------------------------------------- | ---------------------------------------------------------- |
| `add`            | `Integral, Integral`        | `Integral`         | Addition for integral types              | `result = builtin add x y`                                 |
| `sub`            | `Integral, Integral`        | `Integral`         | Subtraction for integral types           | `difference = builtin sub x y`                             |
| `mul`            | `Integral, Integral`        | `Integral`         | Multiplication for integral types        | `product = builtin mul x y`                                |
| `div`            | `Integral, Integral`        | `Integral`         | Division for integral types              | `quotient = builtin div x y`                               |
| `rem`            | `Integral, Integral`        | `Integral`         | Remainder for integral types             | `remainder = builtin rem x y`                              |
| `lt`             | `Integral, Integral`        | `Bool`             | Less than comparison                     | `is_less = builtin lt x y`                                 |
| `lte`            | `Integral, Integral`        | `Bool`             | Less than or equal comparison            | `is_leq = builtin lte x y`                                 |
| `eq`             | `'A, 'A`                    | `Bool`             | Equality check (generic)                 | `is_equal = builtin eq x y`                                |
| `gt`             | `Integral, Integral`        | `Bool`             | Greater than comparison                  | `is_greater = builtin gt x y`                              |
| `gte`            | `Integral, Integral`        | `Bool`             | Greater than or equal comparison         | `is_geq = builtin gte x y`                                 |
| `andb`           | `Bool, Bool`                | `Bool`             | Logical AND                              | `both_true = builtin andb x y`                             |
| `orb`            | `Bool, Bool`                | `Bool`             | Logical OR                               | `either_true = builtin orb x y`                            |
| `notb`           | `Bool`                      | `Bool`             | Logical NOT                              | `inverse = builtin notb x`                                 |
| `sha256hash`     | `ByStr`                     | `ByStr32`          | Performs SHA-256 hashing                 | `hash_val = builtin sha256hash data`                       |
| `keccak256hash`  | `ByStr`                     | `ByStr32`          | Performs Keccak-256 hashing              | `hash_val = builtin keccak256hash data`                    |
| `ripemd160hash`  | `ByStr`                     | `ByStr20`          | Performs RIPEMD-160 hashing              | `hash_val = builtin ripemd160hash data`                    |
| `schnorr_sign`   | `ByStr32, ByStr32`          | `ByStr64`          | Schnorr signature generation             | `signature = builtin schnorr_sign data private_key`        |
| `schnorr_verify` | `ByStr32, ByStr33, ByStr64` | `Bool`             | Schnorr signature verification           | `is_valid = builtin schnorr_verify data pub_key signature` |
| `head`           | `List 'A`                   | `Option 'A`        | Returns the first element                | `first_elem = builtin head lst`                            |
| `tail`           | `List 'A`                   | `Option (List 'A)` | Returns the list minus the first element | `remaining = builtin tail lst`                             |
| `append`         | `List 'A, List 'A`          | `List 'A`          | Appends two lists                        | `combined = builtin append lst1 lst2`                      |
| `is_some`        | `Option 'A`                 | `Bool`             | Checks if Option has value               | `has_value = builtin is_some opt`                          |
| `is_none`        | `Option 'A`                 | `Bool`             | Checks if Option is None                 | `is_empty = builtin is_none opt`                           |
| `option_match`   | `Option 'A, 'A -> 'B, 'B`   | `'B`               | Matches Option value or None             | `result = builtin option_match opt (fun v => v) default`   |
| `put`            | `Map 'K 'V, 'K, 'V`         | `Map 'K 'V`        | Puts a key-value pair in the map         | `updated_map = builtin put m k v`                          |
| `get`            | `Map 'K 'V, 'K`             | `Option 'V`        | Retrieves value for a key from the map   | `value_opt = builtin get m k`                              |
| `contains`       | `Map 'K 'V, 'K`             | `Bool`             | Checks if a key exists in the map        | `key_exists = builtin contains m k`                        |
| `remove`         | `Map 'K 'V, 'K`             | `Map 'K 'V`        | Removes a key-value pair from the map    | `map_without_key = builtin remove m k`                     |

| Special Identifier | Type      | Comments                            | Example                             |
| ------------------ | --------- | ----------------------------------- | ----------------------------------- |
| `_sender`          | `ByStr20` | Address of the caller               | `let caller = _sender`              |
| `_amount`          | `Uint128` | ZILs sent with the current message  | `let funds_received = _amount`      |
| `_this_address`    | `ByStr20` | Address of the current contract     | `let contract_addr = _this_address` |
| `_creation_block`  | `Uint128` | Block number of contract's creation | `let created_on = _creation_block`  |

`add`, `sub`, `mul`, `div`, `rem`, `lt`, `lte`, `eq`, `gt`, `gte`, `andb`,
`orb`, `notb`, `sha256hash`, `keccak256hash`, `ripemd160hash`, `schnorr_sign`,
`schnorr_verify`
