---
tags:
  - Map
---

# Maps

A value of type `Map kt vt` provides a key-value store where `kt` is the type of keys and `vt` is the type of values.

The type of map keys `kt` may be any one of the following primitive types: `String, IntX, UintX, ByStrX, ByStr or BNum`.

The type of values `vt` may be any type except a function type, this includes both builtin and user-defined algebraic data types.

Take the below example of the [non fungible contract](nonfungible) contract, where the association between a token and an address is stored.

`token_owners` maps the value between a token_id and an address. Each token_id will have an associated added when an entry is added.

`Emp` specifies when the contract is initialized the map is empty.

:::tip
`token_owners` has good logical key sense, since one addresses can own many tokens, the modelling relationship is like a primary key where token_id makes sense to be the key here.
:::

```ocaml
field token_owners: Map Uint256 ByStr20 = Emp Uint256 ByStr20
```

The below example shows some simple map functions assuming we have the `token_owners` map.

```ocaml
transition Example(key: Uint256)

  (* fetch address value using key from token_owners map *)
  value <- token_owners[key]

  (* exists is O(1) and is not recommended for large maps *)
  (* operates a traversing search over each element in token_owners for key *)
  bool_exists <- exists token_owners[key]

  (* removes key and value using key in token_owners *)
  delete token_owners[key]

  (* sets a new key, value entry in token_owners, overwrites previous value for key if any *)
  token_owners[key] := value

  (* get an option of a new map *)
  (* put has to read and copy a new map into a variable whic his not recommended for large maps*)
  option_new_map = builtin put m k v

  (* get a uint128 of the amount of items in token_owners from the read value of the map *)
  (* see below notice about how to handle counters better. *)
  token_owners_size = builtin size value;
end

```

## Map counters

If you need to track the how many items are in a large map, please read this article about [map size performance](https://scilla.readthedocs.io/en/latest/scilla-tips-and-tricks.html#field-map-size) on how to consume the size more efficiently using a counter which doesn't require a read of the whole map.

## Map size

Maps should not be considered as a way of storing large objects onchain, this will negatively affect your gas consummation when dealing with large collections of these items. It's recommended to keep maps as small as possible as gas will increase linearly with the amount of items.

## Nested maps

Take the below example of [Zilswap DEX](dex) contract, where they store a balance for a particular liquidity pool.

`balances` maps the value of the fungible contract, to a value of a nested map. The nested map contains both an account and an amount.

When using a nested map, we use the two keys of the Map, which is the fungible contract and account.

:::tip
`balances` has good logical key sense, only one fungible contract can have an account, and an account is associated with an amount.
:::

```ocaml
field balances : Map ByStr20 (Map ByStr20 Uint128) = Emp ByStr20 (Map ByStr20 Uint128)
```

The below example shows some simple map functions assuming we have the `balances` nested map.

```ocaml
transition Example(map_key: ByStr20, nested_key: ByStr20)

  (* fetch amount value using key from balances map using the fungible and the user address *)
  value <- token_owners[map_key][nested_key];

  (* removes key and value using key from balances map *)
  delete token_owners[key][nested_key];

  (* sets a new key, value entry in balances for a given map_key,
     overwrites previous value for nested_key, value if any *)
  token_owners[key][nested_key];  := value;
end
```

## Further reading
