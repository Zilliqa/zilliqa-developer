---
id: api-transaction-get-soft-confirmed-tx
title: GetSoftConfirmedTransaction
---

---

Returns the details of a specified Transaction.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetSoftConfirmedTransaction",
        "params": ["cd8727674bc05e0ede405597a218164e1c13c7103b9d0ba43586785f3d8cede5"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "ID": "cd8727674bc05e0ede405597a218164e1c13c7103b9d0ba43586785f3d8cede5",
    "amount": "24999000000000",
    "gasLimit": "1",
    "gasPrice": "1000000000",
    "nonce": "1",
    "receipt": {
      "cumulative_gas": "1",
      "epoch_num": "589763",
      "success": true
    },
    "signature": "0x593454623A6CE0FEA287E42583445B140F696F79CA508762B8AB44F202686CFA115A2AC36C31E643C9EB0D46A4E6CA8C4EEFD78D7E9A25220DC512C13C9600F0",
    "toAddr": "9148616bfdfab321bdd626682a8c446e193eabb2",
    "version": "65537"
  }
}
```

### HTTP Request

| Chain(s)              | URL(s)                                                                                       |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **Zilliqa mainnet**   | [https://api.zilliqa.com/](https://api.zilliqa.com/)                                         |
| **Developer testnet** | [https://dev-api.zilliqa.com/](https://dev-api.zilliqa.com/)                                 |
| **Local testnet**     | [http://localhost:4201/](http://localhost:4201/)                                             |
| **Isolated server**   | [https://zilliqa-isolated-server.zilliqa.com/](https://zilliqa-isolated-server.zilliqa.com/) |

### Arguments

| Parameter | Type   | Required | Description                                              |
| --------- | ------ | -------- | -------------------------------------------------------- |
| `id`      | string | Required | `"1"`                                                    |
| `jsonrpc` | string | Required | `"2.0"`                                                  |
| `method`  | string | Required | `"GetSoftConfirmedTransaction"`                          |
| `params`  | string | Required | Transaction hash of 32 bytes of a specified transaction. |
