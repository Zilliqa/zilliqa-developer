# Schnorr Algorithm

## Overview

Zilliqa employs Elliptic Curve Based Schnorr Signature Algorithm (EC-Schnorr) as the base signing algorithm. Schnorr allows for [multisignatures](multisignatures.md), is faster than ECDSA, and has a smaller signature size (64 bytes).

Refer to the Zilliqa [whitepaper](https://docs.zilliqa.com/whitepaper.pdf) for a more complete discussion of the algorithm.

## Core Usage and Implementation

The Schnorr algorithm is used during the consensus protocol, message signing, and generally anywhere where a signature is needed both for authenticity and for optionally storing alongside the signed data (e.g., DS or Tx block, Tx body, etc.).

Peers are also identified by their Schnorr public keys, alongside their IP information.

The Schnorr algorithm is implemented in `libCrypto` and is broken down into these cryptographic components: `PubKey`, `PrivKey`, and `Signature`. The `Schnorr` class provides the `Sign` and `Verify` functions, as well as `GenKeyPair` for key generation.

The signing procedure is (as noted in `Schnorr::Sign`):

```console
1. Generate a random k from [1, ..., order-1]
2. Compute the commitment Q = kG, where G is the base point
3. Compute the challenge r = H(Q, kpub, m)
4. If r = 0 mod(order), goto 1
5. Compute s = k - r*kpriv mod(order)
6. If s = 0 goto 1
7. Signature on m is (r, s)
```

The verification procedure is (as noted in `Schnorr::Verify`):

```console
1. Check if r,s is in [1, ..., order-1]
2. Compute Q = sG + r*kpub
3. If Q = O (the neutral point), return false
4. r' = H(Q, kpub, m)
5. return r' == r
```

## Dev History
1. 2017: Amrit wrote the first version in C (no copy)
2. 2017: Antonio C++-ized into first Zilliqa repo
   - https://github.com/Zilliqa/nuQoin/pull/12
3. Dec 2018: Modification of multisig hashing operation after security audit:
   - Slack discussion ("audit" private channel)
   - https://github.com/Zilliqa/Zilliqa/pull/1097
4. Jan 2019: Some re-org and better handling of initialization
   - https://github.com/Zilliqa/Zilliqa/pull/1274
5. Jan 2019: Code moved into its own repo
   - https://github.com/Zilliqa/schnorr
6. May 2019: Trezor/Trustwallet support (TrustWallet repo)
   - https://github.com/trezor/trezor-firmware/pull/93
7. Jan 2020: Small fix for `BN_nnmod`
   - https://github.com/Zilliqa/schnorr/pull/1

## Notes

The Schnorr algorithm was initially based on section 4.2.3 page 24 of version 1.0 of [BSI TR-03111 Elliptic Curve Cryptography (ECC)](https://www.bsi.bund.de/EN/Publications/TechnicalGuidelines/TR03111/BSITR03111.html).
