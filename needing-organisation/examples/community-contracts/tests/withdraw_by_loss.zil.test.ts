import { Zilliqa } from "@zilliqa-js/zilliqa";
import fs from "fs";
import { getAddressFromPrivateKey, schnorr } from "@zilliqa-js/crypto";

import { increaseBNum, getJSONParams, getErrorMsg } from "./testutil";

import {
  API,
  TX_PARAMS,
  CONTRACTS,
  FAUCET_PARAMS,
  asyncNoop,
  STAKING_ERROR,
} from "./config";

const JEST_WORKER_ID = Number(process.env["JEST_WORKER_ID"]);
const GENESIS_PRIVATE_KEY = global.GENESIS_PRIVATE_KEYS[JEST_WORKER_ID - 1];

const zilliqa = new Zilliqa(API);
zilliqa.wallet.addByPrivateKey(GENESIS_PRIVATE_KEY);

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

  const init0 = getJSONParams({
    _scilla_version: ["Uint32", 0],
    contract_owner: ["ByStr20", getTestAddr(OWNER)],
    name: ["String", CONTRACTS.token0.name],
    symbol: ["String", CONTRACTS.token0.symbol],
    decimals: ["Uint32", CONTRACTS.token0.decimals],
    init_supply: ["Uint128", CONTRACTS.token0.init_supply],
  });

  [, contract] = await zilliqa.contracts
    .new(fs.readFileSync(CONTRACTS.token0.path).toString(), init0)
    .deploy(TX_PARAMS, 33, 1000, true);
  globalToken0ContractAddress = contract.address;

  const init1 = getJSONParams({
    _scilla_version: ["Uint32", 0],
    contract_owner: ["ByStr20", getTestAddr(OWNER)],
    name: ["String", CONTRACTS.token1.name],
    symbol: ["String", CONTRACTS.token1.symbol],
    decimals: ["Uint32", CONTRACTS.token1.decimals],
    init_supply: ["Uint128", CONTRACTS.token1.init_supply],
  });

  [, contract] = await zilliqa.contracts
    .new(fs.readFileSync(CONTRACTS.token1.path).toString(), init1)
    .deploy(TX_PARAMS, 33, 1000, true);
  globalToken1ContractAddress = contract.address;

  const init2 = getJSONParams({
    _scilla_version: ["Uint32", 0],
    contract_owner: ["ByStr20", getTestAddr(OWNER)],
    name: ["String", CONTRACTS.token2.name],
    symbol: ["String", CONTRACTS.token2.symbol],
    decimals: ["Uint32", CONTRACTS.token2.decimals],
    init_supply: ["Uint128", CONTRACTS.token2.init_supply],
  });

  [, contract] = await zilliqa.contracts
    .new(fs.readFileSync(CONTRACTS.token2.path).toString(), init2)
    .deploy(TX_PARAMS, 33, 1000, true);
  globalToken2ContractAddress = contract.address;

  const init3 = getJSONParams({
    _scilla_version: ["Uint32", 0],
    init_contract_owner: ["ByStr20", getTestAddr(OWNER)],
    init_staking_token_address: ["ByStr20", globalToken0ContractAddress],
    blocks_per_cycle: ["Uint256", "10"],
  });

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
  const tx0: any = await zilliqa.contracts.at(globalToken0ContractAddress).call(
    "IncreaseAllowance",
    getJSONParams({
      spender: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 100000000000000],
    }),
    TX_PARAMS
  );
  if (!tx0.receipt.success) {
    throw new Error();
  }

  const tx1: any = await zilliqa.contracts.at(globalToken1ContractAddress).call(
    "IncreaseAllowance",
    getJSONParams({
      spender: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 100000000000000],
    }),
    TX_PARAMS
  );
  if (!tx1.receipt.success) {
    throw new Error();
  }

  const tx2: any = await zilliqa.contracts.at(globalToken2ContractAddress).call(
    "IncreaseAllowance",
    getJSONParams({
      spender: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 100000000000000],
    }),
    TX_PARAMS
  );
  if (!tx2.receipt.success) {
    throw new Error();
  }

  const tx3: any = await zilliqa.contracts
    .at(globalStakingContractAddress)
    .call(
      "update_token_rewards",
      getJSONParams({
        token_address: ["ByStr20", globalToken1ContractAddress],
        amount_per_cycle: ["Uint128", 10000000000000],
      }),
      TX_PARAMS
    );
  if (!tx3.receipt.success) {
    throw new Error();
  }

  const tx4: any = await zilliqa.contracts
    .at(globalStakingContractAddress)
    .call(
      "update_token_rewards",
      getJSONParams({
        token_address: ["ByStr20", globalToken2ContractAddress],
        amount_per_cycle: ["Uint128", 10000000000000],
      }),
      TX_PARAMS
    );
  if (!tx4.receipt.success) {
    throw new Error();
  }

  const tx5: any = await zilliqa.contracts.at(globalToken1ContractAddress).call(
    "Transfer",
    getJSONParams({
      to: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 1000000000000000],
    }),
    TX_PARAMS
  );
  if (!tx5.receipt.success) {
    throw new Error();
  }

  const tx6: any = await zilliqa.contracts.at(globalToken2ContractAddress).call(
    "Transfer",
    getJSONParams({
      to: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 1000000000000000],
    }),
    TX_PARAMS
  );
  if (!tx6.receipt.success) {
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
        amount: ["Uint128", 10],
      }),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) === `{"1":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"` &&
            JSON.stringify(state.last_cycle) === `"1"` &&
            JSON.stringify(state.stakers_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":{"1":"10"}}` &&
            JSON.stringify(state.stakers_total_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"10"}` &&
            JSON.stringify(state.last_deposit_cycle) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"1"}`
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
      error: STAKING_ERROR.StillInLockupPeriod,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) === `{"1":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"` &&
            JSON.stringify(state.last_cycle) === `"1"` &&
            JSON.stringify(state.stakers_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":{"1":"10"}}` &&
            JSON.stringify(state.stakers_total_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"10"}` &&
            JSON.stringify(state.last_deposit_cycle) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"1"}`
          );
        },
      },
    },
    {
      name: "withdraw with rewards",
      transition: "withdraw",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: async () => {
        await increaseBNum(zilliqa, 30);
      },
      error: STAKING_ERROR.UserHasUnclaimedReward,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) === `{"1":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"` &&
            JSON.stringify(state.last_cycle) === `"1"` &&
            JSON.stringify(state.stakers_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":{"1":"10"}}` &&
            JSON.stringify(state.stakers_total_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"10"}` &&
            JSON.stringify(state.last_deposit_cycle) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"1"}`
          );
        },
      },
    },
    {
      name: "claim",
      transition: "claim",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"10","2":"10","3":"10","4":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"` &&
            JSON.stringify(state.last_cycle) === `"4"` &&
            JSON.stringify(state.stakers_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":{"1":"10"}}` &&
            JSON.stringify(state.stakers_total_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"10"}` &&
            JSON.stringify(state.last_deposit_cycle) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"1"}`
          );
        },
      },
    },
    {
      name: "withdraw again",
      transition: "withdraw",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: STAKING_ERROR.StillInLockupPeriod,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"10","2":"10","3":"10","4":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"` &&
            JSON.stringify(state.last_cycle) === `"4"` &&
            JSON.stringify(state.stakers_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":{"1":"10"}}` &&
            JSON.stringify(state.stakers_total_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"10"}` &&
            JSON.stringify(state.last_deposit_cycle) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"1"}`
          );
        },
      },
    },
    {
      name: "withdraw by loss",
      transition: "withdraw_by_loss",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        events: [
          {
            name: "WithdrawStakeByLoss",
            getParams: () => ({
              stake_amount: ["Uint128", 10],
              transfer_amount: ["Uint128", 9],
              penalty_amount: ["Uint128", 1],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(OWNER)],
              amount: ["Uint128", "9"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(OWNER)],
              amount: ["Uint128", "9"],
            }),
          },
        ],
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"10","2":"10","3":"10","4":"0"}` &&
            JSON.stringify(state.total_stake) === `"0"` &&
            JSON.stringify(state.last_cycle) === `"4"` &&
            JSON.stringify(state.stakers_bal) === `{}` &&
            JSON.stringify(state.stakers_total_bal) === `{}` &&
            JSON.stringify(state.last_deposit_cycle) === `{}`
          );
        },
      },
    },
    {
      name: "deposit again",
      transition: "deposit",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({
        amount: ["Uint128", 10],
      }),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"10","2":"10","3":"10","4":"10"}` &&
            JSON.stringify(state.total_stake) === `"10"` &&
            JSON.stringify(state.last_cycle) === `"4"` &&
            JSON.stringify(state.stakers_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":{"4":"10"}}` &&
            JSON.stringify(state.stakers_total_bal) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"10"}` &&
            JSON.stringify(state.last_deposit_cycle) ===
              `{"${getTestAddr(OWNER).toLocaleLowerCase()}":"4"}`
          );
        },
      },
    },
    {
      // case tested already, here just for increasing cycles
      name: "claim after 10 cycles",
      transition: "claim",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: async () => {
        await increaseBNum(zilliqa, 100);
      },
      error: undefined,
    },
    {
      name: "withdraw by loss should fail",
      transition: "withdraw_by_loss",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: STAKING_ERROR.OutofLockupPeriod,
    },
  ];

  for (const testCase of testCases) {
    it(`${testCase.transition}: ${testCase.name}`, async () => {
      await testCase.beforeTransition();

      zilliqa.wallet.setDefault(testCase.getSender());
      const tx: any = await zilliqa.contracts
        .at(globalStakingContractAddress)
        .call(
          testCase.transition,
          getJSONParams(testCase.getParams()),
          TX_PARAMS
        );
      console.log("transaction id = ", tx.id);
      if (testCase.error === undefined) {
        if (!tx.receipt.success) {
          throw new Error();
        }
      } else {
        expect(tx.receipt.exceptions[0].message).toBe(
          getErrorMsg(testCase.error)
        );
      }
      if (
        testCase.want !== undefined &&
        testCase.want.verifyState !== undefined
      ) {
        const state = await zilliqa.contracts
          .at(globalStakingContractAddress)
          .getState();
        expect(testCase.want.verifyState(state)).toBe(true);
      }
    });
  }
});
