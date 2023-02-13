---
id: api-blockchain-get-network-id
title: GetNetworkId
---

---

Returns the `CHAIN_ID` of the specified network. This is represented as a `String`.

See table below for the `CHAIN_ID` for different chains:

| Chain(s)              | `CHAIN_ID` |
| --------------------- | ---------- |
| **Zilliqa mainnet**   | `1`        |
| **Developer testnet** | `333`      |
| **Isolated server**   | `222`      |

**NOTE:** `CHAIN_ID` from `2` to `9` are reserved for Zilliqa Core use.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNetworkId",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const NetworkId = await zilliqa.network.GetNetworkId();
    console.log(NetworkId);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<String> networkId = client.getNetworkId();
            System.out.println(new Gson().toJson(networkId));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    from pyzil.zilliqa.api import ZilliqaAPI

    # EITHER
    chain.set_active_chain(chain.MainNet)
    network_id = chain.active_chain.api.GetNetworkId()
    print(network_id)

    # OR
    new_api = ZilliqaAPI(endpoint="https://api.zilliqa.com")
    network_id = new_api.GetNetworkId()
    print(network_id)
    ```

=== "Go"

    ```go
    func GetNetworkId() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetNetworkId()
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "1"
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

| Parameter | Type   | Required | Description       |
| --------- | ------ | -------- | ----------------- |
| `id`      | string | Required | `"1"`             |
| `jsonrpc` | string | Required | `"2.0"`           |
| `method`  | string | Required | `"GetNetworkId"`  |
| `params`  | string | Required | Empty string `""` |
