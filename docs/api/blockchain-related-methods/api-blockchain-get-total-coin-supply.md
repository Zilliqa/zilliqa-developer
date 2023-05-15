---
id: api-blockchain-get-total-coin-supply
title: GetTotalCoinSupply
---

---

There are two variations of the API - `GetTotalCoinSupply` and `GetTotalCoinSupplyAsInt`.

`GetTotalCoinSupply` Returns the total supply (ZIL) of coins in the network. This is represented as a
`String`.

`GetTotalCoinSupplyAsInt` Returns the total supply (ZIL) of coins in the network. This is represented as a
`Rounded Number`.

### Example Request-1

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
    const totalCoinSupply = await zilliqa.blockchain.getTotalCoinSupply();
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

### Example Response-1

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "13452081092.277490607172"
}
```

### Example Request-2

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTotalCoinSupplyAsInt",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

### Example Response-2

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": 13452081092
}
```

!!! note

    `GetTotalCoinSupplyAsInt` is not avaliable to call through SDKs.

### HTTP Request

| Chain(s)              | URL(s)                                                                                       |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **Zilliqa mainnet**   | [https://api.zilliqa.com/](https://api.zilliqa.com/)                                         |
| **Developer testnet** | [https://dev-api.zilliqa.com/](https://dev-api.zilliqa.com/)                                 |
| **Local testnet**     | [http://localhost:4201/](http://localhost:4201/)                                             |
| **Isolated server**   | [https://zilliqa-isolated-server.zilliqa.com/](https://zilliqa-isolated-server.zilliqa.com/) |

### Arguments

| Parameter | Type   | Required | Description                                       |
| --------- | ------ | -------- | ------------------------------------------------- |
| `id`      | string | Required | `"1"`                                             |
| `jsonrpc` | string | Required | `"2.0"`                                           |
| `method`  | string | Required | `"GetTotalCoinSupply or GetTotalCoinSupplyAsint"` |
| `params`  | string | Required | Empty string `""`                                 |
