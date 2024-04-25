---
id: api-blockchain-get-current-mini-epoch
title: GetCurrentMiniEpoch
---

---

Returns the current TX block number of the network. This is represented as a `String`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetCurrentMiniEpoch",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const currentMiniEpoch = await zilliqa.blockchain.getCurrentMiniEpoch();
    console.log(currentMiniEpoch.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<String> currentMiniEpoch = client.getCurrentMiniEpoch();
            System.out.println(new Gson().toJson(currentMiniEpoch));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetCurrentMiniEpoch())
    ```

=== "Go"

    ```go
    func GetCurrentMiniEpoch() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetCurrentMiniEpoch()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "589793"
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

| Parameter | Type   | Required | Description             |
| --------- | ------ | -------- | ----------------------- |
| `id`      | string | Required | `"1"`                   |
| `jsonrpc` | string | Required | `"2.0"`                 |
| `method`  | string | Required | `"GetCurrentMiniEpoch"` |
| `params`  | string | Required | Empty string `""`       |
