---
id: api-blockchain-get-prev-difficulty
title: GetPrevDifficulty
---

---

Returns the minimum shard difficulty of the previous block. This is represented as an `Number`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetPrevDifficulty",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const prevDifficulty = await zilliqa.blockchain.getPrevDifficulty();
    console.log(prevDifficulty.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<Integer> prevDifficulty = client.getPrevDifficulty();
            System.out.println(new Gson().toJson(prevDifficulty));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetPrevDifficulty())
    ```

=== "Go"

    ```go
    func GetPrevDifficulty() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetPrevDifficulty()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 91
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
| `method`  | string | Required | `"GetPrevDifficulty"` |
| `params`  | string | Required | Empty string `""`     |
