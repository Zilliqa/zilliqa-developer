---
id: api-contract-get-smartcontract-substate
title: GetSmartContractSubState
---

---

Returns the state (or a part specified) of a smart contract address, represented
in a JSON format.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetSmartContractSubState",
        "params": ["fe001824823b12b58708bf24edd94d8b5e1cfcf7","admins",[]]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const smartContractState = await zilliqa.blockchain.getSmartContractSubState(
      "fe001824823b12b58708bf24edd94d8b5e1cfcf7"
    );
    console.log(smartContractState.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            List<Object> param = new ArrayList<>();
            param.add("9611c53BE6d1b32058b2747bdeCECed7e1216793");
            param.add("admins");
            param.add(new ArrayList<>());
            String state = client.getSmartContractSubState(param);
            System.out.println(state);
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetSmartContractSubState("fe001824823b12b58708bf24edd94d8b5e1cfcf7","admins",[]))
    ```

=== "Go"

    ```go
    func GetSmartContractSubState() {
        provider := NewProvider("https://zilliqa.com")
        response, _ := provider.GetSmartContractSubState("9611c53BE6d1b32058b2747bdeCECed7e1216793", "admins", []interface{}{})
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
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

| Parameter | Type   | Required | Description                  |
| --------- | ------ | -------- | ---------------------------- |
| `id`      | string | Required | `"1"`                        |
| `jsonrpc` | string | Required | `"2.0"`                      |
| `method`  | string | Required | `"GetSmartContractSubState"` |
| `params`  | array  | Required | State params                 |

### State Params

| Parameter       | Type       | Required     | Description                                                           |
| --------------- | ---------- | ------------ | --------------------------------------------------------------------- |
| `Address`       | string     | Required     | A smart contract address of 20 bytes.                                 |
| `Variable Name` | string     | Can be empty | Name of the variable in the Smart Contract                            |
| `Indices`       | JSON Array | Can be empty | If the variable is of map type, you can specify an index (or indices) |

The `params` is a JSON array.

Example:
`"params"`:`["fe001824823b12b58708bf24edd94d8b5e1cfcf7","admins",[\"0x9bfec715a6bd658fcb62b0f8cc9bfa2ade71434a\""]]`

!!! note

    If Variable Name and Indices Array are both empty, the response would be same as `GetSmartContractState_`
