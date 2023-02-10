---
id: api-blockchain-get-tx-rate
title: GetTransactionRate
---

---

Returns the current Transaction rate per second **(TPS)** of the network. This is represented as an `Number`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTransactionRate",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const transactionRate = await zilliqa.blockchain.getTransactionRate();
    console.log(transactionRate.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<Integer> transactionRate = client.getTransactionRate();
            System.out.println(new Gson().toJson(transactionRate));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetTransactionRate())
    ```

=== "Go"

    ```go
    func GetTransactionRate() {
    	provider := NewProvider("https://api.zilliqa.com/")
    	response := provider.GetTransactionRate()
    	result, _ := json.Marshal(response)
    	fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 9.169180550334216
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

| Parameter | Type   | Required | Description            |
| --------- | ------ | -------- | ---------------------- |
| `id`      | string | Required | `"1"`                  |
| `jsonrpc` | string | Required | `"2.0"`                |
| `method`  | string | Required | `"GetTransactionRate"` |
| `params`  | string | Required | Empty string `""`      |
