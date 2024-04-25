---
id: protocol-interwork
title: Scilla/EVM interwork
keywords:
  - interwork
  - scilla
  - evm
description: Scilla/EVM interwork
---

---

## Scilla/EVM interwork

The current (v8.9) facilities for Scilla/EVM interwork are fairly
primitive; just enough to build the ERC20 -> ZRC2 gateway contract you
can find in the
[zilliqa-developer](https://github.com/zilliqa/zilliqa-developer)
repository. We hope to improve on these in future versions.

They consist of precompiles to call Scilla from EVM (either replacing
or retaining the `sender`), and a precompile to read a fairly
restricted subset of Scilla state from EVM.

!!! info

    This documentation is being prepared - please check back in a couple of days.
