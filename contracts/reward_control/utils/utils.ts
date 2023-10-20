/** General functions used everywhere
 *
 * Most of these came from ionise's hardhatUtils.ts .
 */

import { initZilliqa } from "hardhat-scilla-plugin";
import { expect } from "chai";
import { Zilliqa } from "@zilliqa-js/zilliqa";
import { ScillaContract, Setup as ZilliqaSetup } from "hardhat-scilla-plugin";
import hre, { ethers, network } from "hardhat";
import { getAddressFromPrivateKey } from "@zilliqa-js/crypto";
import { BN, units } from "@zilliqa-js/util";
import { Account, Transaction, TxParams, Wallet } from "@zilliqa-js/account";
import { Blockchain } from "@zilliqa-js/blockchain";
import { Wallet } from "@zilliqa-js/wallet";
import * as configUtils from "./config.ts";
import { Contract as EvmContract } from "ethers";
import * as fs from "fs";
import * as YAML from "yaml";

function getPrivateKey(accountIndex) {
  // Access the private key for the specified account index
  // console.log(hre.network.config.accounts)
  if (accountIndex >= 0 && accountIndex < hre.network.config.accounts.length) {
    const privateKey = hre.network.config.accounts[accountIndex];
    // console.log(`Private Key for Account ${accountIndex}: ${privateKey}`);
    return privateKey;
  } else {
    console.error(`Account ${accountIndex} not found in the configuration`);
    return "";
  }
}

let zilliqaSetup = undefined;

export function ensureZilliqa(): Zilliqa {
  if (zilliqaSetup == undefined) {
    const url = hre.network.config.url;
    const chainId = hre.network.config.chainId & ~0x8000;
    console.log(`Connecting to Zilliqa on ${url}, chainId ${chainId}`);
    // Set the gas price to 2 - because it is in Li and otherwise the amount you need in your account
    // is quite enormous.
    const setup = initZilliqa(
      url,
      chainId,
      hre.network.config.accounts,
      50,
      1000,
      2000,
      40000
    );
    zilliqaSetup = setup;
  }
  return zilliqaSetup;
}

// reimplemented here for convenience.
export function zilliqaAddressFromPrivateKey(privKey: string): string {
  return getAddressFromPrivateKey(privKey);
}

export function evmAddressFromPrivateKey(privKey: string): string {
  const wallet = new ethers.Wallet(privKey);
  return wallet.address;
}

export async function getEvmSignerForAccount(account: string) {
  let rec = configUtils.getAccounts()[account];
  return getEvmSignerForAddress(evmAddressFromPrivateKey(rec.privkey));
}

export async function getZilliqaBlockchainForAccount(account: string) {
  let rec = configUtils.getAccounts()[account];
  return getZilliqaBlockchainForPrivateKey(rec.privkey);
}

export function setZilliqaSignerToAccountByHardhatWallet(accountIndex: number) {
  let zho = hre.zilliqa;
  let pvt_key = getPrivateKey(accountIndex);
  // console.log(pvt_key)
  let address = zilliqaAddressFromPrivateKey(pvt_key);

  console.log(`Setting default wallet for signing to: ${address}`);
  zho.getZilliqaJSObject().wallet.setDefault(address);
  hre.setActiveAccount(accountIndex);
  // zho.getZilliqaJSObject().wallet.addByPrivateKey(pvt_key);
}

export function setZilliqaSignerToAccount(account: string) {
  let zho = hre.zilliqa;
  let rec = configUtils.getAccounts()[account];
  let address = zilliqaAddressFromPrivateKey(rec.privkey);
  zho.getZilliqaJSObject().wallet.setDefault(address);
}

export function getZilliqaAddressForAccount(account: string): string {
  let rec = configUtils.getAccounts()[account];
  let address = zilliqaAddressFromPrivateKey(rec.privkey);
  return address;
}

export function getZilliqaAddressForAccountByHardhatWallet(
  accountIndex: number
): string {
  let pvt_key = getPrivateKey(accountIndex);
  let address = zilliqaAddressFromPrivateKey(pvt_key);
  return address;
}

export function getZilliqaAccountForAccountByHardhatWallet(
  accountIndex: number
): string {
  let pvt_key = getPrivateKey(accountIndex);
  // let address = zilliqaAddressFromPrivateKey(pvt_key);
  return new Account(pvt_key);
}

export function getZilliqaAccountForAccount(account: string): string {
  let rec = configUtils.getAccounts()[account];
  return new Account(rec.privkey);
}

export function getEvmAddressForAccount(account: string): string {
  let rec = configUtils.getAccounts()[account];
  return evmAddressFromPrivateKey(rec.privkey);
}

export async function getEvmSignerForAddress(address: string) {
  const allSigners = await ethers.getSigners();
  const selectedSigner = allSigners.find(
    (s) => s.address.toLowerCase() == address.toLowerCase()
  );
  if (!selectedSigner) {
    throw new Error(
      `No signer found for ${address}. Did you include the private key in your account list in hardhat.config.ts`
    );
  }
  return selectedSigner;
}

export async function getZilliqaBlockchainForPrivateKey(
  setup: ZilliqaSetup,
  privKey: string
): Blockchain {
  let accounts = [privKey].map((x) => new Account(x));
  const wallet = new Wallet(setup.zilliqa.provider, accounts);
  const sender = new Blockchain(setup.zilliqa.provider, wallet);
  return sender;
}

export async function getZilBalance(setup: ZilliqaSetup, address: string): BN {
  let balance = await setup.zilliqa.blockchain.getBalance(address);
  if (
    balance["result"] !== undefined &&
    balance["result"]["balance"] !== undefined
  ) {
    return new BN(balance["result"]["balance"]);
  }
  return new BN("0");
}

export async function assertSuccess(ptx: Promise<ContractTransaction>) {
  const ll = logger.child({ utils: "assertSuccess" });

  try {
    const tx = await ptx;

    if (tx.confirmations < 1) {
      ll.info("awaiting tx receipt");
      const receipt = await tx.wait(1);

      if (receipt.status === 0) {
        ll.error({ receipt });
        throw new Error("transaction failed");
      }

      ll.info("transaction succeeded");
    } else {
      /**
       * tx.wait with ethere 5 gives "not an array" error
       * turns out that await ptx returns transaction that
       * was confirmed by just one block
       */
      ll.info("transaction succeeded without waiting for receipt");
    }
  } catch (e) {
    ll.error(e);
    throw e;
  }
}

export function TestGeneral(num: Number, desc: string) {
  return TestName("GEN", num, desc);
}

export function TestName(kind: string, num: Number, desc: string) {
  let pad = num.toString().padStart(4, "0");
  return `${kind.toUpperCase()}${pad} - ${desc}`;
}

// Check that a scilla event reported from an EVM call "looks" OK
export function validateScillaEvent(
  scillaEventName: string,
  contractAddress: string,
  event: any
) {
  expect(event["address"].toLowerCase()).to.eq(contractAddress.toLowerCase());
  const EXPECTED_TOPIC_0 = utils.keccak256(
    toUtf8Bytes(scillaEventName + "(string)")
  );
  expect(event["topics"][0].toLowerCase()).to.eq(
    EXPECTED_TOPIC_0.toLowerCase()
  );
  const decodedData = defaultAbiCoder.decode(["string"], event["data"]);
  console.log(`decoded ${decodedData}`);
  const scillaEvent = JSON.parse(decodedData.toString());
  expect(scillaEvent["_eventname"]).to.be.equal(scillaEventName);
}

// Check that an EVM event reported from an EVM call "looks" OK.
export function validateEvmEvent(
  evmEventName: string,
  contractAddress: string,
  event: any
) {
  expect(event["address"].toLowerCase()).to.eq(contractAddress.toLowerCase());
  const EXPECTED_TOPIC_0 = utils.keccak256(toUtf8Bytes(evmEventName + "()"));
  expect(event["topics"][0].toLowerCase()).to.eq(
    EXPECTED_TOPIC_0.toLowerCase()
  );
}

// Check that the given Zilliqa native transaction receipt was successful; throw and print complaints if it wasn't.
export function checkScillaTransactionSuccess(txn: any) {
  console.log(`S ${txn.status}`);
  if (txn.deployed_by.receipt !== undefined) {
    console.log(`XX ${JSON.stringify(txn.deployed_by.receipt)}`);
    let errs = txn.deployed_by.receipt.errors;
    console.log(`E ${JSON.stringify(errs)}`);
    for (var idx in errs) {
      console.log(`Idx ${idx}`);
    }
  }
}

export async function registerEvmDeployment(
  name: string,
  contract: EvmContract,
  extra: any
) {
  // Write a description file.
  let receipt = await contract.deploymentTransaction();
  let chainId = receipt.chainId;
  let address = await contract.getAddress();
  let description = <configUtils.ContractDescription>{
    name: name,
    extra: extra,
    chainid: Number(chainId),
    address: address,
    contractType: configUtils.ContractType.EVM,
  };
  // Write out the description
  configUtils.updateDeploymentDescription(description);
  // @TODO - upload metadata to Sourcify
}

export async function registerScillaDeployment(
  name: string,
  contract: ScillaContract,
  extra: any
): configUtils.ContractDescription {
  let chainId = contract.deployed_by.version >> 16;
  let address = contract.address;
  let description = <configUtils.ContractDescription>{
    name: name,
    extra: extra,
    chainid: Number(chainId),
    address: address,
    contractType: configUtils.ContractType.Scilla,
  };
  configUtils.updateDeploymentDescription(description);
  return description;
}

export function assertDeployedContractData(
  name: string
): configUtils.ContractDescription {
  console.log(`Called!`);
  let data = configUtils.getDeploymentDescription(name);
  console.log(`getDeployment ${JSON.stringify(data)}`);
  if (data === undefined) {
    throw Error(`No deployed contract ${name}`);
  } else {
    console.log("Fish!");
    return data as configUtils.ContractDescription;
  }
}

export function assertDeployedSecrets(name: string): any {
  let data = configUtils.getSecrets(name);
  if (data === undefined) {
    throw Error(`No deployed secrets ${name}`);
  }
  return data;
}

export function updateDeployedSecrets(name: string, value: any) {
  configUtils.updateSecrets(name, value);
}
