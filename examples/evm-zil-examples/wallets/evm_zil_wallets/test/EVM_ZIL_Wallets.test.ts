import hre, {web3} from "hardhat";
import clc from "cli-color";

import { ScillaContract, initZilliqa, setAccount } from "hardhat-scilla-plugin";
import { Value } from "@zilliqa-js/contract";
import { assert, expect } from "chai";
import { loadZilliqaHardhatObject } from "hardhat-scilla-plugin/dist/src/ZilliqaHardhatObject";
import * as zcrypto from "@zilliqa-js/crypto";
import {bytes, Zilliqa} from "@zilliqa-js/zilliqa";
const {BN, Long} = require("@zilliqa-js/util");

const newpk = zcrypto.schnorr.generatePrivateKey();

// these are all isolated server genesis accounts; if you're using devnet,
// you'll need to find three accounts and put their private keys here.
const adminpk =
  "e53d1c3edaffc7a7bab5418eb836cf75819a82872b4a1a0f1c7fcf5c3e020b89";
const userpk =
  "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba";

const privateKeys = [adminpk, userpk];
const network_url = "http://localhost:5555";
const chain_id = 222; // 222;
const setup = initZilliqa(network_url, chain_id, privateKeys);

const adminaddress = zcrypto.getAddressFromPrivateKey(adminpk).toLowerCase();
const useraddress = zcrypto.getAddressFromPrivateKey(userpk).toLowerCase();

const adminpubkey = zcrypto.getPubKeyFromPrivateKey(adminpk);
const userpubkey = zcrypto.getPubKeyFromPrivateKey(userpk);

const adminwallet = zcrypto.getAddressFromPublicKey(adminpubkey)
const userwallet = zcrypto.getAddressFromPublicKey(userpubkey)

const adminwalletbech32 = zcrypto.toBech32Address(adminwallet)
const userwalletbech32 = zcrypto.toBech32Address(userwallet)

console.log("ZIL format: ")
console.log(adminwallet)
console.log(adminwalletbech32)
console.log(adminwallet)
console.log(userwalletbech32)

const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(hre.getZilliqaChainId(), msgVersion);

describe("EVM_ZIL_Wallets_Test", () => {

  it("Check EVM balance", async () => {

    let ethAddr = web3.eth.accounts.privateKeyToAccount(adminpk);
    console.log("ETH address: ", ethAddr.address);
    let ethAddrConverted = zcrypto.toChecksumAddress(ethAddr.address); // Zil checksum
    let initialAccountBal = await web3.eth.getBalance(ethAddr.address);
    console.log("Account to send to (zil checksum): ", ethAddrConverted);
    console.log("Account to send to, initial balance : ", initialAccountBal);

    let zilliqa = new Zilliqa(hre.getNetworkUrl());
    zilliqa.wallet.addByPrivateKey(adminpk);
    const address = zcrypto.getAddressFromPrivateKey(adminpk);
    const walletbech32 = zcrypto.toBech32Address(address)
    console.log(`My ZIL account address is: ${walletbech32}`);

    const res = await zilliqa.blockchain.getBalance(address);

    if (res.error?.message) {
      console.log("Error: ", res.error);
      throw res.error
    }

    const balance = res.result.balance;

    console.log(`My ZIL account balance is: ${balance}`)

    {

      // const gasp = await web3.eth.getGasPrice();
      const gasp = 2000000000
      const gasPrice = new BN(gasp);
      console.log(`Gas Price is: ${gasp}`)

      const tx = await zilliqa.blockchain.createTransactionWithoutConfirm(
        zilliqa.transactions.new(
          {
            version: VERSION,
            toAddr: ethAddrConverted,
            amount: new BN(balance).div(new BN(2)), // Sending an amount in Zil (1) and converting the amount to Qa
            gasPrice: gasPrice, // Minimum gasPrice veries. Check the `GetMinimumGasPrice` on the blockchain
            gasLimit: Long.fromNumber(2100)
          },
          false
        )
      );

      // process confirm
      console.log(`The transaction id is:`, tx.id);
      const confirmedTxn = await tx.confirm(tx.id);

      console.log(`The transaction status is:`);
      console.log(confirmedTxn.receipt);
    }

    {  
      let finalBal = await web3.eth.getBalance(ethAddr.address);
      console.log(`My new account balance is: ${finalBal}`);
      assert(
        finalBal == balance / 2,
        "Half of balance should be transferred"
      );
    }

  });

  
});
