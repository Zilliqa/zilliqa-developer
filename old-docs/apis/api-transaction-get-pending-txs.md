---
id: api-transaction-get-pending-txs
title: GetPendingTxns
---

---

Returns the status (code) of all unconfirmed Transactions.

Please refer to [GetPendingTxn](api-transaction-get-pending-tx#api-availability) for details on API availability and status codes.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetPendingTxns",
        "params": []
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const pendingTransaction = await zilliqa.blockchain.getPendingTxns();
    console.log(pendingTransaction.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<PendingStatus> pengdingStatus = client.getPendingTxns();
            System.out.println(new Gson().toJson(pengdingStatus));
        }
    }
    ```

=== "Go"

    ```go
    func GetPendingTxns() {
      provider := NewProvider("https://api.zilliqa.com/")
      response := provider.GetPendingTxns()
      result, _ := json.Marshal(response)
      fmt.Println(string(result))
    }
    ```

### Example Response

#### Since Zilliqa V6.3.0

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "Txns": [
      {
        "TxnHash": "ec5ef8110a285563d0104269081aa77820058067091a9b3f3ae70f38b94abda3",
        "code": 1
      },
      {
        "TxnHash": "cf546d80fa2e0cc0b5b8f9fbb639050fe292d1601aa5d4a7c48106c624311bf9",
        "code": 24
      }
    ]
  }
}
```

#### Zilliqa V6.2.0 and before

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "Txns": [
      {
        "Status": 1,
        "TxnHash": "ec5ef8110a285563d0104269081aa77820058067091a9b3f3ae70f38b94abda3"
      }
    ]
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

| Parameter | Type   | Required | Description        |
| --------- | ------ | -------- | ------------------ |
| `id`      | string | Required | `"1"`              |
| `jsonrpc` | string | Required | `"2.0"`            |
| `method`  | string | Required | `"GetPendingTxns"` |
| `params`  | None   |          |                    |
