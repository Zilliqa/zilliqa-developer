/// <reference types="node" />
import * as schnorr from './schnorr';
import { Signature } from './signature';
/**
 * sign
 *
 * @param {string} hash - hex-encoded hash of the data to be signed
 *
 * @returns {string} the signature
 */
export declare const sign: (msg: Buffer, privateKey: string, pubKey: string) => string;
export { schnorr, Signature };
export * from './util';
export * from './keystore';
export * from './random';
export * from './types';
export * from './bech32';
//# sourceMappingURL=index.d.ts.map