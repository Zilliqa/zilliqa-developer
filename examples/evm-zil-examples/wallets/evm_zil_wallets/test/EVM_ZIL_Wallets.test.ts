import { ScillaContract, initZilliqa, setAccount } from "hardhat-scilla-plugin";
import { Value } from "@zilliqa-js/contract";
import { assert, expect } from "chai";
import hre, {web3} from "hardhat";
import clc from "cli-color";
import { loadZilliqaHardhatObject } from "hardhat-scilla-plugin/dist/src/ZilliqaHardhatObject";
import * as zcrypto from "@zilliqa-js/crypto";

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

console.log("EVM format: ")

describe("EVM_ZIL_Wallets_Test", () => {

  it("Check EVM balance", async () => {
    let ethAddr = web3.eth.accounts.privateKeyToAccount(adminpk);
    let ethAddrConverted = zcrypto.toChecksumAddress(ethAddr.address); // Zil checksum
    let initialAccountBal = await web3.eth.getBalance(ethAddr.address);
    console.log("Account to send to (zil checksum): ", ethAddrConverted);
    console.log("Account to send to, initial balance : ", initialAccountBal);
  });

  
});
