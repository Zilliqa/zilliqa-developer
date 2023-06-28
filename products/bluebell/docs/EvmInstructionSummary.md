The list below is not complete. See https://github.com/wolflo/evm-opcodes

1. `STOP`: Halts execution.
2. `ADD`: Addition operation.
3. `MUL`: Multiplication operation.
4. `SUB`: Subtraction operation.
5. `DIV`: Integer division operation.
6. `SDIV`: Signed integer division operation.
7. `MOD`: Modulo remainder operation.
8. `SMOD`: Signed modulo remainder operation.
9. `ADDMOD`: Modulo addition operation.
10. `MULMOD`: Modulo multiplication operation.
11. `EXP`: Exponential operation.
12. `SIGNEXTEND`: Extend length of two's complement signed integer.
13. `LT`: Less-than comparison.
14. `GT`: Greater-than comparison.
15. `SLT`: Signed less-than comparison.
16. `SGT`: Signed greater-than comparison.
17. `EQ`: Equality comparison.
18. `ISZERO`: Simple not operator.
19. `AND`: Bitwise AND operation.
20. `OR`: Bitwise OR operation.
21. `XOR`: Bitwise XOR operation.
22. `NOT`: Bitwise NOT operation.
23. `BYTE`: Retrieve single byte from word.
24. `SHA3`: Compute Keccak-256 hash.
25. `ADDRESS`: Get address of currently executing account.
26. `BALANCE`: Get balance of the given account.
27. `ORIGIN`: Get execution origination address.
28. `CALLER`: Get caller address.
29. `CALLVALUE`: Get deposited value by the instruction/transaction responsible
    for this execution.
30. `CALLDATALOAD`: Get input data of current environment.
31. `CALLDATASIZE`: Get size of input data in current environment.
32. `CALLDATACOPY`: Copy input data in current environment to memory.
33. `CODESIZE`: Get size of code running in current environment.
34. `CODECOPY`: Copy code running in current environment to memory.
35. `GASPRICE`: Get price of gas in current environment.
36. `EXTCODESIZE`: Get size of an account's code.
37. `EXTCODECOPY`: Copy an account's code to memory.
38. `RETURNDATASIZE`: Get size of output data from the previous call.
39. `RETURNDATACOPY`: Copy output data from the previous call to memory.
40. `BLOCKHASH`: Get the hash of one of the 256 most recent complete blocks.
41. `COINBASE`: Get the block's beneficiary address.
42. `TIMESTAMP`: Get the block's timestamp.
43. `NUMBER`: Get the block's number.
44. `DIFFICULTY`: Get the block's difficulty.
45. `GASLIMIT`: Get the block's gas limit.
46. `POP`: Remove item from stack.
47. `MLOAD`: Load word from memory.
48. `MSTORE`: Save word to memory.
49. `MSTORE8`: Save byte to memory.
50. `SLOAD`: Load word from storage.
51. `SSTORE`: Save word to storage.
52. `JUMP`: Alter the program counter.
53. `JUMPI`: Conditionally alter the program counter.
54. `PC`: Get the value of the program counter prior to the increment.
55. `MSIZE`: Get the size of active memory in bytes.
56. `GAS`: Get the amount of available gas, including the corresponding
    reduction for the cost of this instruction.
57. `JUMPDEST`: Mark a valid destination for jumps.
58. `PUSHn` (for 1 <= n <= 32): Place n-byte item on stack.
59. `DUPn` (for 1 <= n <= 16): Duplicate n-th stack item.
60. `SWAPn` (for 1 <= n <= 16): Exchange 1st and (n+1)-th stack items.
61. `LOGn` (for 0 <= n <= 4): Append log record with n topics.
62. `CREATE`: Create a new account with associated code.
63. `CALL`: Message-call into an account.
64. `CALLCODE`: Message-call into this account with alternative account's code.
65. `RETURN`: Halt execution returning output data.
66. `DELEGATECALL`: Message-call into this account with an alternative account’s
    code, but persisting the current values for `sender` and `value`.
67. `CREATE2`: Create a new account with associated code at a defined address.
68. `STATICCALL`: Static message-call into an account.
69. `REVERT`: Halt execution reverting state changes but returning data and
    remaining gas.
70. `SELFDESTRUCT`: Halt execution and register account for later deletion.
