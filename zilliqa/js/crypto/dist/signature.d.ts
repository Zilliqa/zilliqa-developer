/// <reference types="node" />
import { BN } from '@zilliqa-js/util';
interface SignatureOptions {
    r: number | string | number[] | Uint8Array | Buffer | BN;
    s: number | string | number[] | Uint8Array | Buffer | BN;
}
export declare class Signature {
    r: BN;
    s: BN;
    constructor(options: SignatureOptions);
}
export {};
//# sourceMappingURL=signature.d.ts.map