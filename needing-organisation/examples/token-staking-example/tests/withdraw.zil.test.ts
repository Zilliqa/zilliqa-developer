import { Zilliqa } from "@zilliqa-js/zilliqa";
import fs from "fs";
import { getAddressFromPrivateKey, schnorr } from "@zilliqa-js/crypto";

import { useContractInfo, increaseBNum } from "./testutil";

import {
  CONTAINER,
  API,
  TX_PARAMS,
  CONTRACTS,
  GAS_LIMIT,
  FAUCET_PARAMS,
  asyncNoop,
  STAKING_ERROR,
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
    globalToken2ContractInfo,
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
    "10"
  );
  [, contract] = await zilliqa.contracts
    .new(fs.readFileSync(CONTRACTS.staking_contract.path).toString(), init3)
    .deploy(TX_PARAMS, 33, 1000, true);
  globalStakingContractAddress = contract.address;

  console.table({
    token0: globalToken0ContractAddress,
    token1: globalToken1ContractAddress,
    token2: globalToken2ContractAddress,
    staking: globalStakingContractAddress,
  });

  zilliqa.wallet.setDefault(getTestAddr(OWNER));
  const tx0 = await globalToken0ContractInfo.callGetter(
    zilliqa.contracts.at(globalToken0ContractAddress),
    TX_PARAMS
  )(
    "IncreaseAllowance",
    globalStakingContractAddress,
    Number(10000000000000000).toString()
  );
  if (!tx0.receipt.success) {
    throw new Error();
  }
  console.log(tx0.receipt);

  const tx1 = await globalToken1ContractInfo.callGetter(
    zilliqa.contracts.at(globalToken1ContractAddress),
    TX_PARAMS
  )(
    "Transfer",
    globalStakingContractAddress,
    Number(10000000000000000).toString()
  );
  if (!tx1.receipt.success) {
    throw new Error();
  }

  const tx2 = await globalToken2ContractInfo.callGetter(
    zilliqa.contracts.at(globalToken2ContractAddress),
    TX_PARAMS
  )(
    "Transfer",
    globalStakingContractAddress,
    Number(10000000000000000).toString()
  );
  if (!tx2.receipt.success) {
    throw new Error();
  }

  const tx3 = await globalStakingContractInfo.callGetter(
    zilliqa.contracts.at(globalStakingContractAddress),
    TX_PARAMS
  )("update_token_rewards", globalToken1ContractAddress, "10000000000000");
  if (!tx3.receipt.success) {
    throw new Error();
  }

  const tx4 = await globalStakingContractInfo.callGetter(
    zilliqa.contracts.at(globalStakingContractAddress),
    TX_PARAMS
  )("update_token_rewards", globalToken2ContractAddress, "10000000000000");
  if (!tx4.receipt.success) {
    throw new Error();
  }
});

describe("staking contract", () => {
  const testCases = [
    {
      name: "deposit once",
      transition: "deposit",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({
        amount: "10",
      }),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) === `{"1":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"`
          );
        },
      },
    },
    {
      name: "withdraw on current cycle",
      transition: "withdraw",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
    },
    {
      name: "deposit once again",
      transition: "deposit",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({
        amount: "10",
      }),
      beforeTransition: asyncNoop,
      error: undefined,
    },
    {
      name: "withdraw with rewards",
      transition: "withdraw",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: async () => {
        await increaseBNum(zilliqa, 3650);
      },
      error: STAKING_ERROR.UserHasUnclaimedReward,
    },
    {
      name: "claim",
      transition: "claim",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
    },
    {
      name: "withdraw without rewards",
      transition: "withdraw",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
    },
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

      console.log("transaction id = ", tx.id);
      console.log(tx.receipt);
      const state = await zilliqa.contracts
        .at(globalStakingContractAddress)
        .getState();
      console.log(state);
    });
  }
});
