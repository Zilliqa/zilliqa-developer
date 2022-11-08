import { Signer, Provider } from '@zilliqa-js/core';
import * as zcrypto from '@zilliqa-js/crypto';
import { Account } from './account';
import { Transaction } from './transaction';
export declare class Wallet extends Signer {
    accounts: {
        [address: string]: Account;
    };
    defaultAccount?: Account;
    provider: Provider;
    /**
     * constructor
     *
     * Takes an array of Account objects and instantiates a Wallet instance.
     *
     * @param {Account[]} accounts
     */
    constructor(provider: Provider, accounts?: Account[]);
    /**
     * create
     *
     * Creates a new keypair with a randomly-generated private key. The new
     * account is accessible by address.
     *
     * @returns {string} - address of the new account
     */
    create(): string;
    /**
     * addByPrivateKey
     *
     * Adds an account to the wallet by private key.
     *
     * @param {string} privateKey - hex-encoded private key
     * @returns {string} - the corresponing address, computer from the private
     * key.
     */
    addByPrivateKey(privateKey: string): string;
    /**
     * addByKeystore
     *
     * Adds an account by keystore. This method is asynchronous and returns
     * a Promise<string>, in order not to block on the underlying decryption
     * operation.
     *
     * @param {string} keystore
     * @param {string} passphrase
     * @returns {Promise<string>}
     */
    addByKeystore(keystore: string, passphrase: string): Promise<string>;
    /**
     * addByMnemonic
     *
     * Adds an `Account` by use of a mnemonic as specified in BIP-32 and BIP-39
     *
     * @param {string} phrase - 12-word mnemonic phrase
     * @param {number} index=0 - the number of the child key to add
     * @returns {string} - the corresponding address
     */
    addByMnemonic(phrase: string, index?: number): string;
    /**
     * addByMnemonicLedger
     *
     * Adds an `Account` by use of a mnemonic as specified in BIP-32 and BIP-39
     * The key derivation path used in Ledger is different from that of
     * addByMnemonic.
     *
     * @param {string} phrase - 12-word mnemonic phrase
     * @param {number} index=0 - the number of the child key to add
     * @returns {string} - the corresponding address
     */
    addByMnemonicLedger(phrase: string, index?: number): string;
    /**
     * export
     *
     * Exports the specified account as a keystore file.
     *
     * @param {string} address
     * @param {string} passphrase
     * @param {KDF} kdf='scrypt'
     * @returns {Promise<string>}
     */
    export(address: string, passphrase: string, kdf?: zcrypto.KDF): Promise<string>;
    /**
     * remove
     *
     * Removes an account from the wallet and returns boolean to indicate
     * failure or success.
     *
     * @param {string} address
     * @returns {boolean}
     */
    remove(address: string): boolean;
    /**
     * setDefault
     *
     * Sets the default account of the wallet.
     *
     * @param {string} address
     */
    setDefault(address: string): void;
    /**
     * sign
     *
     * signs an unsigned transaction with the default account.
     *
     * @param {Transaction} tx
     * @param {boolean} offlineSign
     * @returns {Transaction}
     */
    sign(tx: Transaction, offlineSign?: boolean): Promise<Transaction>;
    signBatch(txList: Transaction[]): Promise<Transaction[]>;
    /**
     * signWith
     *
     * @param {Transaction} tx
     * @param {string} account
     * @param {boolean} offlineSign
     * @returns {Transaction}
     */
    signWith(tx: Transaction, account: string, offlineSign?: boolean): Promise<Transaction>;
    private isValidMnemonic;
}
//# sourceMappingURL=wallet.d.ts.map