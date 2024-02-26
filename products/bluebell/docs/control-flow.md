# Control flow

## match

The `match` construct is used for pattern matching and in place of `if`
statements. It's especially useful for handling ADTs (Algebraic Data Types) and
Options.

Here's a simple example of using `match` with an ADT:

```scilla
scilla_version 0

library MatchExample

type Animal =
| Dog
| Cat
| Elephant

contract AnimalSound()

transition GetSound(animal: Animal)
  sound =
    match animal with
    | Dog => "Bark"
    | Cat => "Meow"
    | Elephant => "Trumpet"
    end;

  e = { _eventname : "AnimalSound"; sound : sound };
  event e
end
```

Here's another example of using `match` to handle an Option type:

```scilla
scilla_version 0

library OptionMatchExample

contract HandleOption()

transition GetNumber(opt: Option Uint32)
  result =
    match opt with
    | Some n => n
    | None => Uint32 0
    end;

  e = { _eventname : "NumberResult"; number : result };
  event e
end
```
