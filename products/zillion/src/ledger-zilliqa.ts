import type Transport from "@ledgerhq/hw-transport";
import TransportU2F from "@ledgerhq/hw-transport-u2f";
import TransportWebHID from "@ledgerhq/hw-transport-webhid";
import TransportWebUSB from "@ledgerhq/hw-transport-webusb";
import { Constants } from "./util/enum";

const { BN, Long } = require("@zilliqa-js/util");
const { encodeTransactionProto } = require("@zilliqa-js/zilliqa");

const CLA = 0xe0;
const INS = {
    getVersion: 0x01,
    getPublicKey: 0x02,
    getPublicAddress: 0x02,
    signTxn: 0x04,
};

const PubKeyByteLen = 33;
const SigByteLen = 64;
const Bech32AddrLen = "zil".length + 1 + 32 + 6;

export class LedgerZilliqa {
    transport: Transport<any>;

    // helper to create the transport object
    // which is later used to init the constructor
    static async getTransport() {
        const isWebHIDSupported = await TransportWebHID.isSupported().catch(
            () => false
        );
        const isWebUSBSupported = await TransportWebUSB.isSupported().catch(
            () => false
        );

        if (isWebHIDSupported) {
            console.log("ledger webhid supported");
            const list = await TransportWebHID.list();
            if (
                list.length > 0 &&
                list[0].opened &&
                list[0].vendorId === Constants.LEDGER_VENDOR_ID
            ) {
                return new TransportWebHID(list[0]);
            }
            const existing = await TransportWebHID.openConnected().catch(
                () => null
            );
            return existing ?? TransportWebHID.request();
        }

        if (isWebUSBSupported) {
            console.log("ledger webusb supported");
            const existing = await TransportWebUSB.openConnected().catch(
                () => null
            );
            if (existing === null) {
                return TransportWebUSB.create();
            }
            return existing;
        }

        console.log("ledger u2f fallback");
        return TransportU2F.create();
    }

    constructor(transport: Transport<any>, scrambleKey: string = "w0w") {
        this.transport = transport;
        transport.decorateAppAPIMethods(
            this,
            ["getVersion", "getPublicKey", "getPublicAddress", "signTxn"],
            scrambleKey
        );
    }

    getVersion() {
        const P1 = 0x00;
        const P2 = 0x00;

        return this.transport
            .send(CLA, INS.getVersion, P1, P2)
            .then((response) => {
                let version = "v";
                for (let i = 0; i < 3; ++i) {
                    version += parseInt("0x" + response[i]);
                    if (i !== 2) {
                        version += ".";
                    }
                }
                return { version };
            });
    }

    // index: index indicate by ledger, default: 0
    getPublicKey(index: number) {
        const P1 = 0x00;
        const P2 = 0x00;

        let payload = Buffer.alloc(4);
        payload.writeInt32LE(index, 0);

        return this.transport
            .send(CLA, INS.getPublicKey, P1, P2, payload)
            .then((response) => {
                // The first PubKeyByteLen bytes are the public address
                const publicKey = response
                    .toString("hex")
                    .slice(0, PubKeyByteLen * 2);
                return { publicKey };
            });
    }

    // index: index indicate by ledger, default: 0
    getPublicAddress(index: number) {
        const P1 = 0x00;
        const P2 = 0x01;

        let payload = Buffer.alloc(4);
        payload.writeInt32LE(index, 0);

        return this.transport
            .send(CLA, INS.getPublicAddress, P1, P2, payload)
            .then((response) => {
                // After the first PubKeyByteLen bytes, the remaining is the bech32 address string.
                const pubAddr = response
                    .slice(PubKeyByteLen, PubKeyByteLen + Bech32AddrLen)
                    .toString("utf-8");
                const pubKey = response
                    .toString("hex")
                    .slice(0, PubKeyByteLen * 2);
                return { pubAddr, pubKey };
            });
    }

    signTxn(keyIndex: number, txnParams: any) {
        // https://github.com/Zilliqa/Zilliqa-JavaScript-Library/tree/dev/packages/zilliqa-js-account#interfaces
        const P1 = 0x00;
        const P2 = 0x00;

        let indexBytes = Buffer.alloc(4);
        indexBytes.writeInt32LE(keyIndex, 0);

        // convert to zilliqa types
        if (!(txnParams.amount instanceof BN)) {
            txnParams.amount = new BN(txnParams.amount);
        }

        if (!(txnParams.gasPrice instanceof BN)) {
            txnParams.gasPrice = new BN(txnParams.gasPrice);
        }

        if (!(txnParams.gasLimit instanceof BN)) {
            txnParams.gasLimit = Long.fromNumber(txnParams.gasLimit);
        }

        var txnBytes = encodeTransactionProto(txnParams);

        const STREAM_LEN = 128; // Stream in batches of STREAM_LEN bytes each.
        var txn1Bytes;
        if (txnBytes.length > STREAM_LEN) {
            txn1Bytes = txnBytes.slice(0, STREAM_LEN);
            txnBytes = txnBytes.slice(STREAM_LEN, undefined);
        } else {
            txn1Bytes = txnBytes;
            txnBytes = Buffer.alloc(0);
        }

        var txn1SizeBytes = Buffer.alloc(4);
        txn1SizeBytes.writeInt32LE(txn1Bytes.length, 0);
        var hostBytesLeftBytes = Buffer.alloc(4);
        hostBytesLeftBytes.writeInt32LE(txnBytes.length, 0);
        // See signTxn.c:handleSignTxn() for sequence details of payload.
        // 1. 4 bytes for indexBytes.
        // 2. 4 bytes for hostBytesLeftBytes.
        // 3. 4 bytes for txn1SizeBytes (number of bytes being sent now).
        // 4. txn1Bytes of actual data.
        const payload = Buffer.concat([
            indexBytes,
            hostBytesLeftBytes,
            txn1SizeBytes,
            txn1Bytes,
        ]);

        let transport = this.transport;
        return transport
            .send(CLA, INS.signTxn, P1, P2, payload)
            .then(function cb(response): any {
                // Keep streaming data into the device till we run out of it.
                // See signTxn.c:istream_callback() for how this is used.
                // Each time the bytes sent consists of:
                //  1. 4-bytes of hostBytesLeftBytes.
                //  2. 4-bytes of txnNSizeBytes (number of bytes being sent now).
                //  3. txnNBytes of actual data.
                if (txnBytes.length > 0) {
                    var txnNBytes;
                    if (txnBytes.length > STREAM_LEN) {
                        txnNBytes = txnBytes.slice(0, STREAM_LEN);
                        txnBytes = txnBytes.slice(STREAM_LEN, undefined);
                    } else {
                        txnNBytes = txnBytes;
                        txnBytes = Buffer.alloc(0);
                    }

                    var txnNSizeBytes = Buffer.alloc(4);
                    txnNSizeBytes.writeInt32LE(txnNBytes.length, 0);
                    hostBytesLeftBytes.writeInt32LE(txnBytes.length, 0);
                    const payload = Buffer.concat([
                        hostBytesLeftBytes,
                        txnNSizeBytes,
                        txnNBytes,
                    ]);
                    return transport
                        .send(CLA, INS.signTxn, P1, P2, payload)
                        .then(cb);
                }
                return response;
            })
            .then((result) => {
                return result.toString("hex").slice(0, SigByteLen * 2);
            });
    }
}
