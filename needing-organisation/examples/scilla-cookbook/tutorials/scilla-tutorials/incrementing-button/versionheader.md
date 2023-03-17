---
sidebar_position: 3
---

# Version headers

## Scilla version

Our previous contract had no defined Scilla version and therefore wouldn't be valid if we tried to compile it.

:::caution
If you tried to compile the contract in it's current state you would be returned the error `Scilla version number unspecified`.
:::

To fix this, we declare the current Scilla version using the `scilla_version` keyword. The variable appended is the Scilla version we are implementing in our contract.
Currently we are using `scilla_version 0`

```ocaml {1}
scilla_version 0

contract IncrementingButton
(
  contract_owner: ByStr20
)
```

## User defined libraries

Our previous contract had no user library defined and therefore wouldn't be valid if we tried to compile it.

:::caution
If you tried to compile the previous contract, you would be returned the error `After specifying a Scilla version number, either imports, a library or contract is expected.`
:::

To fix this, we declare the current library we are defining using the `library` keyword. The variable appended is the library name we are implementing in our contract.

We will use our contract name for easier association, `library MyFirstContract`.

In the library section, we will put global variables and functions which define our contract. We can refer to these from another contract.

:::note
A Scilla file with only a library definition is considered valid Scilla and is referred to as a .scillib
:::

```ocaml {3}
scilla_version 0

library IncrementingButton

contract IncrementingButton
(
  contract_owner: ByStr20
)
```

:::tip
`IncrementingButton` now successfully compiles!
:::

## Builtin internal libraries

The Scilla standard library contains five libraries: `BoolUtils.scilla`, `IntUtils.scilla`, `ListUtils.scilla`, `NatUtils.scilla` and `PairUtils.scilla.` As the names suggests these contracts implement utility operations for the Bool, Int, List, Nat and Pair types, respectively.

To use functions from the standard library in a contract, the relevant library file must be imported using the `import` keyword followed by the library to be imported. To import more than one library, separate each library with a whitespace character.

Scilla supports deploying both libraries and contract, and each library or contract can import already deployed libraries.

:::note
We will omit the internal libraries from this IncrementingButton as they are not required.
:::

```ocaml {3}
scilla_version 0

import ListUtils IntUtils

library IncrementingButton

contract IncrementingButton
(
  contract_owner: ByStr20
)
```
