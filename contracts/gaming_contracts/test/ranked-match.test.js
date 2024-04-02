const { deployFungibleToken } = require("../scripts/deployFungibleToken.js");
const {
  deployRankedMatchContract,
} = require("../scripts/deployRankedMatchContract.js");
const { BN, bytes } = require("@zilliqa-js/util");
const {
  getAccounts,
  toToken,
  fromZil,
  signData,
  getMetaData,
} = require("../scripts/utils/helper");
const accounts = getAccounts();
const metaData = getMetaData();
const {
  callContract,
  sendZil,
  getBalance,
  getSmartContractState,
} = require("../scripts/utils/call.js");

const currency = {
  ZIL: 100,
  TOKEN: 2,
  COMMISSION: 3,
};

function serializeData(data) {
  const matchIdHexArray = bytes.intToHexArray(data.matchId, 32);

  // Convert wallet address to hex array
  let walletAddress = data.ownerWalletAddress;
  if (walletAddress.startsWith("0x")) {
    walletAddress = walletAddress.substring(2);
  }

  const walletAddressHexArray = [walletAddress];

  // Convert ZRC6 smart contract address to hex array
  let playerWalletAddress = data.playerWalletAddress;
  if (playerWalletAddress.startsWith("0x")) {
    playerWalletAddress = playerWalletAddress.substring(2);
  }

  const playerWalletAddressHexArray = [playerWalletAddress];

  // Concat data to serialize
  let serializeData = matchIdHexArray
    .concat(walletAddressHexArray)
    .concat(playerWalletAddressHexArray)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

async function startMatch(privKey, address, matchAddress, player_list) {
  const toDataSignDataPairArr = [];
  for (let i in player_list) {
    let v = player_list[i];
    const serialize = await serializeData({
      ownerWalletAddress: address,
      playerWalletAddress: v,
      matchId: "0",
    });
    const signedData = signData(serialize);
    const pair = await createDataSignDataPairADT(serialize, signedData);
    toDataSignDataPairArr.push(pair);
  }

  console.log("toDataSignDataPairArr", JSON.stringify(toDataSignDataPairArr));

  return await callContract(
    privKey,
    matchAddress,
    "StartMatch",
    [
      {
        vname: "data_sig_pair_list",
        type: "List (Pair ByStr ByStr64)",
        value: toDataSignDataPairArr,
      },
    ],
    0,
    false,
    true
  );
}

async function endMatch(privKey, address, matchAddress, match_id, player_list) {
  const toDataSignDataPairArr = [];
  for (let i in player_list) {
    let v = player_list[i];
    const serialize = await serializeData({
      ownerWalletAddress: address,
      playerWalletAddress: v,
      matchId: match_id,
    });
    const signedData = signData(serialize);
    const pair = await createDataSignDataPairADT(serialize, signedData);
    toDataSignDataPairArr.push(pair);
  }

  console.log("toDataSignDataPairArr", JSON.stringify(toDataSignDataPairArr));

  return await callContract(
    privKey,
    matchAddress,
    "EndMatch",
    [
      {
        vname: "data_sig_pair_list",
        type: "List (Pair ByStr ByStr64)",
        value: toDataSignDataPairArr,
      },
    ],
    0,
    false,
    true
  );
}

function getTimestampInSeconds() {
  return new Date().getTime() * 1000;
}

async function createDataSignDataPairADT(data, sigData) {
  return {
    constructor: "Pair",
    argtypes: ["ByStr", "ByStr64"],
    arguments: [data, sigData],
  };
}

let player_list = [];
let zrc2tokenAddress;
let tokenDecimal;
let matchContractAddress;
describe("Ranked Match smart contract: Test all the transition of ranked match contract.", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T1",
      symbol: null,
      decimals: 6,
      supply: new BN("10000000000000000"),
      dexCheck: "True",
    };
    const [zrc2deployedToken] = await deployFungibleToken(
      accounts[0].privateKey,
      fungibleTokenDeployParams,
      accounts[0].address
    );
    zrc2tokenAddress = zrc2deployedToken.address;
    tokenDecimal = fungibleTokenDeployParams.decimals;
    if (zrc2tokenAddress === undefined) {
      throw new Error("Failed to deploy zrc2 contract.");
    }
    console.log("zrc2tokenAddress", zrc2tokenAddress);

    const [matchContract] = await deployRankedMatchContract(
      accounts[0].privateKey,
      {
        entryFee: toToken(currency.TOKEN, tokenDecimal),
        tokenContract: zrc2tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    matchContractAddress = matchContract.address;
    console.log("matchContractAddress =", matchContractAddress);
    if (matchContractAddress === undefined) {
      throw new Error("Failed to deploy ranked match contract.");
    }

    console.log("Set the revenue wallet");
    await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "SetServiceFeeRecipient",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: accounts[1].address,
        },
      ],
      0,
      false,
      false
    );

    console.log("Set the Admin users in ranked match contract.");
    await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "AddAdmin",
      [
        {
          vname: "address",
          type: "ByStr20",
          value: accounts[1].address,
        },
      ],
      0,
      false,
      false
    );

    for (let i in accounts) {
      let v = accounts[i];

      console.log("Transfer token to user address");
      await callContract(
        accounts[0].privateKey,
        zrc2tokenAddress,
        "Transfer",
        [
          {
            vname: "to",
            type: "ByStr20",
            value: v.address,
          },
          {
            vname: "amount",
            type: "Uint128",
            value: toToken(currency.TOKEN, tokenDecimal),
          },
        ],
        0,
        false,
        false
      );
      // Send zils if player does not have zil balance for paying gas fee.
      let zilBalance = await getBalance(v.address);
      if (fromZil(zilBalance) < currency.ZIL) {
        await sendZil(
          accounts[0].privateKey,
          v.address,
          Math.floor(parseInt(currency.ZIL - fromZil(zilBalance))),
          40000
        );
      }

      await callContract(
        v.privateKey,
        zrc2tokenAddress,
        "IncreaseAllowance",
        [
          {
            vname: "spender",
            type: "ByStr20",
            value: matchContractAddress,
          },
          {
            vname: "amount",
            type: "Uint128",
            value: toToken(currency.TOKEN, tokenDecimal),
          },
        ],
        0,
        false,
        false
      );

      if (i > 1) player_list.push(v.address);
    }
  });

  afterEach(() => {
    player_list = [];
  });

  test("RM1: SetServiceFeeBPS.", async () => {
    let serviceFeeBps = 2000;
    const tx = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "SetServiceFeeBPS",
      [
        {
          vname: "fee_bps",
          type: "Uint128",
          value: `${serviceFeeBps}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`SetServiceFeeBPS: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "SetServiceFeeBPS",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "Uint128",
            value: `${serviceFeeBps}`,
            vname: "service_fee_bps",
          },
        ],
      },
    ]);
  });

  test("RM2: SetBurnTokenFeeBps.", async () => {
    let serviceFeeBps = 300;
    const tx = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "SetBurnTokenFeeBps",
      [
        {
          vname: "fee_bps",
          type: "Uint128",
          value: `${serviceFeeBps}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`SetBurnTokenFeeBps: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "SetBurnTokenFeeBps",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "Uint128",
            value: `${serviceFeeBps}`,
            vname: "burn_token_fee_bps",
          },
        ],
      },
    ]);
  });

  test("RM3: SetEntryFee.", async () => {
    let serviceFeeBps = 300;
    const tx = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "SetEntryFee",
      [
        {
          vname: "fee",
          type: "Uint128",
          value: `${serviceFeeBps}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`SetEntryFee: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "SetEntryFeeSuccess",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "Uint128",
            value: `${serviceFeeBps}`,
            vname: "entry_fee",
          },
        ],
      },
    ]);
  });

  test("RM4: SetServiceFeeRecipient.", async () => {
    const serviceFeeRecipient = accounts[2].address;
    const tx = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "SetServiceFeeRecipient",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: `${serviceFeeRecipient}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`SetServiceFeeRecipient: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "SetServiceFeeRecipient",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${serviceFeeRecipient.toLowerCase()}`,
            vname: "to",
          },
        ],
      },
    ]);
  });

  test("RM5: Test add and remove admin from ranked match contract.", async () => {
    const adminWalletAddress = accounts[1].address;
    const addAdminTxns = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "AddAdmin",
      [
        {
          vname: "address",
          type: "ByStr20",
          value: `${adminWalletAddress}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`AddAdmin: => tx: ${JSON.stringify(addAdminTxns.receipt)}`);
    expect(addAdminTxns.receipt.success).toEqual(true);
    expect(addAdminTxns.receipt.event_logs).toEqual([
      {
        _eventname: "AddAdminSuccess",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${adminWalletAddress.toLowerCase()}`,
            vname: "addressAdded",
          },
        ],
      },
    ]);

    const removeAdminTxns = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "RemoveAdmin",
      [{
        vname: "address",
        type: "ByStr20",
        value: `${adminWalletAddress}`,
      }],
      0,
      false,
      true
    );

    console.log(
      `RemoveAdmin: => tx: ${JSON.stringify(removeAdminTxns.receipt)}`
    );
    expect(removeAdminTxns.receipt.success).toEqual(true);
    expect(removeAdminTxns.receipt.event_logs).toEqual([
      {
        _eventname: "RemoveAdminSuccess",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${adminWalletAddress.toLowerCase()}`,
            vname: "addressRemoved",
          },
        ],
      },
    ]);
  });
  test("RM6: Test accept contract ownership.", async () => {
    const AddContractOwnershipRecipienttx = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "AddContractOwnershipRecipient",
      [
        {
          vname: "address",
          type: "ByStr20",
          value: accounts[1].address,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `AcceptContractOwnershipTransfer: AddContractOwnershipRecipientTx: ${JSON.stringify(
        AddContractOwnershipRecipienttx.receipt
      )}`
    );
    expect(AddContractOwnershipRecipienttx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[1].privateKey,
      matchContractAddress,
      "AcceptContractOwnershipTransfer",
      [],
      0,
      false,
      true
    );

    console.log(
      `AcceptContractOwnershipTransfer: Success tx ====>: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "AcceptContractOwnershipTransferSuccess",
        address: `${matchContractAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${accounts[1].address.toLowerCase()}`,
            vname: "recipient_address",
          },
        ],
      },
    ]);
  });
  test("RM7: Start Match.", async () => {
    const tx = await startMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      player_list
    );

    const tokenBalance = await getSmartContractState(zrc2tokenAddress);

    console.log(`StartMatch: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tokenBalance.balances[matchContractAddress.toLowerCase()]).toEqual(
      toToken(currency.TOKEN * player_list.length, tokenDecimal)
    );
    expect(tx.receipt.event_logs[0]).toEqual({
      _eventname: "StartMatch",
      address: matchContractAddress.toLowerCase(),
      params: [
        {
          type: "String",
          value: "Match started Successfully",
          vname: "status",
        },
        {
          type: "ByStr20",
          value: `${accounts[1].address.toLowerCase()}`,
          vname: "initiator",
        },
        {
          type: "List (ByStr20)",
          value: player_list.map((row) => row.toLowerCase()),
          vname: "player_list",
        },
        {
          type: "Uint256",
          value: "1",
          vname: "match_id",
        },
      ],
    });
  });

  test("RM8: EndMatch.", async () => {
    const startMatcRxns = await startMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      player_list
    );

    expect(startMatcRxns.receipt.success).toEqual(true);

    const winning_list = [
      player_list[2].toLowerCase(),
      player_list[3].toLowerCase(),
    ];

    const tx = await endMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      1,
      winning_list
    );

    console.log(`EndMatch: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs[0]).toEqual({
      _eventname: "EndMatch",
      address: matchContractAddress.toLowerCase(),
      params: [
        {
          type: "String",
          value: "Match finished Successfully",
          vname: "status",
        },
        {
          type: "ByStr20",
          value: `${accounts[1].address.toLowerCase()}`,
          vname: "initiator",
        },
        {
          type: "Uint128",
          value: "4000000",
          vname: "reward_amount",
        },
        {
          type: "List (ByStr20)",
          value: winning_list,
          vname: "winners_list",
        },
      ],
    });
  });

  test("RM9: WithdrawRewardByPlayer.", async () => {
    const startMatchTxns = await startMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      player_list
    );

    expect(startMatchTxns.receipt.success).toEqual(true);

    const matchId = 1;
    const winners_list = [player_list[0], player_list[1]];
    const endMatchTxns = await endMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      matchId,
      winners_list
    );

    console.log(`EndMatch: => tx: ${JSON.stringify(endMatchTxns.receipt)}`);
    expect(endMatchTxns.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[2].privateKey,
      matchContractAddress,
      "WithdrawRewardByPlayer",
      [
        {
          vname: "match_id",
          type: "Uint256",
          value: `${matchId}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`WithdrawRewards: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs[0]).toEqual({
      _eventname: "WithdrawRewardsSuccess",
      address: matchContractAddress.toLowerCase(),
      params: [
        {
          type: "String",
          value: "Reward withdrawal was Successful.",
          vname: "status",
        },
        {
          type: "Uint256",
          value: `${matchId}`,
          vname: "match_id",
        },
        {
          type: "ByStr20",
          value: `${accounts[2].address.toLowerCase()}`,
          vname: "initiator",
        },
        {
          type: "Uint128",
          value: "100",
          vname: "pl_fee_bps",
        },
        {
          type: "Uint128",
          value: "40000",
          vname: "penalty_amount",
        },
        {
          type: "ByStr20",
          value: `${accounts[1].address.toLowerCase()}`,
          vname: "penalty_fee_recipient",
        },
        {
          type: "Uint128",
          value: "1000",
          vname: "svc_fee_bps",
        },
        {
          type: "Uint128",
          value: "400000",
          vname: "service_amount",
        },
        {
          type: "ByStr20",
          value: `${accounts[1].address.toLowerCase()}`,
          vname: "svc_fee_recipient",
        },
        {
          type: "Uint128",
          value: "200",
          vname: "burn_fee_bps",
        },
        {
          type: "Uint128",
          value: "80000",
          vname: "burn_amount",
        },
        {
          type: "Uint128",
          value: "3480000",
          vname: "reward_amount",
        },
      ],
    });
  });

  test("RM10: WithdrawRewardByAdmin: Should fail as we have send a empty array of player wallet address.", async () => {
    const startMatchTxns = await startMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      player_list
    );

    expect(startMatchTxns.receipt.success).toEqual(true);

    const matchId = 1;
    const endMatchTxns = await endMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      matchId,
      [player_list[0], player_list[1]]
    );

    console.log(`EndMatch: => tx: ${JSON.stringify(endMatchTxns.receipt)}`);
    expect(endMatchTxns.receipt.success).toEqual(true);

    const { rewards } = await getSmartContractState(matchContractAddress);

    console.log("rewards", rewards);

    const currentTimeStamp = getTimestampInSeconds();
    const toDataSignDataPairArr = [];
    for (let key in rewards) {
      reward = rewards[key];
      console.log("key", key, "reward", reward);
      for (let k in reward) {
        const v = reward[k];
        const [end_timestamp, rewardAmount] = v.arguments;
        if (end_timestamp < currentTimeStamp) {
          const serialize = await serializeData({
            ownerWalletAddress: accounts[1].address,
            playerWalletAddress: key,
            matchId: k,
          });
          const signedData = signData(serialize);
          const pair = await createDataSignDataPairADT(serialize, signedData);
          toDataSignDataPairArr.push(pair);
        }
      }
    }

    console.log("toDataSignDataPairArr", toDataSignDataPairArr);

    const tx = await callContract(
      accounts[1].privateKey,
      matchContractAddress,
      "WithdrawRewardByAdmin",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toDataSignDataPairArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(`WithdrawRewards: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(false);
    expect(tx.receipt.exceptions[0]).toEqual({
      line: 1,
      message:
        'Exception thrown: (Message [(_exception : (String "Error")) ; (code : (Int32 -10))])',
    });
  });
  test("RM11: Cancel Match.", async () => {
    const startTxns = await startMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      player_list
    );

    console.log(
      `CancelMatch: => startTxns: ${JSON.stringify(startTxns.receipt)}`
    );
    expect(startTxns.receipt.success).toEqual(true);

    const matchId = 1;
    const endTxns = await callContract(
      accounts[1].privateKey,
      matchContractAddress,
      "CancelMatch",
      [
        {
          vname: "match_id",
          type: "Uint256",
          value: `${matchId}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`EndMatch: => endTxns: ${JSON.stringify(endTxns.receipt)}`);
    expect(endTxns.receipt.success).toEqual(true);
    expect(endTxns.receipt.event_logs[0]).toEqual({
      _eventname: "CancelMatch",
      address: `${matchContractAddress.toLowerCase()}`,
      params: [
        {
          type: "String",
          value: "Match cancelled Successfully",
          vname: "status",
        },
        {
          type: "ByStr20",
          value: `${accounts[1].address.toLowerCase()}`,
          vname: "initiator",
        },
        {
          type: "Uint256",
          value: `${matchId}`,
          vname: "match_id",
        },
        {
          type: "List (ByStr20)",
          value: player_list.map((row) => row.toLowerCase()),
          vname: "players",
        },
        {
          type: "Uint128",
          value: "1000",
          vname: "svc_fee_bps",
        },
        {
          type: "Uint128",
          value: "800000",
          vname: "service_amount",
        },
        {
          type: "ByStr20",
          value: `${accounts[1].address.toLowerCase()}`,
          vname: "svc_fee_recipient",
        },
        {
          type: "Uint128",
          value: "7200000",
          vname: "total_refund_amount",
        },
      ],
    });
    const zrc2ContractState = await getSmartContractState(zrc2tokenAddress);
    for (let i in player_list) {
      let v = player_list[i].toLowerCase();
      expect(zrc2ContractState.balances[v]).toEqual("1800000");
    }
  });

  test("RM12: Withdraw all funds in case of hack.", async () => {
    const startTxns = await startMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      player_list
    );

    console.log(
      `WithdrawToken: => startTxns: ${JSON.stringify(startTxns.receipt)}`
    );
    expect(startTxns.receipt.success).toEqual(true);

    const matchId = 1;
    const endMatchTxns = await endMatch(
      accounts[1].privateKey,
      accounts[1].address,
      matchContractAddress,
      matchId,
      [player_list[0], player_list[1]]
    );

    console.log(`EndMatch: => tx: ${JSON.stringify(endMatchTxns.receipt)}`);
    expect(endMatchTxns.receipt.success).toEqual(true);

    const withdrawTxns = await callContract(
      accounts[0].privateKey,
      matchContractAddress,
      "WithdrawToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN * player_list.length, tokenDecimal),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `EndMatch: => withdrawTxns: ${JSON.stringify(withdrawTxns.receipt)}`
    );
    expect(withdrawTxns.receipt.success).toEqual(true);
    expect(withdrawTxns.receipt.event_logs[0]).toEqual({
      _eventname: "WithdrawAllFundsSuccess",
      address: `${matchContractAddress.toLowerCase()}`,
      params: [
        {
          type: "Uint128",
          value: toToken(currency.TOKEN * player_list.length, tokenDecimal),
          vname: "amount",
        },
      ],
    });
  });
});
