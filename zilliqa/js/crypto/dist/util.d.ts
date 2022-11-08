/**
 * getAddressFromPrivateKey
 *
 * takes a hex-encoded string (private key) and returns its corresponding
 * 20-byte hex-encoded address.
 *
 * @param {string} privateKey
 * @returns {string}
 */
export declare const getAddressFromPrivateKey: (privateKey: string) => string;
/**
 * getPubKeyFromPrivateKey
 *
 * takes a hex-encoded string (private key) and returns its corresponding
 * hex-encoded 33-byte public key.
 *
 * @param {string} privateKey
 * @returns {string}
 */
export declare const getPubKeyFromPrivateKey: (privateKey: string) => string;
/**
 * getAccountFrom0xPrivateKey
 *
 * Utility method for recovering account from 0x private key.
 * See https://github.com/Zilliqa/zilliqa-js/pull/159
 * @param privateKeyWith0x : private key with 0x prefix
 */
export declare const getAccountFrom0xPrivateKey: (privateKeyWith0x: string) => {
    with0x: {
        prv: string;
        pub: string;
        addr: string;
        bech32: string;
    };
    without0x: {
        prv: string;
        pub: string;
        addr: string;
        bech32: string;
    };
    changed: {
        prv: string;
        pub: string;
        addr: string;
        bech32: string;
    };
};
/**
 * compressPublicKey
 *
 * @param {string} publicKey - 65-byte public key, a point (x, y)
 *
 * @returns {string}
 */
export declare const compressPublicKey: (publicKey: string) => string;
/**
 * getAddressFromPublicKey
 *
 * takes hex-encoded string and returns the corresponding address
 *
 * @param {string} pubKey
 * @returns {string}
 */
export declare const getAddressFromPublicKey: (publicKey: string) => string;
/**
 * toChecksumAddress
 *
 * takes hex-encoded string and returns the corresponding address
 *
 * @param {string} address
 * @returns {string}
 */
export declare const toChecksumAddress: (address: string) => string;
/**
 * isValidChecksumAddress
 *
 * takes hex-encoded string and returns boolean if address is checksumed
 *
 * @param {string} address
 * @returns {boolean}
 */
export declare const isValidChecksumAddress: (address: string) => boolean;
/**
 * normaliseAddress
 *
 * takes in a base16 address or a zilliqa bech32 encoded address
 * and returns a checksum base16 address. If the address is neither a base16
 * nor bech32 address, the code will return an error
 * @param {string)} address
 * @returns {string}
 */
export declare const normaliseAddress: (address: string) => string;
/**
 * encodeBase58 - may be required for DID public key
 * undeprecating this function after version 2.0.0
 *
 * @param {string} hex - base 16 encoded string
 * @returns {string} - big endian base 58 encoded string
 */
export declare const encodeBase58: (hex: string) => string;
/**
 * decodeBase58 - may be required for DID public key
 * undeprecating this function after version 2.0.0
 *
 * @param {string} raw - base 58 string
 * @returns {string} - big endian base 16 string
 */
export declare const decodeBase58: (raw: string) => string;
/**
 * verifyPrivateKey
 *
 * @param {string|Buffer} privateKey
 * @returns {boolean}
 */
export declare const verifyPrivateKey: (privateKey: string) => boolean;
/**
 * normalizePrivateKey : normalise private key from 0x or without 0x prefix
 *
 * @param {string} privateKey
 * @returns {string}
 */
export declare const normalizePrivateKey: (privateKey: string) => string;
//# sourceMappingURL=util.d.ts.map