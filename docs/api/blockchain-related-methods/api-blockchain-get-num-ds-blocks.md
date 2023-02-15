---
id: api-blockchain-num-ds-blocks
title: GetNumDSBlocks
---

---

Returns the current number of validated Directory Service blocks in the network. This is represented as a `String`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNumDSBlocks",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const numDsBlock = await zilliqa.blockchain.getNumDSBlocks();
    console.log(numDsBlock.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<String> numDSBlocks = client.getNumDSBlocks();
            System.out.println(new Gson().toJson(numDSBlocks));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetNumDSBlocks())
    ```

=== "Go"

    ```go
    func GetNumDSBlocks() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetNumDSBlocks()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "5899"
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
| `method`  | string | Required | `"GetNumDSBlocks"` |
| `params`  | string | Required | Empty string `""`  |
