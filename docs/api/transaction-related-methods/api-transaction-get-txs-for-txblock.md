---
id: api-transaction-get-txs-for-txblock
title: GetTransactionsForTxBlock
---

---

Returns the validated transactions included within a specified final transaction block as an array of length _i_, where _i_ is the number of shards plus the DS committee. The transactions are grouped based on the group that processed the transaction. The first element of the array refers to the first shard. The last element of the array at index, _i_, refers to the transactions processed by the DS Committee.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTransactionsForTxBlock",
        "params": ["2"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const txns = await zilliqa.blockchain.getTransactionsForTxBlock("2");
    console.log(txns.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com");
            Rep<List<List<String>>> transactionList = client.getTransactionsForTxBlock("2");
            System.out.println(new Gson().toJson(transactionList));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetTransactionsForTxBlock("2"))
    ```

=== "Go"

    ```go
    func GetTransactionsForTxBlock() {
        provider := NewProvider("https://api.zilliqa.com/")
        response := provider.GetTransactionsForTxBlock("1")
        result, _ := json.Marshal(response)
        fmt.Println(string(result))
    }
    ```

### Example Response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": [
    [
      "2398362e23635582ed58f83dbcff7af2d8ccb017f6ff2bb49d343e7b8bb8bd68",
      "3f337358c07c4e984714da804985f23eca9a9dd14aa8ba1ddd89583cf5110bf0",
      "35823ae3377b91792fa34fa5577fa267385374e08da51555f63a537942d5adb6",
      "04e5f20de988a4afea17408c87a8d4f73d14082f13df552cce849e4ddd4cfffc"
    ],
    [
      "5830a93aafe6571099aa38e99218c4495a2af73d481a28aba8a34c45768d0fb9",
      "9250ce07210b75ef8ec5fcf42f3b5afa4cd4b60414b338be0caddcfb316293cf",
      "60fe6307f27e084bfb84ff5b6cafcbb05e1bc450d1b67d9102d57066d931ba7f",
      "92562be5d4fd4b39ea44f22e010636163b6500561ddab58aca0a90ac7c11f04c"
    ],
    [
      "9cfe6d32b31cf31267bc46b2a99f0b243266f4842d140dab5b3ee31369ae9926",
      "3526b2b8f226fff6643c60deea71129dcd98c320521c8e96715e2f02c651a081",
      "477a9c79acaa9aa2060b9d21f2e01760a31499b32723e0e2e1cb2cb8c4e4be7e",
      "a1fa8ec4253c8ad125b81f8ee952f4e12abc445c80f56d85e18a9246541b7f37",
      "251ce0e4a60be05363cad225b61f48ae4dd017230b0b3c58c7257239ec51aa09",
      "452471a31af62ecd48cadc1536a4fef3b7ac243dd1023d8f9a12b1448f096c69",
      "a5b1c0354433304ca6a3d3bc95eb41bae856b8aea5eb6d7ea28fff19e4b1033b"
    ],
    [
      "adba475e9ae7419a91107987c93838ac72c305937c5683aecee2d98024002eca",
      "0f0cf6f5e4ba6ed7302db5b00f958b305e33d28e5a5a7297f87f51307b59aa82",
      "d05be491318e6bbadb4705d436daf0c46762de1e14bf8d4794ff34782584f027",
      "2157babdc5c65e7b4ca5e774782119d811d54d4dcc9b64a176120f1ac3c73c1c",
      "dafc9b289def10232da12efcc6fa37a142982c832357628e830e616e9663501e"
    ]
  ]
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

| Parameter | Type   | Required | Description                                        |
| --------- | ------ | -------- | -------------------------------------------------- |
| `id`      | string | Required | `"1"`                                              |
| `jsonrpc` | string | Required | `"2.0"`                                            |
| `method`  | string | Required | `"GetTransactionsForTxBlock"`                      |
| `params`  | string | Required | Specifed TX block number to return. Example: `"2"` |
