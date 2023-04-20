import hre, {ethers as hh_ethers, web3} from "hardhat";
import clc from "cli-color";

import { ScillaContract, initZilliqa, setAccount } from "hardhat-scilla-plugin";
import { Value } from "@zilliqa-js/contract";
import { assert, expect } from "chai";
import { loadZilliqaHardhatObject } from "hardhat-scilla-plugin/dist/src/ZilliqaHardhatObject";
import * as zcrypto from "@zilliqa-js/crypto";
import {bytes, Zilliqa} from "@zilliqa-js/zilliqa";
const {BN, Long} = require("@zilliqa-js/util");
import {ethers} from "hardhat";

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

  it("Transfer from ZIL address to EVM address", async () => {

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

  it("Transfer from EVM address to EVM address", async () => {

    let ethAddr = web3.eth.accounts.privateKeyToAccount(adminpk);
    let ethAddrConverted = zcrypto.toChecksumAddress(ethAddr.address); // Zil checksum
    let initialAccountBal = await web3.eth.getBalance(ethAddr.address);

    let zilliqa = new Zilliqa(hre.getNetworkUrl());
    zilliqa.wallet.addByPrivateKey(userpk);
    const address = zcrypto.getAddressFromPrivateKey(userpk);
    const walletbech32 = zcrypto.toBech32Address(address)

    let ethAddrUser = web3.eth.accounts.privateKeyToAccount(userpk);
    let ethAddrConvertedUser = zcrypto.toChecksumAddress(ethAddrUser.address); // Zil checksum

    const res = await zilliqa.blockchain.getBalance(address);

    if (res.error?.message) {
      console.log("Error: ", res.error);
      throw res.error
    }

    const balance = res.result.balance;

    let initialAccountBalTo = await web3.eth.getBalance(ethAddrUser.address);

    console.log("BEFORE TRANSFER")
    console.log("EVM address From: ", ethAddr.address);
    console.log("EVM account balance : ", initialAccountBal);
    console.log("EVM address To: ", ethAddrUser.address);
    console.log("EVM account balance : ", initialAccountBalTo);
    console.log("ZIL address to send to: ", walletbech32);
    console.log(`My ZIL account balance is: ${balance}`)

    const FUNDINGWEI = ethers.utils.parseUnits("100000000000000", "gwei");
    const FUNDINQA = 100000000000000000000000

    {
      // Sender private key
      let privateKey = 'Please put your private key';

      // Create a wallet instance
      let ethFromWallet = new ethers.Wallet(adminpk, ethers.provider);
      let ethToWallet = new ethers.Wallet(userpk, ethers.provider)

      console.log("ethFromWallet: ", ethFromWallet.address);
      console.log("ethToWallet: ", ethToWallet.address);

      // Ether amount to send

      const gasPrice = await web3.eth.getGasPrice()
      console.log("Gas Price is: ", gasPrice)
      const res = await ethFromWallet.sendTransaction({
        to: ethToWallet.address,
        value: FUNDINGWEI,
        gasLimit: 21000,
        gasPrice: 20000000000
      });

      console.log("The EVM transaction is: ", res)
    }

    {
      const res = await zilliqa.blockchain.getBalance(address);

      if (res.error?.message) {
        console.log("Error: ", res.error);
        throw res.error
      }

      const newbalance = res.result.balance;

      let finalBalFrom = await web3.eth.getBalance(ethAddr.address);
      let finalBalTo = await web3.eth.getBalance(ethAddrUser.address);

      console.log("AFTER TRANSFER")
      console.log("EVM address From: ", ethAddr.address);
      console.log("EVM account balance : ", finalBalFrom);
      console.log("EVM address To: ", ethAddrUser.address);
      console.log("EVM account balance : ", finalBalTo);
      console.log("ZIL address receiver: ", walletbech32);
      console.log(`ZIL account balance receiver: ${newbalance}`)

      assert(
        Number(finalBalTo) == FUNDINQA,
        "Correct balance should be transferred"
      );
    }

  });

  it("Transfer from EVM address to ZIL address", async () => {

    let ethAddr = web3.eth.accounts.privateKeyToAccount(adminpk);
    let initialAccountBal = await web3.eth.getBalance(ethAddr.address);

    let zilliqa = new Zilliqa(hre.getNetworkUrl());
    zilliqa.wallet.addByPrivateKey(userpk);
    const userAddress = zcrypto.getAddressFromPrivateKey(userpk).toLowerCase();
    const walletbech32 = zcrypto.toBech32Address(userAddress)

    const res = await zilliqa.blockchain.getBalance(userAddress);

    if (res.error?.message) {
      console.log("Error: ", res.error);
      throw res.error
    }

    const balance = res.result.balance;

    const userAddressEVM = web3.utils.toChecksumAddress(userAddress)

    let initialAccountBalTo = await web3.eth.getBalance(userAddressEVM);

    console.log("BEFORE TRANSFER")
    console.log("EVM address From: ", ethAddr.address);
    console.log("EVM account balance : ", initialAccountBal);
    console.log("EVM address To: ", userAddressEVM);
    console.log("EVM account balance : ", initialAccountBalTo);
    console.log("ZIL address to send to: ", walletbech32);
    console.log(`My ZIL account balance is: ${balance}`)

    const FUNDINGWEI = ethers.utils.parseUnits("100000000000000", "gwei");
    const FUNDINQA = 100000000000000000000000

    {
      // Sender private key
      let privateKey = 'Please put your private key';

      // Create a wallet instance
      let ethFromWallet = new ethers.Wallet(adminpk, ethers.provider);

      console.log("ethFromWallet: ", ethFromWallet.address);
      console.log("ethToWallet: ", userAddressEVM);

      // Ether amount to send

      const gasPrice = await web3.eth.getGasPrice()
      console.log("Gas Price is: ", gasPrice)
      const res = await ethFromWallet.sendTransaction({
        to: userAddressEVM,
        value: FUNDINGWEI,
        gasLimit: 21000,
        gasPrice: 20000000000
      });

      console.log("The EVM transaction is: ", res)
    }

    {
      const res = await zilliqa.blockchain.getBalance(userAddress);

      if (res.error?.message) {
        console.log("Error: ", res.error);
        throw res.error
      }

      const newbalance = res.result.balance;

      let finalBalFrom = await web3.eth.getBalance(ethAddr.address);
      let finalBalTo = await web3.eth.getBalance(userAddressEVM);

      console.log("AFTER TRANSFER")
      console.log("EVM address From: ", ethAddr.address);
      console.log("EVM account balance : ", finalBalFrom);
      console.log("EVM address To: ", userAddressEVM);
      console.log("EVM account balance : ", finalBalTo);
      console.log("ZIL address receiver: ", walletbech32);
      console.log(`ZIL account balance receiver: ${newbalance}`)

      assert(
        Number(newbalance) == (Number(balance) + FUNDINQA),
        "Correct balance should be transferred"
      );
    }

  });

  
});
