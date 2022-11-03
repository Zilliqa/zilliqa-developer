---
tags:
  - natural
  - nat
  - peano

---

# Natural numbers

NAT types are natural numbers. You can start at 0 using,

```Zero```

and have a way to get the next one, i.e. its successor using,

```Succ```

```Nat``` is used as a part of [folding](folding).

```ocaml
scilla_version 0
(***************************************************)
(* The built-in ADT "Nat"                          *)
(***************************************************)
import NatUtils

contract NatType()

field counter: Nat = Zero

(* emit the current value of the counter in an event *)
procedure EmitCounterAsNumber()
  c <- counter;
  number = nat_to_int c; (* from NatUtils *)
  ev = {_eventname : "EmitCounter"; counter_value: number};
  event ev
end

(* Increase the counter: get next Peano Number *)
transition Increase()
  c <- counter;
  next = Succ c;
  counter := next;
  EmitCounterAsNumber
end

(* Decrease the counter: get previous Peano Number    *)
(* floored at Zero, i.e. there is no previous to Zero *)
transition DecreaseFlooredAtZeo()
  c <- counter;
  previous_opt = nat_prev c; (* from NatUtils *)
  previous = let p =
    match previous_opt with
    | None => (* counter was Zero, remains at Zero *)
      Zero
    | Some x =>
      x
    end in (* previous_opt *)
  p;
  counter := previous;
  EmitCounterAsNumber
end
```

## Further reading

[Haskell Peano Numbers](https://wiki.haskell.org/Peano_numbers)

[NatType.scilla](https://github.com/TheDrBee/oSCILLAtor/blob/079f2400cfa1e6fdc7a7b0449bd65406186a1f3e/contracts/NatType.scilla)

[Scilla Documentation - Nat](https://scilla.readthedocs.io/en/latest/scilla-in-depth.html?highlight=adt#nat)
