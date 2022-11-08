---
tags:
  - library
  - utilities
  - Util
---

# Imports

To use functions from the standard library in a contract, the relevant library file must be imported using the import declaration. The following code snippet shows how to import the functions from the ListUtils and IntUtils libraries:

## Functions

You can find all the typical functions available to the Scilla checker and compiler [here](https://github.com/Zilliqa/scilla-compiler/tree/master/src/stdlib). A typical query is asking about Integer '>=' or '<=' as these are different to the ```builtin lt``` and ```builtin gt```.

We can find all of the wrappers for the Int class in the above link [for the IntUtils](https://github.com/Zilliqa/scilla-compiler/blob/master/src/stdlib/IntUtils.scillib)

```ocaml
uint128_le
uint128_ge
uint128_gt
```

```ocaml
import ListUtils IntUtils (* builtin library function *)

library ExampleLib (* Your library definitions for variables and functions*)

contract Example() (* your consumation of said library definitions *)
```

## PairUtils

```ocaml
fst : Pair 'A 'B -> 'A: Extract the first element of a Pair.
snd : Pair 'A 'B -> 'B: Extract the second element of a Pair.
```

```ocaml
procedure Minting(input_pair: Pair ByStr20 String)
  to = let fst_string_bystr20 = @fst ByStr20 String in
    fst_string_bystr20 input_pair;
  token_uri = let snd_string_bystr20 = @snd ByStr20 String in
    snd_string_bystr20 input_pair;
```

## BoolUtils

```ocaml
andb : Bool -> Bool -> Bool: Computes the logical AND of two Bool values.
orb  : Bool -> Bool -> Bool: Computes the logical OR of two Bool values.
negb : Bool -> Bool: Computes the logical negation of a Bool value.
bool_to_string : Bool -> String: Transforms a Bool value into a String value. True is transformed into "True", and False is transformed into "False".
```

```ocaml
procedure IsOwnerOrOperator(token_owner: ByStr20)
  is_owner = builtin eq _sender token_owner;
  is_approved_for_all <- exists operator_approvals[token_owner][_sender];
  is_authorised = orb is_owner is_approved_for_all;
```

## IntUtils

```ocaml
intX_eq : IntX -> IntX -> Bool: 
Equality operator specialised for each IntX type.

uintX_eq : UintX -> UintX -> Bool: 
Equality operator specialised for each UintX type.

intX_lt : IntX -> IntX -> Bool: 
Less-than operator specialised for each IntX type.

uintX_lt : UintX -> UintX -> Bool:
Less-than operator specialised for each UintX type.

intX_neq : IntX -> IntX -> Bool:
 Not-equal operator specialised for each IntX type.

uintX_neq : UintX -> UintX -> Bool:
 Not-equal operator specialised for each UintX type.

intX_le : IntX -> IntX -> Bool:
 Less-than-or-equal operator specialised for each IntX type.

uintX_le : UintX -> UintX -> Bool:
 Less-than-or-equal operator specialised for each UintX type.

intX_gt : IntX -> IntX -> Bool: 
Greater-than operator specialised for each IntX type.

uintX_gt : UintX -> UintX -> Bool:
 Greater-than operator specialised for each UintX type.

intX_ge : IntX -> IntX -> Bool: 
Greater-than-or-equal operator specialised for each IntX type.

uintX_ge : UintX -> UintX -> Bool: 
Greater-than-or-equal operator specialised for each UintX type.
```

```ocaml
let a = Int32 100 in
let b = Int32 2 in
let c = Int32 40 in

let a_gt_b = int32_gt a b in
let a_ge_a = int32_ge a a in
let a_le_b = int32_le a b in
let a_lt_c = int32_lt a c in
let c_eq_a = int32_eq c a in
let b_neq_c = int32_neq b c 
```

## ListUtils

```ocaml
list_map : ('A -> 'B) -> List 'A -> : List 'B.
Apply f : 'A -> 'B to every element of l : List 'A, 
constructing a list (of type List 'B) of the results.

list_filter : ('A -> Bool) -> List 'A -> List 'A.
Filter out elements on the list based on the predicate f : 'A -> Bool. 
If an element satisfies f, it will be in the resultant list, 
otherwise it is removed. The order of the elements is preserved.

list_append : (List 'A -> List 'A ->  List 'A). 
Return the head element of a list l : List 'A as an optional value. 
If l is not empty with the first element h, 
the result is Some h. If l is empty, then the result is None.

list_head : (List 'A) -> (Option 'A).
Return the head element of a list l : List 'A as an optional value. 
If l is not empty with the first element h,
the result is Some h. If l is empty, then the result is None.

list_tail : (List 'A) -> (Option List 'A).
Return the tail of a list l : List 'A as an optional value.
If l is a non-empty list of the form Cons h t,
then the result is Some t. If l is empty, then the result is None.

list_foldl_while : ('B -> 'A -> Option 'B) -> 'B -> List 'A -> 'B
Given a function f : 'B -> 'A -> Option 'B, accumulator z : 'B and list ls : List 'A 
execute a left fold when our given function returns Some x : Option 'B using f z x : 'B or list is empty
but in the case of None : Option 'B terminate early, returning z.

list_append : (List 'A -> List 'A ->  List 'A).
Append the first list to the front of the second list, keeping the order of the elements in both lists.
Note that list_append has linear time complexity in the length of the first argument list.

list_reverse : (List 'A -> List 'A).
Return the reverse of the input list. Note that list_reverse has linear time complexity in the length of the argument list.

list_flatten : (List List 'A) -> List 'A.
Construct a list of all the elements in a list of lists. 
Each element (which has type List 'A) of the input list (which has type List List 'A) 
are all concatenated together, keeping the order of the input list. 
Note that list_flatten has linear time complexity in the total number of elements in all of the lists.

list_length : List 'A -> Uint32
Count the number of elements in a list. Note that list_length has linear time complexity in the number of elements in the list.

list_eq : ('A -> 'A -> Bool) -> List 'A -> List 'A -> Bool.
Compare two lists element by element, using a predicate function f : 'A -> 'A -> Bool. If f returns True for every pair of elements, then list_eq returns True. If f returns False for at least one pair of elements, or if the lists have different lengths, then list_eq returns False.

list_mem : ('A -> 'A -> Bool) -> 'A -> List 'A -> Bool.
Checks whether an element a : 'A is an element in the list l : List'A.
f : 'A -> 'A -> Bool should be provided for equality comparison.

list_forall : ('A -> Bool) -> List 'A -> Bool.
Check whether all elements of list l : List 'A satisfy the predicate f : 'A -> Bool.
list_forall returns True if all elements satisfy f, and False if at least one element does not satisfy f.

list_exists : ('A -> Bool) -> List 'A -> Bool.
Check whether at least one element of list l : List 'A satisfies the predicate f : 'A -> Bool.
list_exists returns True if at least one element satisfies f, and False if none of the elements satisfy f.

list_sort : ('A -> 'A -> Bool) -> List 'A -> List 'A.
Sort the input list l : List 'A using insertion sort. 
The comparison function flt : 'A -> 'A -> Bool provided must return True if its first argument is less than its second argument.
list_sort has quadratic time complexity.

list_find : ('A -> Bool) -> List 'A -> Option 'A.
Return the first element in a list l : List 'A satisfying the predicate f : 'A -> Bool. If at least one element in the list satisfies the predicate, and the first one of those elements is x, then the result is Some x. If no element satisfies the predicate, the result is None.

list_zip : List 'A -> List 'B -> List (Pair 'A 'B).
Combine two lists element by element, resulting in a list of pairs. If the lists have different lengths, the trailing elements of the longest list are ignored.

list_zip_with : ('A -> 'B -> 'C) -> List 'A -> List 'B -> List 'C ).
Combine two lists element by element using a combining function f : 'A -> 'B -> 'C.
The result of list_zip_with is a list of the results of applying f to the elements of the two lists. 
If the lists have different lengths, the trailing elements of the longest list are ignored.

list_unzip : List (Pair 'A 'B) -> Pair (List 'A) (List 'B).
Split a list of pairs into a pair of lists consisting of the elements of the pairs of the original list.

list_nth : Uint32 -> List 'A -> Option 'A.
```

```ocaml
let int_append = @list_append Int32 in

let one = Int32 1 in
let two = Int32 2 in
let ten = Int32 10 in
let eleven = Int32 11 in

let nil = Nil {Int32} in
let l1 = Cons {Int32} eleven nil in
let l2 = Cons {Int32} ten l1 in
let l3 = Cons {Int32} two l2 in
let l4 = Cons {Int32} one l3 in

int_append  l4 l2
```

## NatUtils

```ocaml
nat_prev : Nat -> Option Nat: 
Return the Peano number one less than the current one. 
If the current number is Zero, the result is None. If the current number is Succ x, then the result is Some x.

nat_fold_while : ('T -> Nat -> Option 'T) -> 'T -> Nat -> 'T: Takes arguments f : 'T -> Nat -> Option 'T, z : `T and m : Nat. 
This is nat_fold with early termination. Continues recursing so long as f returns Some y with new accumulator y. 
Once f returns None, the recursion terminates.

is_some_zero : Nat -> Bool: 
Zero check for Peano numbers.

nat_eq : Nat -> Nat -> Bool: 
Equality check specialised for the Nat type.

nat_to_int : Nat -> Uint32: 
Convert a Peano number to its equivalent Uint32 integer.

uintX_to_nat : UintX -> Nat: Convert a UintX integer to its equivalent Peano number. 
The integer must be small enough to fit into a Uint32. If it is not, then an overflow error will occur.

intX_to_nat : IntX -> Nat: Convert an IntX integer to its equivalent Peano number.
The integer must be non-negative, and must be small enough to fit into a Uint32. If it is not, then an underflow or overflow error will occur.
```

## Further reading
