# Zilliqa Address Standard

## Introduction

### Abstract

This post is meant to be an informative post detailing the address standard adopted by the Zilliqa blockchain. This document provides the concise details of the `bech32` address format customised for the Zilliqa protocol.

### Motivation

Due to [`EIP-55`](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md) being not widely adopted by wallets and exchanges, the 20-bytes `base16` checksum variation used by Zilliqa protocol to prevent the loss of funds sent to an Ethereum address is not viable.

Hence, Zilliqa shall adopt a variation of the `bech32` format on the wallets/SDKs level in order to prevent Users from sending ERC20 ZIL tokens from their Ethereum wallets (i.e. MyCrypto/MyEtherWallet) to a native ZIL address and vice versa.

Please note that the native protocol will still utilise the 20-bytes `base16` checksum on the backend. This is a cosmetic change of the 20-bytes `base16` checksum address to `bech32` format on the wallets/SDKs level only. It will only be visible to end-users.

## Examples

All examples below use the public key `039fbf7df13d0b6798fa16a79daabb97d4424062d2f8bd4e9a7c7851e732a25e1d`:

- Mainnet legacy `base16` checksummed address: `0x7Aa7eA9f4534d8D70224b9c2FB165242F321F12b`
- Mainnet `bech32` checksummed address: `zil102n74869xnvdwq3yh8p0k9jjgtejruft268tg8`
- Testnet `bech32` checksummed address: `zil102n74869xnvdwq3yh8p0k9jjgtejruft268tg8`

## Specification

Please refer to [`bip-0173`](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#bech32) for more details of the `bech32` technical specification.

A Zilliqa `bech32` checksummed address consists of the following aspects:

|               | Human-readable prefix | separator | `bech32` formatted address         | checksum |
| ------------- | --------------------- | --------- | ---------------------------------- | -------- |
| **Example 1** | `zil`                 | `1`       | `02n74869xnvdwq3yh8p0k9jjgtejruft` | `268tg8` |
| **Example 2** | `zil`                 | `1`       | `48fy8yjxn6jf5w36kqc7x73qd3ufuu24` | `a4u8t9` |
| **Example 3** | `zil`                 | `1`       | `fdv7u7rll9epgcqv9xxh9lhwq427nsql` | `58qcs9` |

- Do note that the human-readable prefix for [Zilliqa mainnet](https://viewblock.io/zilliqa) is `zil`, and the human-readable prefix for [Zilliqa Developer testnet](https://viewblock.io/zilliqa?network=testnet) is also `zil`.
- Do also note that the last six characters of the data part form a checksum and contain no information.

## Recommended implementation

In order to support both `bech32` (**DEFAULT**) and legacy `base16` (**OPTIONAL**) address formats, we recommend to refer to the code snippet below in order to perform a sanity check with the utility tools provided by our official [`zilliqa-js` SDK](https://github.com/Zilliqa/Zilliqa-JavaScript-Library):

```javascript
  private normaliseAddress(address: string) {
    if (validation.isBech32(address)) {
      return fromBech32Address(address);
    }

    if (isValidChecksumAddress(address)) {
      return address;
    }

    throw new Error('Address format is invalid');
  }
```

- Reference encoder and decoder in within different languages SDKs:

  - [For JavaScript](https://github.com/Zilliqa/Zilliqa-JavaScript-Library/blob/dev/packages/zilliqa-js-crypto/src/bech32.ts)
  - [For Java](https://github.com/FireStack-Lab/LaksaJ/blob/master/src/main/java/com/firestack/laksaj/utils/Bech32.java)
  - [For Python](https://github.com/deepgully/pyzil/blob/master/pyzil/crypto/bech32.py)
  - More languages support to come...

- Online utility tool to convert `base16` checksummed addresses to `bech32` format:
  - [Zilliqa Address Tool](https://www.coinhako.com/zil-check)

## Current Adoption

The lists below will be updated over time as adoption of `bech32` checksummed format increases:

### For Wallets

| Wallet                                                           | Support `bech32` checksummed addresses | Support legacy `base16` checksummed address |
| ---------------------------------------------------------------- | :------------------------------------: | :-----------------------------------------: |
| [Moonlet](https://moonlet.xyz/)                                  |           :heavy_check_mark:           |             :heavy_check_mark:              |
| [ZilPay](https://zilpay.xyz/)                                    |           :heavy_check_mark:           |             :heavy_check_mark:              |
| [Zillet](https://zillet.io/)                                     |           :heavy_check_mark:           |             :heavy_check_mark:              |
| [Trust Wallet](https://trustwallet.com/)                         |           :heavy_check_mark:           |                      -                      |
| [Math Wallet](https://www.mathwallet.org/en/)                    |           :heavy_check_mark:           |                      -                      |
| [ZHIP](https://itunes.apple.com/app/zhip/id1455248315?l=en&mt=8) |           :heavy_check_mark:           |                                             |
| [xZIL](https://tinyurl.com/y2lzmfl6)                             |           :heavy_check_mark:           |                      -                      |

### For Exchanges

| Exchanges                             | Support `bech32` checksummed addresses | Support legacy `base16` checksummed address |
| ------------------------------------- | :------------------------------------: | :-----------------------------------------: |
| [Binance](https://www.binance.com/)   |           :heavy_check_mark:           |                      -                      |
| [Kucoin](https://www.kucoin.com/)     |           :heavy_check_mark:           |                      -                      |
| [Upbit](https://upbit.com/)           |           :heavy_check_mark:           |                      -                      |
| [Coinhako](https://www.coinhako.com/) |           :heavy_check_mark:           |             :heavy_check_mark:              |
| [Coinone](https://coinone.co.kr/)     |           :heavy_check_mark:           |                      -                      |
| [Huobi](https://www.huobi.com/)       |           :heavy_check_mark:           |                      -                      |
| [OKEx](https://www.okex.com/)         |           :heavy_check_mark:           |                      -                      |
| [Gate.io](https://www.gate.io/)       |           :heavy_check_mark:           |                      -                      |
| [CoinEx](https://www.coinex.com/)     |           :heavy_check_mark:           |                      -                      |

### For Explorers

[Viewblock explorer](https://viewblock.io/zilliqa) will be supporting both the new `bech32` and legacy `base16` checksummed in the search bar in order to ease the transition to `bech32` checksummed addresses for all Users. A tooltip will also be available for Users to convert between the two address formats temporarily.
