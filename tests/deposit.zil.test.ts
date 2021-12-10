import { Zilliqa } from "@zilliqa-js/zilliqa";
import fs from "fs";
import { getAddressFromPrivateKey, schnorr } from "@zilliqa-js/crypto";

import {
  increaseBNum,
  getJSONParams
} from "./testutil";

import {
  //CONTAINER,
  API,
  TX_PARAMS,
  CONTRACTS,
  FAUCET_PARAMS,
  asyncNoop,
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
      staking: globalStakingContractAddress
    });

    zilliqa.wallet.setDefault(getTestAddr(OWNER));
    const tx0: any = await zilliqa.contracts
        .at(globalToken0ContractAddress)
        .call(
          "IncreaseAllowance",
          getJSONParams({
            spender: ["ByStr20", globalStakingContractAddress],
            amount: ["Uint128", Number(100000000000000).toString()]
          }),
          TX_PARAMS
    );
    if (!tx0.receipt.success) {
      throw new Error();
    }
    console.log(tx0.receipt);

    const tx1: any = await zilliqa.contracts
      .at(globalToken1ContractAddress)
      .call(
        "IncreaseAllowance",
        getJSONParams({
          spender: ["ByStr20", globalStakingContractAddress],
          amount: ["Uint128", Number(100000000000000).toString()]
        }),
        TX_PARAMS
    );
    if (!tx1.receipt.success) {
      throw new Error();
    }
    console.log(tx1.receipt);

    const tx2: any = await zilliqa.contracts
      .at(globalToken2ContractAddress)
      .call(
        "IncreaseAllowance",
        getJSONParams({
          spender: ["ByStr20", globalStakingContractAddress],
          amount: ["Uint128", Number(100000000000000).toString()]
        }),
        TX_PARAMS
    );
    if (!tx2.receipt.success) {
      throw new Error();
    }
    console.log(tx2.receipt);
});


describe("staking contract", () => {
    const testCases = [
        {
          name: "unpause",
          transition: "unpause",
          getSender: () => getTestAddr(OWNER),
          beforeTransition: asyncNoop,
          error: undefined,
          getParams: () => ({}),
          want: undefined
        },
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
                JSON.stringify(state.total_stake_per_cycle) ===
                  `{"1":"10"}` &&
                JSON.stringify(state.total_stake) === `"10"`
              )
            }
          }
        },
        {
          name: "deposit multiple times before next cycle",
          transition: "deposit",
          getSender: () => getTestAddr(OWNER),
          getParams: () => ({
            amount: ["Uint128", 10],
          }),
          beforeTransition: async () => {
            await increaseBNum(zilliqa, 5);
          },
          error: undefined,
          want: {
            verifyState: (state) => {
              return (
                JSON.stringify(state.total_stake_per_cycle) ===
                  `{"1":"20"}` &&
                  JSON.stringify(state.total_stake) === `"20"`
              )
            }
          }
        },
        {
          name: "deposit multiple times crross next cycle",
          transition: "deposit",
          getSender: () => getTestAddr(OWNER),
          getParams: () => ({
            amount: ["Uint128", 10],
          }),
          beforeTransition: async () => {
            await increaseBNum(zilliqa, 10);
          },
          error: undefined,
          want: {
            verifyState: (state) => {
              return (
                JSON.stringify(state.total_stake_per_cycle) ===
                  `{"1":"20","2":"30"}` &&
                  JSON.stringify(state.total_stake) === `"30"`
              )
            }
          },
        },
        {
          name: "cross multiple cycle",
          transition: "deposit",
          getSender: () => getTestAddr(OWNER),
          getParams: () => ({
            amount: ["Uint128", 10],
          }),
          beforeTransition: async () => {
            await increaseBNum(zilliqa, 33);
          },
          error: undefined,
          want: {
            verifyState: (state) => {
              return (
                JSON.stringify(state.total_stake_per_cycle) ===
                  `{"1":"20","2":"30","3":"30","4":"30","5":"40"}` &&
                  JSON.stringify(state.total_stake) === `"40"`
              )
            }
          },
        },
    ];

    for (const testCase of testCases) {
        it(`${testCase.transition}: ${testCase.name}`, async () => {
            console.log("testing: ", testCase.name);
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
            console.log(tx.receipt);

            if (testCase.want !== undefined && testCase.want.verifyState !== undefined) {
              const state = await zilliqa.contracts
              .at(globalStakingContractAddress)
              .getState();
              expect(testCase.want.verifyState(state)).toBe(true);
            }
        });
    }

});
