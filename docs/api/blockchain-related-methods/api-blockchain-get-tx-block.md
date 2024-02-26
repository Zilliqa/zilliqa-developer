---
id: api-blockchain-get-tx-block
title: GetTxBlock
---

---

There are two variations of the API - `GetTsBlock` and `GetTsBlockVerbose`.

Returns the details of a specified Transaction block. In verbose mode, additional information will be included in the response.

### Example Request-1

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTxBlock",
        "params": ["1002353"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

=== "Node.js"

    ```js
    const txBlock = await zilliqa.blockchain.getTxBlock("40");
    console.log(txBlock.result);
    ```

=== "Java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zilliqa.com/");
            Rep<TxBlock> txBlock = client.getTxBlock("40");
            System.out.println(new Gson().toJson(txBlock));
        }
    }
    ```

=== "Python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetTxBlock("40"))
    ```

=== "Go"

    ```go
    func GetTxBlock(t *testing.T) {
      provider := NewProvider("https://api.zilliqa.com/")
      response := provider.GetTxBlock("40")
      result, _ := json.Marshal(response)
      fmt.Println(string(result))
    }
    ```

### Example Response-1

!!! note

    From Zilliqa `V7.2.0` onwards, an additional `NumPages` field is
    included in the `header` response section. This field is used by
    [GetTransactionsForTxBlockEx](../transaction-related-methods/api-transaction-get-txs-for-txblock-ex.md) and
    [GetTxnBodiesForTxBlockEx](../transaction-related-methods/api-transaction-get-txbodies-for-txblock-ex.md).

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "body": {
      "BlockHash": "53a24881823dd5f2a3dfda5902d1b79710e2bec5477ed3aa7325d74e30436b58",
      "HeaderSign": "8E0C73945CC2282173CF8CF44D7EB55E5DAD9B2D6D3437C6AC09DE8CF0D6B698575E535168AA898B6B3A3107603BDFC4BC671A4621E77C9004369FC3513F53A0",
      "MicroBlockInfos": [
        {
          "MicroBlockHash": "ebadc2d6e80b749e6e322ae54467d516618ea79d1ae495f26f3592c70b4de71a",
          "MicroBlockShardId": 0,
          "MicroBlockTxnRootHash": "165049b84c5f4499ce781aab63cba06aa31ed4e1b556f0aac643f01eb5814da4"
        },
        {
          "MicroBlockHash": "7111f32a526a381ecb3492e21a382f2dc5ad10c346340aaae3addd1a349cc559",
          "MicroBlockShardId": 1,
          "MicroBlockTxnRootHash": "640a7019993fcdaec2bfd10b50f5f9faea92920a1a4c0cb931ae56e061f983d9"
        },
        {
          "MicroBlockHash": "1a914f52aaef51fa3d585c666e56ae55c2dc5e3b8c759c66d1b79b211b783d0e",
          "MicroBlockShardId": 2,
          "MicroBlockTxnRootHash": "aea9eafc983f75947ef63d0aedd14c0c138025cbbaa5934f3ef327b2116bfd68"
        },
        {
          "MicroBlockHash": "cf095207f2f3cece2bc21f172022e2e3473c8a9279ba67a4d9bd1e352890f496",
          "MicroBlockShardId": 3,
          "MicroBlockTxnRootHash": "d97261b9c32ca9d1cfc8431a64523c9e3d26beff7e5265c5d431d5a41b416e49"
        }
      ]
    },
    "header": {
      "BlockNum": "1002353",
      "DSBlockNum": "10024",
      "GasLimit": "650000",
      "GasUsed": "25517",
      "MbInfoHash": "b2a862649507a9d86b21246b1538aa237c75f65cf7ef9a512e03ba39d0e62933",
      "NumMicroBlocks": 4,
      "NumPages": 5,
      "NumTxns": 10022,
      "PrevBlockHash": "18426f28438c500dd8b424f7923844290f4f082d43e84262ce629eebce68b82c",
      "Rewards": "0",
      "StateDeltaHash": "9e2c6b2b542219e421792892e8d42923f30fd3e4d4c55369feb89e3979b5a3a7",
      "StateRootHash": "57710511d91f7ec765c264babd53d6b607b320167029cc88c477fafd78c14632",
      "Timestamp": "1612477810820092",
      "TxnFees": "51138500000000",
      "Version": 1
    }
  }
}
```

### Example Request-2

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetTxBlockVerbose",
        "params": ["1002353"]
    }' -H "Content-Type: application/json" -X POST "https://api.zilliqa.com/"
    ```

### Example Response-2

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": {
    "B1": [
      false,
      false,
      false
      // Output truncated
    ],
    "B2": [
      false,
      false
      // Output truncated
    ],
    "CS1": "FBA696961142862169D03EED67DD302EAB91333CBC4EEFE7EDB230515DA31DC1B9746EEEE5E7C105685E22C483B1021867B3775D30215CA66D5D81543E9FE8B5",
    "PrevDSHash": "585373fb2c607b324afbe8f592e43b40d0091bbcef56c158e0879ced69648c8e",
    "header": {
      "BlockNum": "9000",
      "CommitteeHash": "da38b3b21b26b71835bb1545246a0a248f97003de302ae20d70aeaf854403029",
      "Difficulty": 95,
      "DifficultyDS": 156,
      "EpochNum": "899900",
      "GasPrice": "2000000000",
      "MembersEjected": [
        "0x02572A2FCD59F8115297B399F76D7ACCFDA7E82AC53702063C3A61FB4D85E0D0C1",
        "0x029933F07FF634654C2ECB17A90EAD00CF9EE9F75395E206660CCEFB21874ECEA1",
        "0x02AAD92E5A3C9D8ECB364225719478B51026DD5C786BF7312C5C9765353BC4C98B"
      ],
      "PoWWinners": [
        "0x0207184EB580333132787B360CA6D93290000C9F71E0B6A02C4412E7148FB1AF81",
        "0x0285B572471A9D3BA729719ED2EEE86395D3B8F243572E9099A5E8B750F46092A7",
        "0x02C1D8C0C7884E65A22FFD76DF9ACC2EA3551133E4ADD59C2DF74F327E09F709FF",
        "0x02D728E77C8DA14E900BA8A2014A0D4B5512C6BABCCB77B83F21381437E0038F44",
        "0x0321B0E1A20F02C99394DD24B34AB4E79AE6CBF0C689C222F246431A764D6B59DB",
        "0x038A724504899CCCA068BD165AE15CE2947667225C72912039CEE4EF3992334843",
        "0x03AB477A7A895DD4E84F240A2F1FCF5F86B1A3D59B6AD3065C18CD69729D089959",
        "0x03B29C7F3F85329B0621914AB0367BA78135889FB8E4F937DDB7DAA8123AD4DF3C",
        "0x03E82B00B53ECC10073404E844841C519152E500A655EEF1D8EAD6612ABDF5B552"
      ],
      "PoWWinnersIP": [
        {
          "IP": "34.212.122.139",
          "port": 33133
        },
        {
          "IP": "34.214.85.15",
          "port": 33133
        },
        {
          "IP": "54.148.246.51",
          "port": 33133
        },
        {
          "IP": "54.218.112.25",
          "port": 33133
        },
        {
          "IP": "54.184.108.224",
          "port": 33133
        },
        {
          "IP": "34.211.53.138",
          "port": 33133
        },
        {
          "IP": "44.234.38.187",
          "port": 33133
        },
        {
          "IP": "44.234.126.143",
          "port": 33133
        },
        {
          "IP": "34.223.254.106",
          "port": 33133
        }
      ],
      "PrevHash": "585373fb2c607b324afbe8f592e43b40d0091bbcef56c158e0879ced69648c8e",
      "ReservedField": "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      "SWInfo": {
        "Scilla": [0, 0, 0, "0", 0],
        "Zilliqa": [0, 0, 0, "0", 0]
      },
      "ShardingHash": "3216a33bfd4801e1907e72c7d529cef99c38d57cd281d0e9d726639fd9882d25",
      "Timestamp": "1606443830834512",
      "Version": 2
    },
    "signature": "7EE023C56602A17F2C8ABA2BEF290386D7C2CE1ABD8E3621573802FA67B243DE60B3EBEE5C4CCFDB697C80127B99CB384DAFEB44F70CD7569F2816DB950877BB"
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

| Parameter | Type   | Required | Description                                               |
| --------- | ------ | -------- | --------------------------------------------------------- |
| `id`      | string | Required | `"1"`                                                     |
| `jsonrpc` | string | Required | `"2.0"`                                                   |
| `method`  | string | Required | `"GetTxBlock"` or `"GetTxBlockVerbose"`                   |
| `params`  | string | Required | Specified TX block number to return. Example: `"1002353"` |
