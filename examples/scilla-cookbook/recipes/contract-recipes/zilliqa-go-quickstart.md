---
tags:
  - GO SDK
---

# Zilliqa-Go SDK

The [gozilliqa-sdk](https://github.com/Zilliqa/gozilliqa-sdk) version of Zilliqa blockchain library.

Supports :

- multisig
- traversal of blocks
- create account
- sign transaction
- send transaction
- track transaction
- deploy contract
- generate key private keys
- call json rpc api
- validation
- utils

## Generate a large number of private keys

```go
func TestGeneratePrivateKey(t *testing.T) {
 for i := 0; i < 100000; i++ {
  privateKey, err := GeneratePrivateKey()
  if err != nil {
   panic("cannot generate private key")
  }

  prikeys := LaksaGo.EncodeHex(privateKey[:])
  if len(prikeys) != 64 {
   panic("generate private key error")
  }
  println("private key = " + prikeys)
  publickKey := GetPublicKeyFromPrivateKey(LaksaGo.DecodeHex(prikeys), true)
  pubkeys := LaksaGo.EncodeHex(publickKey)
  if len(pubkeys) != 66 {
   panic("generate public key error")
  }
  println("public key = " + pubkeys)

 }
}
```

### Call a contract

```go
func TestContract_Call(t *testing.T) {
 if os.Getenv("CI") != "" {
  t.Skip("Skipping testing in CI environment")
 }
 host := "https://dev-api.zilliqa.com/"
 privateKey := "e19d05c5452598e24caad4a0d85a49146f7be089515c905ae6a19e8a578a6930"
 chainID := 333
 msgVersion := 1

 publickKey := keytools.GetPublicKeyFromPrivateKey(util.DecodeHex(privateKey), true)
 address := keytools.GetAddressFromPublic(publickKey)
 pubkey := util.EncodeHex(publickKey)
 provider := provider2.NewProvider(host)

 wallet := account.NewWallet()
 wallet.AddByPrivateKey(privateKey)

 contract := Contract{
  Address:  "bd7198209529dC42320db4bC8508880BcD22a9f2",
  Signer:   wallet,
  Provider: provider,
 }

 args := []core.ContractValue{
  {
   "to",
   "ByStr20",
   "0x" + address,
  },
  {
   "tokens",
   "Uint128",
   "10",
  },
 }

 balAndNonce, err := provider.GetBalance("9bfec715a6bd658fcb62b0f8cc9bfa2ade71434a")
 assert.Nil(t, err, err)
 n := balAndNonce.Nonce + 1
 gasPrice, _ := provider.GetMinimumGasPrice()

 params := CallParams{
  Nonce:        strconv.FormatInt(n, 10),
  Version:      strconv.FormatInt(int64(util.Pack(chainID, msgVersion)), 10),
  GasPrice:     gasPrice,
  GasLimit:     "1000",
  SenderPubKey: pubkey,
  Amount:       "0",
}

 tx, err2 := contract.Call("Transfer", args, params, true)
 assert.Nil(t, err2, err2)
 tx.Confirm(tx.ID, 1000, 3, provider)
 assert.True(t, tx.Status == core.Confirmed)
}

```
