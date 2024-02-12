---
id: api-blockchain-get-sharding-structure
title: GetShardingStructure
---

---

Retrieves the sharding structure from the lookup server.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetShardingStructure",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

### Example Response

```json
{ "id": "1", "jsonrpc": "2.0", "result": { "NumPeers": [0] } }
```

### HTTP Request

| Chain(s)              | URL(s)                                                                                       |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **Zilliqa mainnet**   | [https://api.zilliqa.com/](https://api.zilliqa.com/)                                         |
| **Developer testnet** | [https://dev-api.zilliqa.com/](https://dev-api.zilliqa.com/)                                 |
| **Local testnet**     | [http://localhost:4201/](http://localhost:4201/)                                             |
| **Isolated server**   | [https://zilliqa-isolated-server.zilliqa.com/](https://zilliqa-isolated-server.zilliqa.com/) |

### Arguments

| Parameter | Type   | Required | Description              |
| --------- | ------ | -------- | ------------------------ |
| `id`      | string | Required | `"1"`                    |
| `jsonrpc` | string | Required | `"2.0"`                  |
| `method`  | string | Required | `"GetShardingStructure"` |
| `params`  | string | Required | Empty string `""`        |
