import { Zilliqa } from "@zilliqa-js/zilliqa";
import fs from "fs";
import { getAddressFromPrivateKey, schnorr } from "@zilliqa-js/crypto";

import {
  increaseBNum,
  getJSONParams,
  verifyCheckRewardsEvents,
  verifyEvents,
  getErrorMsg,
} from "./testutil";

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
const Alice = 1;
const Bob = 2;

const getTestAddr = (index) => globalTestAccounts[index]!.address as string;

beforeAll(async () => {
  let contract;
  const accounts = Array.from({ length: 3 }, schnorr.generatePrivateKey).map(
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
    Alice: getTestAddr(Alice),
    Bob: getTestAddr(Bob),
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
      amount: ["Uint128", 10000000000000],
    }),
    TX_PARAMS
  );
  if (!tx0.receipt.success) {
    throw new Error();
  }

  const tx1: any = await zilliqa.contracts.at(globalToken1ContractAddress).call(
    "Transfer",
    getJSONParams({
      to: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 100000000000000],
    }),
    TX_PARAMS
  );
  if (!tx1.receipt.success) {
    throw new Error();
  }

  const tx2: any = await zilliqa.contracts.at(globalToken2ContractAddress).call(
    "Transfer",
    getJSONParams({
      to: ["ByStr20", globalStakingContractAddress],
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

  const tx5: any = await zilliqa.contracts.at(globalToken0ContractAddress).call(
    "Transfer",
    getJSONParams({
      to: ["ByStr20", accounts[Alice]!.address],
      amount: ["Uint128", 10000000000000],
    }),
    TX_PARAMS
  );
  if (!tx5.receipt.success) {
    throw new Error();
  }

  const tx6: any = await zilliqa.contracts.at(globalToken0ContractAddress).call(
    "Transfer",
    getJSONParams({
      to: ["ByStr20", accounts[Bob]!.address],
      amount: ["Uint128", 10000000000000],
    }),
    TX_PARAMS
  );
  if (!tx6.receipt.success) {
    throw new Error();
  }

  zilliqa.wallet.setDefault(getTestAddr(Alice));
  const tx7: any = await zilliqa.contracts.at(globalToken0ContractAddress).call(
    "IncreaseAllowance",
    getJSONParams({
      spender: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 10000000000000],
    }),
    TX_PARAMS
  );
  if (!tx7.receipt.success) {
    throw new Error();
  }

  zilliqa.wallet.setDefault(getTestAddr(Bob));
  const tx8: any = await zilliqa.contracts.at(globalToken0ContractAddress).call(
    "IncreaseAllowance",
    getJSONParams({
      spender: ["ByStr20", globalStakingContractAddress],
      amount: ["Uint128", 10000000000000],
    }),
    TX_PARAMS
  );
  if (!tx8.receipt.success) {
    throw new Error();
  }
});

describe("staking contract", () => {
  const testCases = [
    {
      name: "deposit from owner",
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
      name: "deposit from alice",
      transition: "deposit",
      getSender: () => getTestAddr(Alice),
      getParams: () => ({
        amount: ["Uint128", 10],
      }),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) === `{"1":"20"}` &&
            JSON.stringify(state.total_stake) === `"20"` &&
            JSON.stringify(state.last_cycle) === `"1"`
          );
        },
      },
    },
    {
      name: "deposit from bob",
      transition: "deposit",
      getSender: () => getTestAddr(Bob),
      getParams: () => ({
        amount: ["Uint128", 10],
      }),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) === `{"1":"30"}` &&
            JSON.stringify(state.total_stake) === `"30"` &&
            JSON.stringify(state.last_cycle) === `"1"`
          );
        },
      },
    },
    {
      name: "check rewards from bob after 1",
      transition: "check_rewards",
      getSender: () => getTestAddr(Bob),
      getParams: () => ({}),
      beforeTransition: async () => {
        await increaseBNum(zilliqa, 10);
      },
      error: undefined,
      want: {
        events: [
          {
            name: "check_rewards",
            getParams: () => ({
              rewards: [
                "List (Pair (ByStr20) (Uint128))",
                [
                  [globalToken1ContractAddress, "3333333333333"],
                  [globalToken2ContractAddress, "3333333333333"],
                ],
              ],
            }),
          },
        ],
        verifyEvents: verifyCheckRewardsEvents,
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"30"}` &&
              JSON.stringify(state.total_stake) === `"30"`,
            JSON.stringify(state.last_cycle) === `"2"`
          );
        },
      },
    },
    {
      name: "deposit from bob again",
      transition: "deposit",
      getSender: () => getTestAddr(Bob),
      getParams: () => ({
        amount: ["Uint128", 10],
      }),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"40"}` &&
              JSON.stringify(state.total_stake) === `"40"`,
            JSON.stringify(state.last_cycle) === `"2"`
          );
        },
      },
    },
    {
      name: "check rewards from bob after 1",
      transition: "check_rewards",
      getSender: () => getTestAddr(Bob),
      getParams: () => ({}),
      beforeTransition: async () => {
        await increaseBNum(zilliqa, 10);
      },
      error: undefined,
      want: {
        events: [
          {
            name: "check_rewards",
            // cycle 1: owner 10 alice 10 bob 10 => bob's rewards: 3333333333333
            // cycle 2: owner 10 alice 10 bob 20 => bob's rewards: 5000000000000
            getParams: () => ({
              rewards: [
                "List (Pair (ByStr20) (Uint128))",
                [
                  [globalToken1ContractAddress, "8333333333333"],
                  [globalToken2ContractAddress, "8333333333333"],
                ],
              ],
            }),
          },
        ],
        verifyEvents: verifyCheckRewardsEvents,
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"40","3":"40"}` &&
            JSON.stringify(state.total_stake) === `"40"` &&
            JSON.stringify(state.last_cycle) === `"3"`
          );
        },
      },
    },
    {
      name: "claim from bob after 1",
      transition: "claim",
      getSender: () => getTestAddr(Bob),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        events: [
          {
            name: "RewardClaim",
            getParams: () => ({
              reward_list: ["List (Uint32)", ["1", "2"]],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Bob)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Bob)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Bob)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Bob)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
        ],
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"40","3":"40"}` &&
            JSON.stringify(state.total_stake) === `"40"` &&
            JSON.stringify(state.last_cycle) === `"3"`
          );
        },
      },
    },
    {
      name: "withdraw from bob",
      transition: "withdraw",
      getSender: () => getTestAddr(Bob),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: STAKING_ERROR.StillInLockupPeriod,
    },
    {
      name: "check rewards from alice after 1",
      transition: "check_rewards",
      getSender: () => getTestAddr(Alice),
      getParams: () => ({}),
      beforeTransition: async () => {
        await increaseBNum(zilliqa, 10);
      },
      error: undefined,
      want: {
        events: [
          {
            name: "check_rewards",
            // cycle 1: owner 10 alice 10 bob 10 => alice's rewards: 3333333333333
            // cycle 2: owner 10 alice 10 bob 20 => alice's rewards: 2500000000000
            // cycle 3: owner 10 alice 10 bob 20 => alice's rewards: 2500000000000
            getParams: () => ({
              rewards: [
                "List (Pair (ByStr20) (Uint128))",
                [
                  [globalToken1ContractAddress, "8333333333333"],
                  [globalToken2ContractAddress, "8333333333333"],
                ],
              ],
            }),
          },
        ],
        verifyEvents: verifyCheckRewardsEvents,
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"40","3":"40","4":"40"}` &&
            JSON.stringify(state.total_stake) === `"40"` &&
            JSON.stringify(state.last_cycle) === `"4"`
          );
        },
      },
    },
    {
      name: "claim from alice",
      transition: "claim",
      getSender: () => getTestAddr(Alice),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        events: [
          {
            name: "RewardClaim",
            getParams: () => ({
              reward_list: ["List (Uint32)", ["1", "2", "3"]],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Alice)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Alice)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Alice)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(Alice)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
        ],
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"40","3":"40","4":"40"}` &&
            JSON.stringify(state.total_stake) === `"40"` &&
            JSON.stringify(state.last_cycle) === `"4"`
          );
        },
      },
    },
    {
      name: "claim from owner",
      transition: "claim",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: undefined,
      want: {
        events: [
          {
            name: "RewardClaim",
            getParams: () => ({
              reward_list: ["List (Uint32)", ["1", "2", "3"]],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(OWNER)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(OWNER)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccess",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(OWNER)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
          {
            name: "TransferSuccessCallBack",
            getParams: () => ({
              sender: ["ByStr20", globalStakingContractAddress],
              recipient: ["ByStr20", getTestAddr(OWNER)],
              amount: ["Uint128", "8333333333333"],
            }),
          },
        ],
        verifyState: (state) => {
          return (
            JSON.stringify(state.total_stake_per_cycle) ===
              `{"1":"30","2":"40","3":"40","4":"40"}` &&
            JSON.stringify(state.total_stake) === `"40"` &&
            JSON.stringify(state.last_cycle) === `"4"`
          );
        },
      },
    },
    {
      name: "withdraw from alice",
      transition: "withdraw",
      getSender: () => getTestAddr(Alice),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: STAKING_ERROR.StillInLockupPeriod,
    },
    {
      name: "withdraw from owner",
      transition: "withdraw",
      getSender: () => getTestAddr(OWNER),
      getParams: () => ({}),
      beforeTransition: asyncNoop,
      error: STAKING_ERROR.StillInLockupPeriod,
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
      const state = await zilliqa.contracts
        .at(globalStakingContractAddress)
        .getState();

      if (
        testCase.want !== undefined &&
        testCase.want.verifyState !== undefined
      ) {
        expect(testCase.want.verifyState(state)).toBe(true);
      }
      if (testCase.want !== undefined && testCase.want.events !== undefined) {
        const checkEvents = testCase.want.verifyEvents || verifyEvents;
        expect(checkEvents(tx.receipt.event_logs, testCase.want.events)).toBe(
          true
        );
      }
    });
  }
});
