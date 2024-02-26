# Error checking

Error checking in Scilla is crucial for ensuring that contracts behave as
intended, especially when handling crucial operations involving assets. Proper
error checking can prevent unintended behavior and mitigate potential
vulnerabilities.

## Panic

In Scilla, the `panic` function allows a contract to halt execution immediately,
without making any state changes. This can be useful when a condition is met
that should stop the execution of the contract immediately.

Example:

```scilla
scilla_version 0

library PanicContract

contract PanicExample()

transition TriggerPanic()
  panic "This is a panic message.";
end
```

In the above contract, calling the `TriggerPanic` transition will always cause
the contract to halt execution with the provided panic message.

## Assert

The `assert` function in Scilla checks a given condition, and if it evaluates to
`False`, the contract execution halts immediately without making any state
changes.

Example:

```scilla
scilla_version 0

library AssertContract

contract AssertExample()

transition AssertNonZero(value: Uint128)
  is_non_zero = builtin lt value Uint128 1;
  assert is_non_zero;
  e = { _eventname : "AssertionPassed"; value : value };
  event e
end
```

In the above contract, the `AssertNonZero` transition checks if the provided
`value` is not zero using the `assert` function. If the `value` is zero, the
execution will halt.
