---
id: api-transaction-get-pending-tx
title: GetPendingTxn
---

---

Returns the status (code) of a specified unconfirmed Transaction.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetPendingTxn",
        "params": ["b9e545ab3ed0b61a4d326425569605255e0990da7dda18b4658fdb17b390844e"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const pendingTransaction = await zilliqa.blockchain.getPendingTxn(txId);
    console.log(pendingTransaction.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<PendingStatus> pengdingStatus = client.getPendingTxn("b9e545ab3ed0b61a4d326425569605255e0990da7dda18b4658fdb17b390844e");
            System.out.println(new Gson().toJson(pengdingStatus));
        }
    }
    ```

=== "Go"

    ```go
    func GetPendingTxn() {
      provider := NewProvider("https://api.zilliqa.com/")
      response := provider.GetPendingTxn("2cf109b25f2132c08a4248e2be8add6b95b92aef5b2c77e737faefbc9353ee7c")
      result, _ := json.Marshal(response)
      fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "code": 24,
    "confirmed": false,
    "pending": false
  }
}
```

### API Availability

Please note that the status of newly created transactions (using `CreateTransaction`) may not immediately be available for checking using this API.

A created transaction will be included in this API if:

1. It has already been dispatched to the network (this may require one transaction epoch)
2. The network has acknowledged receiving this transaction (this occurs at the end of every transaction epoch)

Hence, we recommend calling `GetPendingTxn` around 1-2 transaction epochs after transaction creation to get accurate results.

### Status Codes

Refer to [GetTransactionStatus](https://dev.zilliqa.com/docs/apis/api-transaction-get-transaction-status#status-codes) for the status codes.

_Note: Dropped transactions are available for querying for up to 5 transaction epochs only._

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
| `method`  | string | Required | `"GetPendingTxn"`                                        |
| `params`  | string | Required | Transaction hash of 32 bytes of a specified transaction. |
