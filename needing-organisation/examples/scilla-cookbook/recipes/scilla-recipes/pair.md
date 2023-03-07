---
tags:
---

# Pair

Pairs of values are specified using the type `Pair t1 t2`, where `t1` and `t2` are types. Pairs are considered ADT's.

Take the below example, we have a library function called `build_pair` which takes two parameters, a `ByStr20` and a `String`, then creates a `Pair` of those values in the constructor declaration `{ByStr20 String}` . This is called in the public transition `Mint` where `input_pair` is of type `Pair`. The procedure `Minting` takes a Pair of `ByStr20 String` and then uses `PairUtils` to be able to use `@fst` and `@snd` to retrieve the first and second value from the Pair.

```ocaml
import PairUtils

let build_pair =
  fun (input_to_address : ByStr20) =>
  fun (input_token_uri : String) =>
    Pair {ByStr20 String} input_to_address input_token_uri

...

transition Mint(to: ByStr20, token_uri: String)
  input_pair = build_pair to token_uri;
  Minting input_pair;
  ...
end

procedure Minting(input_pair: Pair ByStr20 String)
  to = let fst_string_bystr20 = @fst ByStr20 String in
    fst_string_bystr20 input_pair;
  token_uri = let snd_string_bystr20 = @snd ByStr20 String in
    snd_string_bystr20 input_pair;
  ...
end
```

Consider the below example of `BatchMint`, it takes a single address, and a list of token URI's. `pair_token_uri_list` is defined as a `(Pair ByStr20 String)`, `tmp_func` is a temporary function to create the list and stores `to`. `input_list` then constructs, the empty Map of Pairs, the temporary function with the address, and then the List of Strings, resulting in a full map of a single address of `ByStr20` and multiple uri's of type `String`. Now with a list of `Pairs`, these can be iterated over and passed to the procedure as a single Pair.

```ocaml
transition BatchMint(to: ByStr20, token_uris_list: List String)
  pair_token_uri_list = @list_map String (Pair ByStr20 String);
  tmp_fun = build_pair to;
  input_list = pair_token_uri_list tmp_fun token_uris_list;
  forall input_list Minting;
```

## Further reading

[Scilla Documentation - Pair](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=pair#pair)
