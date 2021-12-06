import { Zilliqa } from "@zilliqa-js/zilliqa";
import fs from "fs";
import { getAddressFromPrivateKey, schnorr } from "@zilliqa-js/crypto";

import {
  useContractInfo,
} from "./testutil";

import {
  CONTAINER,
  API,
  TX_PARAMS,
  CONTRACTS,
  GAS_LIMIT,
  FAUCET_PARAMS,
  asyncNoop
} from "./config";

const JEST_WORKER_ID = Number(process.env["JEST_WORKER_ID"]);
const GENESIS_PRIVATE_KEY = global.GENESIS_PRIVATE_KEYS[JEST_WORKER_ID - 1];

const zilliqa = new Zilliqa(API);
zilliqa.wallet.addByPrivateKey(GENESIS_PRIVATE_KEY);

let globalStakingContractInfo;
let globalToken0ContractInfo;
let globalToken1ContractInfo;
let globalToken2ContractInfo;

let globalStakingContractAddress;
let globalToken0ContractAddress;
let globalToken1ContractAddress;
let globalToken2ContractAddress;


let globalTestAccounts: Array<{
    privateKey: string;
    address: string;
  }> = [];

const OWNER = 0;
const USER = 1;

const getTestAddr = (index) => globalTestAccounts[index]!.address as string;


beforeAll(async () => {
    let contract;
    const accounts = Array.from({ length: 2 }, schnorr.generatePrivateKey).map(
        (privateKey) => ({
          privateKey,
          address: getAddressFromPrivateKey(privateKey),
        })
    );

    for (const { privateKey, address } of accounts) {
        zilliqa.wallet.addByPrivateKey(privateKey);
        const tx = await zilliqa.blockchain.createTransaction(
          zilliqa.transactions.new(
            {
              ...FAUCET_PARAMS,
              toAddr: address,
            },
            false
          )
        );
        if (!tx.getReceipt()!.success) {
          throw new Error();
        }
      }
      globalTestAccounts = accounts;
      console.table({
        GENESIS_PRIVATE_KEY,
        OWNER: getTestAddr(OWNER),
        USER: getTestAddr(USER),
      });

    const asyncFns = [
        CONTRACTS.staking_contract.path,
        CONTRACTS.token0.path,
        CONTRACTS.token1.path,
        CONTRACTS.token2.path,
    ].map((path) => useContractInfo(CONTAINER, path, GAS_LIMIT));

    [
      globalStakingContractInfo,
      globalToken0ContractInfo,
      globalToken1ContractInfo,
      globalToken2ContractInfo
    ] = await Promise.all(asyncFns);

    let init0 = globalToken0ContractInfo.getInitParams(
      getTestAddr(OWNER),
      CONTRACTS.token0.name,
      CONTRACTS.token0.symbol,
      CONTRACTS.token0.decimals,
      CONTRACTS.token0.init_supply
    );
    [, contract] = await zilliqa.contracts
      .new(fs.readFileSync(CONTRACTS.token0.path).toString(), init0)
      .deploy(TX_PARAMS, 33, 1000, true);
      globalToken0ContractAddress = contract.address;

    let init1 = globalToken1ContractInfo.getInitParams(
      getTestAddr(OWNER),
      CONTRACTS.token1.name,
      CONTRACTS.token1.symbol,
      CONTRACTS.token1.decimals,
      CONTRACTS.token1.init_supply
    );
    [, contract] = await zilliqa.contracts
      .new(fs.readFileSync(CONTRACTS.token0.path).toString(), init1)
      .deploy(TX_PARAMS, 33, 1000, true);
      globalToken1ContractAddress = contract.address;

    let init2 = globalToken2ContractInfo.getInitParams(
      getTestAddr(OWNER),
      CONTRACTS.token2.name,
      CONTRACTS.token2.symbol,
      CONTRACTS.token2.decimals,
      CONTRACTS.token2.init_supply
    );
    [, contract] = await zilliqa.contracts
      .new(fs.readFileSync(CONTRACTS.token0.path).toString(), init2)
      .deploy(TX_PARAMS, 33, 1000, true);
      globalToken2ContractAddress = contract.address;


    let init3 = globalStakingContractInfo.getInitParams(
      getTestAddr(OWNER),
      globalToken0ContractAddress,
      "2500"
    );
    [, contract] = await zilliqa.contracts
    .new(fs.readFileSync(CONTRACTS.staking_contract.path).toString(), init3)
    .deploy(TX_PARAMS, 33, 1000, true);
    globalStakingContractAddress = contract.address;

    console.table({
        token0: globalToken0ContractAddress,
        token1: globalToken1ContractAddress,
        token2: globalToken2ContractAddress,
        staking: globalStakingContractAddress
    });
});


describe("unpause contract", () => {
    const testCases = [
        {
          name: "unpause",
          transition: "unpause",
          getSender: () => getTestAddr(OWNER),
          beforeTransition: asyncNoop,
          error: undefined,
          getParams: () => ({}),
        }
    ];

    for (const testCase of testCases) {
        it(`${testCase.transition}: ${testCase.name}`, async () => {
            console.log("testing: ", testCase.name);
            await testCase.beforeTransition();

            zilliqa.wallet.setDefault(testCase.getSender());
            const tx = await globalStakingContractInfo.callGetter(
                zilliqa.contracts.at(globalStakingContractAddress),
                TX_PARAMS
              )(testCase.transition, ...Object.values(testCase.getParams()));

            if (!tx.receipt.success) {
              throw new Error();
            }

            console.log(tx.receipt);
        });
    }

});
