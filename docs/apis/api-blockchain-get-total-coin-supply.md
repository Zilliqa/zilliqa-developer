---
id: api-blockchain-get-total-coin-supply
title: GetTotalCoinSupply
---

---

Returns the total supply (ZIL) of coins in the network. This is represented as a `String`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTotalCoinSupply",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const totalCoinSupply = await zilliqa.network.GetTotalCoinSupply();
    console.log(totalCoinSupply);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<String> totalCoinSupply = client.getTotalCoinSupply();
            System.out.println(new Gson().toJson(totalCoinSupply));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    from pyzil.zilliqa.api import ZilliqaAPI


    # EITHER
    chain.set_active_chain(chain.MainNet)
    total_coin_supply = chain.active_chain.api.GetTotalCoinSupply()
    print(total_coin_supply)

    # OR
    new_api = ZilliqaAPI(endpoint="https://api.zilliqa.com")
    total_coin_supply = new_api.GetTotalCoinSupply()
    print(total_coin_supply)
    ```

=== "Go"

    ```go
    func GetTotalCoinSupply() {
    	provider := NewProvider("https://api.zilliqa.com/")
    	response := provider.GetTotalCoinSupply()
    	result, _ := json.Marshal(response)
    	fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "13452081092.277490607172"
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
| `method`  | string | Required | `"GetTotalCoinSupply"` |
| `params`  | string | Required | Empty string `""`      |
