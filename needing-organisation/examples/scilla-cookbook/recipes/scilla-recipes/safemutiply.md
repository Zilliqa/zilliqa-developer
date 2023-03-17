---
tags:
  - option
  - optional
  - some
  - none
  - divide
  - percentage
---

# Safe multiplication

When your are working with numbers in any programming language you are subject to overflows. An overflow happens when the value of number is bigger than the maximum possible value offered by the machine.

For example if you’re using an unsigned integer in Scilla. The possible values of your variable ranges from 0 to 2^256 (1.1579209e+77). So it means that if you’re around the maximum value and increment your variable it will error as it overflows the maximum value range.

If developers want to multiple two `Uint128` together, they should firstly upcast their current values into `Uint256` so it doesn't overflow, and then perform the multiplication, finally casting the value to a `Uint128` after performing the division.

## Safe percentage

If you are working with trying to find the percentage value of a users share of a pool and then reward them based on that percentage, then consider using safe `muldiv`.

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

Lets assume we are A. The pool has the following entries.

- A 12
- B 3
- C 9
- D 69

We can deduce and reason the following.

- A own's 12
- The total of all the entries is 93
- 12 is x percentage of 93 = 12.9
- If the total rewards is 100 Zil or 100000000000000 QA
- Then A is owed 12.9 Zil or 12.9 Zil QA

```ocaml
user_share = 12
total_amount_rewards = 100000000000000qa
sum_of_all_shares = 93
```

Usage of `muldiv`

```ocaml
user_share_of_rewards = muldiv user_share total_amount_rewards sum_of_all_shares;
```

Test this out on isolated environment `0xd3360fe70a19dc2dd5cb7ad4164db455ddc2a68c`

## Further reading

[SSN Staking Contract](https://github.com/Zilliqa/staking-contract/blob/82fad745a04eedefb1a0cd16e5316626c3736c13/contracts/ssnlist.scilla)
