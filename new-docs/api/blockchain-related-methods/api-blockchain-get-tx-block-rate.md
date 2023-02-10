---
id: api-blockchain-get-tx-block-rate
title: GetTxBlockRate
---

---

Returns the current Transaction blockrate per second for the network.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTxBlockRate",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const txBlockRate = await zilliqa.blockchain.getTxBlockRate();
    console.log(txBlockRate.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<Double> txBlockRate = client.getTxBlockRate();
            System.out.println(new Gson().toJson(txBlockRate));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetTxBlockRate())
    ```

=== "Go"

    ```go
    func GetTxBlockRate() {
    	provider := NewProvider("https://api.zilliqa.com/")
    	response := provider.GetTxBlockRate()
    	result, _ := json.Marshal(response)
    	fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 0.014138050978963283
}
```

### HTTP Request

| Chain(s)              | URL(s)                                       |
| --------------------- | -------------------------------------------- |
| **Zilliqa mainnet**   | https://api.zilliqa.com/                     |
| **Developer testnet** | https://dev-api.zilliqa.com/                 |
| **Local testnet**     | http://localhost:4201/                       |
| **Isolated server**   | https://zilliqa-isolated-server.zilliqa.com/ |

### Arguments

| Parameter | Type   | Required | Description        |
| --------- | ------ | -------- | ------------------ |
| `id`      | string | Required | `"1"`              |
| `jsonrpc` | string | Required | `"2.0"`            |
| `method`  | string | Required | `"GetTxBlockRate"` |
| `params`  | string | Required | Empty string `""`  |
