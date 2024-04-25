---
id: api-blockchain-get-network-id
title: GetNetworkId
---

---

Returns the `CHAIN_ID` of the specified network. This is represented as a `String`.

Our chain ids are listed at [chainlist.org](https://chainlist.org/?search=zilliqa&testnets=true).

The chain id reported by the Zilliqa API has bit 15 clear (`chain_id & ~0x8000`) whilst the chain id reported by the EVM API has bit 15 set (`chain_id | 0x8000`).

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetNetworkId",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com/"
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
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
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
    new_api = ZilliqaAPI(endpoint="https://api.zq2-devnet.zilliqa.com")
    network_id = new_api.GetNetworkId()
    print(network_id)
    ```

=== "Go"

    ```go
    func GetNetworkId() {
        provider := NewProvider("https://api.zq2-devnet.zilliqa.com/")
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

### Arguments

| Parameter | Type   | Required | Description       |
| --------- | ------ | -------- | ----------------- |
| `id`      | string | Required | `"1"`             |
| `jsonrpc` | string | Required | `"2.0"`           |
| `method`  | string | Required | `"GetNetworkId"`  |
| `params`  | string | Required | Empty string `""` |
