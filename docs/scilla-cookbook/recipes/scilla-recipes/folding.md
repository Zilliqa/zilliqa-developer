---
tags:
  - folding
  - recursion
  - iterate

---

# Recursion

Recursion in Scilla takes some collection type with a beginning and end like a Natural number or the entries of a finite List and can process some logic based on that.

## forall List entries, call procedure

Lets assume we are dealing with a  ```Map of ByStr20 Uint128```. This map stores user wallets to token account values.

```ocaml
let user_map : Map ByStr20 Uint128 = Emp ByStr20 Uint128
```

We want to create some functionality to do something to these entries, our example usecase will be give every user in the Map a singular token everytime a Transition is called.

We cannot use a ```Map``` directly in forall, but we can use the builtin ```builtin to_list map_name``` to return us a list based on the current state of the map. We can use this to iterate, this will cost gas for the conversion equal to the elements being transformed from Map entry into List entry. When ```RewardEveryoneOneToken``` passes  ```user_map_as_list``` to ```GiveReward``` it passes each element as a ```Pair ByStr20 Uint128``` to which the developer then parses these types out to use them.

```ocaml
transition RewardEveryoneOneToken()
  (* read the whole map *)
  full_read_user_map <- user_map;

  (* convert each Map element to a List *)
  user_map_as_list =  builtin to_list full_read_user_map;

  (* when we iterate a MappedList, it's passed as a Pair of map_key, map_value*)
  forall user_map_as_list GiveReward;
end
```

```ocaml
let uint128_one = Uint128 1

procedure GiveReward(user_amount_pair : Pair ByStr20 Uint128)
 (* get the first value of a Pair of ByStr20 Uint128 *)
  user_address =
    let fst_user_address = @fst ByStr20 Uint128 in
      fst_user_address user_amount_pair;

 (* get the second value of a Pair of ByStr20 Uint128 *)
  user_amount =
    let snd_user_amount = @snd ByStr20 Uint128 in
      snd_user_amount user_amount_pair;

  (* user_amount += 1 *)
  user_amount_plus_rewards = builtin add user_amount uint128_one;

  (* set this value in state *)
  user_map[user_address] := user_amount_plus_rewards
end
```

## Fibonacci folding example

```ocaml
let fib = fun (n : Nat) =>
  let iter_fun =
    fun (res : Product Int Int) => fun (n: Nat) =>
      match res with
      | Pair x y => let z = builtin add x y in Pair {Int Int} z x
      end
    in
  let zero = 0 in
  let one = 1 in
  let init_val = Pair {Int Int} one zero in
  let typed_folder = @nat_fold (Product Int Int) in
  let folder = typed_folder iter_fun init_val in
  let res = folder n in
  match res with | Pair x y => x end
```

## Further reading

[TheDrBee - Recursion.scilla](https://github.com/TheDrBee/oSCILLAtor/blob/main/contracts/Recursion.scilla)
