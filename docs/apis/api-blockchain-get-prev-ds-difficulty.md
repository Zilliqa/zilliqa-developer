---
id: api-blockchain-get-prev-ds-difficulty
title: GetPrevDSDifficulty
---

---

Returns the minimum DS difficulty of the previous block. This is represented as an `Number`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetPrevDSDifficulty",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const prevDSDifficulty = await zilliqa.blockchain.getPrevDSDifficulty();
    console.log(prevDSDifficulty.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<Integer> prevDSDifficulty = client.getPrevDSDifficulty();
            System.out.println(new Gson().toJson(prevDSDifficulty));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetPrevDSDifficulty())
    ```

=== "Go"

    ```go
    func GetPrevDSDifficulty() {
    	provider := NewProvider("https://api.zilliqa.com/")
    	response := provider.GetPrevDSDifficulty()
    	result, _ := json.Marshal(response)
    	fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 149
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

| Parameter | Type   | Required | Description             |
| --------- | ------ | -------- | ----------------------- |
| `id`      | string | Required | `"1"`                   |
| `jsonrpc` | string | Required | `"2.0"`                 |
| `method`  | string | Required | `"GetPrevDSDifficulty"` |
| `params`  | string | Required | Empty string `""`       |
