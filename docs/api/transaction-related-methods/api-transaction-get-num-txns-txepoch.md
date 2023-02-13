---
id: api-transaction-get-num-txns-txepoch
title: GetNumTxnsTxEpoch
---

---

Returns the number of validated transactions included in this Transaction epoch. This is represented as a `String`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNumTxnsTxEpoch",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const numTxnsTxEpoch = await zilliqa.blockchain.getNumTxnsTxEpoch();
    console.log(numTxnsTxEpoch.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<String> numTxnsTxEpoch = client.getNumTxnsTxEpoch();
            System.out.println(new Gson().toJson(numTxnsTxEpoch));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetNumTxnsTxEpoch())
    ```

=== "Go"

    ```go
    func GetNumTxnsTxEpoch() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetNumTxnsTxEpoch()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "38"
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

| Parameter | Type   | Required | Description           |
| --------- | ------ | -------- | --------------------- |
| `id`      | string | Required | `"1"`                 |
| `jsonrpc` | string | Required | `"2.0"`               |
| `method`  | string | Required | `"GetNumTxnsTxEpoch"` |
| `params`  | string | Required | Empty string `""`     |
