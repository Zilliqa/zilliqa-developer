---
id: api-blockchain-get-node-type
title: GetNodeType
---

---

Returns node type. The possible return values are:

- `"Not in network, synced till epoch [epoch number]"` if the server has not joined the network and is synced until a specific epoch.
- `"Seed"` if the server is in lookup node mode and is an archival lookup node.
- `"Lookup"` if the server is in lookup node mode

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNodeType",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

### Example Response

```json
{ "id": "1", "jsonrpc": "2.0", "result": "Seed" }
```

### HTTP Request

| Chain(s)              | URL(s)                                                                                       |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **Zilliqa mainnet**   | [https://api.zilliqa.com/](https://api.zilliqa.com/)                                         |
| **Developer testnet** | [https://dev-api.zilliqa.com/](https://dev-api.zilliqa.com/)                                 |
| **Local testnet**     | [http://localhost:4201/](http://localhost:4201/)                                             |
| **Isolated server**   | [https://zilliqa-isolated-server.zilliqa.com/](https://zilliqa-isolated-server.zilliqa.com/) |

### Arguments

| Parameter | Type   | Required | Description       |
| --------- | ------ | -------- | ----------------- |
| `id`      | string | Required | `"1"`             |
| `jsonrpc` | string | Required | `"2.0"`           |
| `method`  | string | Required | `"GetNodeType"`   |
| `params`  | string | Required | Empty string `""` |
