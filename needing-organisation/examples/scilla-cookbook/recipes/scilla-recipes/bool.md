---
tags:
  - bool
  - boolean
  - "true"
  - "false"
  - if
---

# Boolean

The boolean (Bool) is a type which can have two values, true and false. A Bool is considered an ADT when performing interactions with contracts.

Consider the below definition of creating a type similar to Bool.

```ocaml
type Example = | True | False
```

A Bool value can be returned from functions like `builtin eq` which leverages the two types of Bool to return a value. The value `is_equal` is either true or false.

```ocaml
is_equal = builtin eq a b
```

Therefore, we can use Bool ADT pattern matching to perform a basic if statement

```ocaml
match is_equal with
    | True =>
    | False =>
end
```

## Further reading

[Bool](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=bool#boolean)

[BoolUtils](https://scilla.readthedocs.io/en/latest/stdlib.html?highlight=bool#boolutils)
