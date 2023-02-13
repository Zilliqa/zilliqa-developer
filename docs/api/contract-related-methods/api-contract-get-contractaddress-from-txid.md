---
id: api-contract-get-contractaddress-from-txid
title: GetContractAddressFromTransactionID
---

---

Returns a smart contract address of 20 bytes. This is represented as a `String`.

**NOTE:** This only works for contract deployment transactions.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetContractAddressFromTransactionID",
        "params": ["AAF3089596437A7C6984FA2627B6F38B5F5B80FAEAAC6993C2E82C6A8EE2615E"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const contractAddress =
      await zilliqa.blockchain.getContractAddressFromTransactionID(
        "AAF3089596437A7C6984FA2627B6F38B5F5B80FAEAAC6993C2E82C6A8EE2615E"
      );
    console.log(contractAddress.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<String> contractAddress = client.getContractAddressFromTransactionID("AAF3089596437A7C6984FA2627B6F38B5F5B80FAEAAC6993C2E82C6A8EE2615E");
            System.out.println(new Gson().toJson(contractAddress));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetContractAddressFromTransactionID(
         "AAF3089596437A7C6984FA2627B6F38B5F5B80FAEAAC6993C2E82C6A8EE2615E"
    ))
    ```

=== "Go"

    ```go
    func GetContractAddressFromTransactionID() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetContractAddressFromTransactionID("AAF3089596437A7C6984FA2627B6F38B5F5B80FAEAAC6993C2E82C6A8EE2615E")
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "c458f39c106582c1a49bac6bc76ec603e2ae0497"
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

| Parameter | Type   | Required | Description                                                                                                       |
| --------- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------- |
| `id`      | string | Required | `"1"`                                                                                                             |
| `jsonrpc` | string | Required | `"2.0"`                                                                                                           |
| `method`  | string | Required | `"GetSmartContracts"`                                                                                             |
| `params`  | string | Required | A Transaction ID of 32 bytes. <br/> Example: `"AAF3089596437A7C6984FA2627B6F38B5F5B80FAEAAC6993C2E82C6A8EE2615E"` |
