# Crypto functions

## SHA256

SHA256 is a cryptographic hash function that produces a fixed-size (256-bit)
hash value. In Scilla, this function is available as a built-in function.

Example:

```scilla
scilla_version 0

library CryptoExampleSHA256

contract CryptoSHA256()

field hashedResult : ByStr32 = 0x0000000000000000000000000000000000000000000000000000000000000000

transition Hash(input : String)
    hashedValue = builtin sha256hash input;
    hashedResult := hashedValue;
    e = { _eventname : "HashedValue"; value : hashedValue };
    event e
end
```

This contract allows a user to hash an input string using SHA256 and store the
result in the contract state.

## Keccak256

Keccak256 is another cryptographic hash function that produces a fixed-size
(256-bit) hash value. It's popularly used in Ethereum.

Example:

```scilla
scilla_version 0

library CryptoExampleKeccak256

contract CryptoKeccak256()

field hashedResult : ByStr32 = 0x0000000000000000000000000000000000000000000000000000000000000000

transition Hash(input : String)
    hashedValue = builtin keccak256hash input;
    hashedResult := hashedValue;
    e = { _eventname : "HashedValue"; value : hashedValue };
    event e
end
```

This contract allows a user to hash an input string using Keccak256 and store
the result in the contract state.

## Other functions

For other cryptographic primitives available in Scilla, a similar pattern can be
followed. Ensure you check Scilla's official documentation for the availability
and correct naming of other cryptographic functions. Below is a table
summarizing cryptographic functions commonly used, using the Scilla convention:

| Function name    | Inputs                          | Outputs   | Comments                                                  |
| ---------------- | ------------------------------- | --------- | --------------------------------------------------------- |
| `sha256hash`     | `String`                        | `ByStr32` | SHA-256 cryptographic hash function                       |
| `keccak256hash`  | `String`                        | `ByStr32` | Keccak-256 cryptographic hash function (used in Ethereum) |
| `ripemd160hash`  | `String`                        | `ByStr20` | RIPEMD-160 cryptographic hash function                    |
| `ecdsa_verify`   | `ByStr64`, `ByStr32`, `ByStr33` | `Bool`    | ECDSA signature verification                              |
| `schnorr_verify` | `ByStr64`, `ByStr32`, `ByStr33` | `Bool`    | Schnorr signature verification                            |
| `blake2b`        | `String`                        | `ByStr32` | Blake2b cryptographic hash function                       |
