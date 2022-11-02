---
tags:
  - folding
  - recursion
  - iterate

---

# Functions

Functions in Scilla are abstracted a level away from Transitions and Procedures. Functions are available to be used by libraries and used in a forall statement as the function to be executed. Functions can also be used as helper methods in Transitions and Procedures.

Let's examine how a function is written and how it's returned.

The below function takes a singular argument of an Optional Uint128, the function has to return the same type, so in the case of no match (None), we will return Uint128 representation of zero, in the case where we have Some, we'll return the Uint128 representation of that value.

```ocaml
let get_val_from_maybe_uint128 =     (* Name of the function *)
  fun (some_val: Option Uint128) =>  (* Argument, in this case an unwrapped Option *)
  match some_val with                (* Scilla expression begins here *)
  | Some val => val                  (* Unwraps value and returns *)
  | None => uint128_zero             (* In the case of no match, return uint128 0, function return Some uint128 *)
  end
```

In our implementation, we can call on this function to unwrap any Option Uint128 without verbose Some/None pattern matching.

```ocaml {10}
let uint128_zero = Uint128 0
let user_map : Map ByStr20 Uint128 = Emp ByStr20 Uint128

(* Map of ByStr20 Uint128 maps a user to a token *)
procedure ExampleFunctionCall() 
  (* get a Some/None Uint128 value*)
  maybe_user_uint128_value <- user_map[recipient];    

  (* return Uint128 If None return 0 *)
  user_balance = get_val_from_maybe_uint128 maybe_user_uint128_value; 
  user_balance_is_zero = builtin eq user_balance uint128_zero

  (* act *)
  match user_balance_is_zero with
    | True =>
        (* user either has 0 balance OR wasn't present in the Map *)
    | False =>
  end
end
```

## Common functions examples

### blk_leq

blk_leq is a library function that takes two blocks. It defines two boolean variables ```bc1``` and ```bc2```.
If the first block is less than the second variable ```bc1``` is set to true.
If the first block is equal to the second variable ```bc2``` is set to true.
The ```ORB``` builtin from the Bools.Util performs a logical OR operator, returning true if either ```bc1``` or ```bc2``` are true.

```ocaml
let blk_leq =
  fun (blk1 : BNum) =>
  fun (blk2 : BNum) =>
    let bc1 = builtin blt blk1 blk2 in 
    let bc2 = builtin eq blk1 blk2 in 
    orb bc1 bc2
```

```ocaml
transition Implementation(query_block: BNum, current_block: BNum)
    less_or_equal_to_query = blk_leq query_block current_block;
end
```

### muldiv

Given the amount of tokens a user has, the total of the pool, and the amount of rewards - returns users percentage of rewards compared to the rest of the pool.

```ocaml
(* Compute "(x * y) / z" with protection against integer overflows *)
let muldiv : Uint128 -> Uint128 -> Uint128 -> Uint128 =
    fun (x : Uint128) =>
    fun (y : Uint128) =>
    fun (z : Uint128) =>
      let x256 = uint128_to_uint256 x in
      let y256 = uint128_to_uint256 y in
      let z256 = uint128_to_uint256 z in
      let x_mul_y256 = builtin mul x256 y256 in
      let res256 = builtin div x_mul_y256 z256 in
      let ores128 = builtin to_uint128 res256 in
      match ores128 with
      | None =>
        (* this must never happen, hence we throw an integer overflow exception *)
        let max_uint128 = Uint128 340282366920938463463374607431768211455 in
        let fourtytwo128 = Uint128 42 in
        builtin mul max_uint128 fourtytwo128
      | Some res128 =>
        res128
      end
```

### uint128_to_uint256

```ocaml
let uint128_to_uint256 : Uint128 -> Uint256 =
  fun (x : Uint128) =>
    let ox256 = builtin to_uint256 x in
      match ox256 with
      | None =>
        (* this never happens, hence we throw a division by zero exception just in case *)
        let zero = Uint256 0 in
        builtin div zero zero
      | Some x256 => x256
      end
```

### uint128_difference

```ocaml
let uint128_difference =
  fun (old: Uint128) =>
  fun (new: Uint128) =>
    let a = builtin lt old new in
    match a with
    | True =>
      builtin sub new old
    | False =>
      builtin sub old new
    end
```

### eqByStr20

Takes two addresses and returns a ```Bool``` from ```builtin eq``` if the addresses are equal.

```ocaml
let eqByStr20 =
  fun(bs1: ByStr20) =>
  fun(bs2: ByStr20) =>
    builtin eq bs1 bs2
```

## build_pair

Takes two arguments and constructs a Pair object from them.

```ocaml
let build_pair =
  fun (input_to_address : ByStr20) =>
  fun (input_token_uri : String) =>
    Pair {ByStr20 String} input_to_address input_token_uri
```

## (Generics) get_option_value

Allows for any generic optional type to be unwrapped using ```tfun 'A``` where ```'A``` is some type.

```ocaml
let get_option_value =
  tfun 'A =>
  fun (default: 'A) =>
  fun (opt_val: Option 'A) =>
    match opt_val with
    | Some v => v
    | None => default
    end
```
