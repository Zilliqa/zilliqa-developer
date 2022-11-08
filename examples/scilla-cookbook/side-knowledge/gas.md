---
tags:
  - Gas
---

# Gas

Gas is required to post a transaction to the network, the amount of gas is deterministic with contract execution complexity. A simpler transaction such as transfer of ZIL funds is less complex than say adding liquidity to a pool, just because of the complexity difference in code. Each read and write will cost more for the network to process your transaction.

## How is gas calculated

Each usage of a Scilla literal, executing an expression and statement in Scilla have a deterministic associated cost. More details can be found the [gas accounting documentation](https://github.com/Zilliqa/scilla-docs/blob/master/docs/texsources/gas-costs/gas-doc.pdf).

## What more is there to know?

```Gas Units``` is an amount of gas used to execute a particular transaction or deployment.

```Gas Limit``` is an amount of ZIL, defined as a QA amount, where if the contract execution gas cost will increase and overflow the limit. The whole transaction errors and reverts. Your gas is fully consumed if rolled back.

```Gas Price``` is an amount you will pay for a unit of Gas. If the user increased gas price by * 2 - the gas units have stayed the same, but you are now paying twice as much for those units.

## Further reading

[Gas Pricing](https://dev.zilliqa.com/docs/basics/basics-zil-gas/)

[Gas Accounting](https://github.com/Zilliqa/scilla-docs/blob/master/docs/texsources/gas-costs/gas-doc.pdf)