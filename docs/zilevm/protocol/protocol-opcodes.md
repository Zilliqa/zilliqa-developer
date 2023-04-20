---
id: protocol-opcodes
title: Opcodes
keywords:
  - opcodes
description: ZILEVM Opcodes
---

---

## Opcodes

| OP code    | Description                                                                                                                                                                                                            |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| COINBASE   | Returns 0 (this opcode returns the address that gets the current block reward; since the reward is split among multiple participating parties in Zilliqa, it's not possible to implement this opcode, and we return 0) |
| CHAINID    | Returns 0x8000 + Zilliqa ChainID. Existing Zilliqa chain ids are incompatible with Ethereum ids (where 1 means mainnet), so we shift our chain id space up by 0x8000.                                                  |
| BASEFEE    | Returns the current ZIL gas price of 0.02 ZIL                                                                                                                                                                          |
| DIFFICULTY | Return current difficulty                                                                                                                                                                                              |
