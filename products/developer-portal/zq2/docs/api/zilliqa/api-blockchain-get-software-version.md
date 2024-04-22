---
id: api-blockchain-get-software-version
title: GetVersion
---

---

Returns the software version of the Zilliqa node. Additionally, returns a commit id if defined.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetVersion",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com/"
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "Commit": "",
    "Version": "v9.0.1"
  }
}
```

### Arguments

| Parameter | Type   | Required | Description       |
| --------- | ------ | -------- | ----------------- |
| `id`      | string | Required | `"1"`             |
| `jsonrpc` | string | Required | `"2.0"`           |
| `method`  | string | Required | `"GetVersion"`    |
| `params`  | string | Required | Empty string `""` |
