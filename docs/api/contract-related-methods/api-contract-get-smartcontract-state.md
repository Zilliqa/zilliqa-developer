---
id: api-contract-get-smartcontract-state
title: GetSmartContractState
---

---

Returns the state (mutable) variables of a smart contract address, represented
in a JSON format.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetSmartContractState",
        "params": ["fe001824823b12b58708bf24edd94d8b5e1cfcf7"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const smartContractState = await zilliqa.blockchain.getSmartContractState(
      "fe001824823b12b58708bf24edd94d8b5e1cfcf7"
    );
    console.log(smartContractState.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            String smartContractState = client.getSmartContractState("fe001824823b12b58708bf24edd94d8b5e1cfcf7");
            System.out.println(smartContractState);
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetSmartContractState("fe001824823b12b58708bf24edd94d8b5e1cfcf7"))
    ```

=== "Go"

    ```go
    func GetSmartContractState() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetSmartContractState("fe001824823b12b58708bf24edd94d8b5e1cfcf7")
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

!!! note

    The format of response has been changed\_

```json
{
  "_balance": "0",
  "admins": {
    "0xdfa89866ae86632b36361d53b76c1373448c28fa": {
      "argtypes": [],
      "arguments": [],
      "constructor": "True"
    }
  }
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

| Parameter | Type   | Required | Description                                                                                                                                                                                       |
| --------- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`      | string | Required | `"1"`                                                                                                                                                                                             |
| `jsonrpc` | string | Required | `"2.0"`                                                                                                                                                                                           |
| `method`  | string | Required | `"GetSmartContractState"`                                                                                                                                                                         |
| `params`  | string | Required | A smart contract address of 20 bytes. Example: `"fe001824823b12b58708bf24edd94d8b5e1cfcf7"` <br/><br/> Also supports Bech32 address <br/> Example: `"zil1lcqpsfyz8vfttpcghujwmk2d3d0pel8h3qptyu"` |
