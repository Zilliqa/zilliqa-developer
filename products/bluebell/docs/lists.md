# Lists

## One dimensional lists

Lists in Scilla are ordered collections of items that can have duplicate
elements. Lists are useful when there's a need to group together elements
sequentially.

```scilla
scilla_version 0
library ListLib

contract OneDimList()

field myList : List Int32 = Nil {Int32}

```

## Two dimensional lists

Two dimensional lists, often referred to as lists of lists, allow for
representation of matrix-like structures.

```scilla
scilla_version 0
library TwoDimListLib

contract TwoDimList()

field my2DList : List (List Int32) = Nil {(List Int32)}

```

## Nested lists

Nested lists allow for representation of tree-like structures or multi-level
lists.

```scilla
scilla_version 0
library NestedListLib

contract NestedList()

field nestedList : List (List (List Int32)) = Nil {(List (List Int32))}
```

## List functions

### Count

Counting elements in a list helps determine its length.

```scilla
scilla_version 0
library ListCountLib

contract ListCount()

field numbers : List Int32 = [1; 2; 3; 4]

transition countList()
  n <- numbers;
  len = @list_length Int32 n;
  event "ListLength" len
end
```

### Append

Appending allows you to combine two lists into one.

```scilla
scilla_version 0
library ListAppendLib

contract ListAppend()

field listA : List Int32 = [1; 2]
field listB : List Int32 = [3; 4]

transition appendLists()
  a <- listA;
  b <- listB;
  combined = @append Int32 a b;
  event "AppendedList" combined
end
```

### Extend

Extend can be used to add elements to the end of the list.

```scilla
scilla_version 0
library ListExtendLib

contract ListExtend()

field listA : List Int32 = [1; 2]

transition extendList(value : Int32)
  a <- listA;
  extended = @list_append Int32 a value;
  listA := extended
end
```

### Pop back and pop front

Popping elements from either the front or the back of a list removes them.

```scilla
(* NOTE: Scilla doesn't natively support pop operations.
   This example demonstrates a potential approach using helper functions *)

scilla_version 0
library PopFrontBackLib

contract PopFrontBack()

field list : List Int32 = [1; 2; 3]

transition popFront()
  l <- list;
  new_list = @tail Int32 l;
  list := new_list
end

transition popBack()
  l <- list;
  new_list = @remove_last Int32 l;
  list := new_list
end
```

### Pop back and front with n

Removing n elements from either the front or the back.

```scilla
(* NOTE: Demonstrating a potential approach with helper functions *)

scilla_version 0
library PopNLib

contract PopN()

field list : List Int32 = [1; 2; 3; 4; 5]

transition popFrontN(n : Uint32)
  l <- list;
  new_list = @remove_first_n Int32 l n;
  list := new_list
end

transition popBackN(n : Uint32)
  l <- list;
  new_list = @remove_last_n Int32 l n;
  list := new_list
end
```

### Reverse

Reversing a list inverts the order of its elements.

```scilla
scilla_version 0
library ReverseLib

contract ListReverse()

field list : List Int32 = [1; 2; 3]

transition reverseList()
  l <- list;
  reversed = @list_reverse Int32 l;
  list := reversed
end
```

### Erase

Erase removes a specific element or range of elements from the list.

```scilla
(* NOTE: Demonstrating a potential approach with helper functions *)

scilla_version 0
library EraseLib

contract ListErase()

field list : List Int32 = [1; 2; 3; 2; 4]

transition eraseValue(value : Int32)
  l <- list;
  new_list = @list_erase_value Int32 l value;
  list := new_list
end
```
