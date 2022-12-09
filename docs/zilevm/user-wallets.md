---
id: user-wallets
title: Wallets
keywords:
  - Wallets
description: Wallets that can interact with ZILEVM
---

---

## MetaMask

### Setup

!!! warning

    Your seed phrase must be backed up and kept secret. Funds can be at
    risk if this phrase cannot be recalled or exposed publically.

    Your Zilliqa seed phrase and your EVM seed phrases are different!
    Your existing Scilla keys WILL NOT resolve to the same ZILEVM address.

!!! info

    If you already have Ledger/Metamask - you can use the existing seed
    phrase to generate the same wallet addresses used on other EVM chains.

!!! tip

    You can send funds between ZILEVM and Zilliqa networks by translating
    your Zilliqa address into it's base 16 representation which ZILEVM and
    Metamask can understand.

To add a new network to MetaMask - click the current network selected at the top
of the extenstion and press 'Add Network'.

Enter the below configuration, and press save.

| Network Type | Network Name | Network RPC                                                        | ChainID | Currency Symbol | Block Explorer URL                                   |
| ------------ | ------------ | ------------------------------------------------------------------ | ------- | --------------- | ---------------------------------------------------- |
| Testnet      | EVM Test     | [https://evm-api-dev.zilliqa.com](https://evm-api-dev.zilliqa.com) | 33101   | ZIL             | [https://evmx.zilliqa.com](https://evmx.zilliqa.com) |
| Mainnet      |              |                                                                    |         |                 |                                                      |

### Sending funds

Do not send Zilliqa NFTs or fungible tokens to ZILEVM addresses and vice-versa. You will lose your assets.

#### Converting addresses

Navigate to `https://devex.zilliqa.com/address/{address}` either using a ZIL bech32 address or an ZILEVM base16 address.

By pressing the covert button, we can turn a bech32 address (zil...) into a base16 address(0x...), and vice-versa.

!["Coverting address types"](/assets/img/evm/convert_address.png)

#### ZIL -> ZILEVM

If we know the Metamask base16 address (0x...) we want to send funds to, we need to convert that into a bech32 address(zil...).

Once we have the converted wallet address starting zil... - ZilPay can be used like usual to send funds to a ZILEVM address.

#### ZILEVM -> ZIL

If we know the Zilliqa bech32 address (zil...) we want to send funds to, we need to convert that inot a base16 address(0x...).

Once we have the converted wallet address starting 0x... - Metamask can be used like normal to send funds to a Zilliqa address.

#### ZILEVM -> ZILEVM

Once Metamask has been configured with ZILEVM network details, it allows us
to send EVMZIL to other Metamask wallets using the send function within Metamask itself.

Do not send funds to Zilpay wallets or base 16 Scilla addresses.

!["Sending EVM ZIL"](/assets/img/evm/send_evm_zil.png)
