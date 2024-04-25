---
id: zil-balance
title: ZIL Balance
keywords:
  - zil
  - wallet
  - balance
description: Native ZIL balance of a given wallet
---

---

Get the ZIL balance oof a wallet.

### Parameters

| Name     | Type   | Required | Description                      |
| -------- | ------ | -------- | -------------------------------- |
| `wallet` | String | Required | Wallet address holding the token |
| `token`  | String | Required | Use zero address for native ZIL  |

### Example Request

=== "Graphql"

    #### Query

    ```graphql
    query UserBalance($input: WalletBalanceInput) {
      getUserBalanceByToken(input: $input) {
        tokenAddress
        walletAddress
        lastBlockID
        amount
      }
    }
    ```

    #### Query Variables

    ```graphql
    {
      "input": {
        "wallet": "0x22b251cc155ac0a181a156aaec74e964a82011c1",
        "token": "0x0000000000000000000000000000000000000000"
      }
    }
    ```

    #### HTTP Headers

    ```graphql
    {
      "Authorization": "Bearer <insert token>"
    }
    ```

=== "cURL"

    ```curl

    ```

### Example Response

```json
{
  "data": {
    "getUserBalanceByToken": {
      "tokenAddress": "0x0000000000000000000000000000000000000000",
      "walletAddress": "0x22b251cc155ac0a181a156aaec74e964a82011c1",
      "lastBlockID": "2513226",
      "amount": "2226027441580000"
    }
  }
}
```
