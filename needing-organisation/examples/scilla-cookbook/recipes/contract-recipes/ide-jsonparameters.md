---
tags:
  - IDE
  - Interaction
  - call
  - type
---

# IDE interaction parameters

Developers may want to deploy contracts through the IDE for testing which require complex types or interaction. This page specifies the untypical type interactions developers may have with the IDE.

### Bool

```ocaml
field is_paused : Bool = True
```

```json
{
  "constructor": "False",
  "argtypes": [],
  "arguments": []
}
```

### List of Uint256

```ocaml
transition Test(input: List Uint256)
```

```json
["1", "2", "3"]
```

### Pair of ByStr20 Uint256

```ocaml
transition Test(input: List (Pair ByStr20 Uint256))
```

```json
[
  {
    "constructor": "Pair",
    "argtypes": ["ByStr20", "Uint256"],
    "arguments": ["0x0496a854570f27687B6f401d5e209fD14c3F3061", "5"]
  }
]
```

### List of Pair

```ocaml
transition Test(input: List (Pair ByStr20 Uint32))
```

```json
{
  "vname": "plist",
  "type": "List (Pair ByStr20 Uint32)",
  "value": [
    {
      "constructor": "Pair",
      "argtypes": ["ByStr20", "Uint32"],
      "arguments": ["0x12345678901234567890123456789012345678ab", "1"]
    },
    {
      "constructor": "Pair",
      "argtypes": ["ByStr20", "Uint32"],
      "arguments": ["0x12345678901234567890123456789012345678bd", "2"]
    }
  ]
}
```

### ADT

```ocaml
type OptionSimpleType = | OptionSimpleType of Uint256 (Option Uint256)

transition Test(input : OptionSimpleType)
```

```json
{
  "constructor": "OptionSimpleType",
  "argtypes": ["Uint256", "Option (Uint256)"],
  "arguments": [
    "500",
    {
      "constructor": "Some",
      "argtypes": ["Uint256"],
      "arguments": ["10"]
    }
  ]
}
```

```ocaml
type Vnode = | Vnode of ByStr20 Uint128 Uint128

transition Test(input: Vnode)
```

```json
{
  "constructor": "Vnode",
  "argtypes": ["ByStr20", "Uint128", "Uint128"],
  "arguments": ["0x12345", 1337, 2468]
}
```
