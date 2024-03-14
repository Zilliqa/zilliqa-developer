const { deployFungibleToken } = require("../scripts/deployFungibleToken.js");
const {
  deployTokenVestingWithTimestampContract,
} = require("../scripts/deployTokenVestingWithTimestampContract.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const { toToken, getAccounts } = require("../scripts/utils/helper.js");
const { BN } = require("@zilliqa-js/util");
const { callContract } = require("../scripts/utils/call.js");
const log = console.log;
const BATCH_SIZE_LIMIT = 5;

function getTimestampInSeconds() {
  const todayDate = new Date().toISOString().slice(0, 10);
  return new Date(todayDate).getTime() * 1000;
}

const accounts = getAccounts();

async function createVestingItemParam(
  contractAddress,
  userWalletAddress,
  amount,
  startTime,
  expiryInDays
) {
  return {
    constructor: `${contractAddress.toLowerCase()}.VestingParam`,
    argtypes: [],
    arguments: [
      userWalletAddress.toLowerCase(),
      amount,
      startTime,
      expiryInDays,
    ],
  };
}

async function createBatchVestingItemParams(
  vestingContractAddress,
  walletAddress,
  token,
  startTime
) {
  const vestingItemList = [];
  let totalTokens = 0;
  for (let i = 0; i < BATCH_SIZE_LIMIT; i++) {
    const formattedAdtVestingParam = await createVestingItemParam(
      vestingContractAddress,
      walletAddress,
      token,
      `${startTime}`,
      `${i}`
    );

    console.log(
      "formattedAdtVestingParam",
      JSON.stringify(formattedAdtVestingParam)
    );

    totalTokens += parseInt(token);

    vestingItemList.push(formattedAdtVestingParam);
  }

  return {
    itemList: vestingItemList,
    totalTokens: totalTokens,
  };
}

let tokenAddress;
let tokenDecimal;
let vestingContractAddress;
let currentTimeStamp;

describe("Test all the Utility transactions in Token Vesting smart contract", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T1",
      symbol: null,
      decimals: 6,
      supply: new BN("10000000000000000"),
      dexCheck: "True",
    };
    const [deployedToken] = await deployFungibleToken(
      accounts[0].privateKey,
      fungibleTokenDeployParams,
      accounts[0].address
    );
    tokenAddress = deployedToken.address;
    tokenDecimal = fungibleTokenDeployParams.decimals;
    if (tokenAddress === undefined) {
      throw new Error("Failed to deploy zrc2 contract.");
    }

    log("tokenAddress", tokenAddress);

    currentTimeStamp = getTimestampInSeconds();

    console.log("currentTimeStamp", currentTimeStamp);

    const [vestingContract] = await deployTokenVestingWithTimestampContract(
      accounts[0].privateKey,
      accounts[0].address,
      tokenAddress
    );
    vestingContractAddress = vestingContract.address;
    log("vestingContractAddress =", vestingContractAddress);
    if (vestingContractAddress === undefined) {
      throw new Error("Failed to deploy token vesting contract.");
    }
  });

  test("Test the transfer ownership feature", async () => {
    const setOwnertxns = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "SetContractOwnershipRecipient",
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

    log("SetContractOwnershipRecipient: setOwnertxns", JSON.stringify(setOwnertxns));
    expect(setOwnertxns.receipt.success).toEqual(true);

    const claimOwnershiptxns = await callContract(
      accounts[1].privateKey,
      vestingContractAddress,
      "AcceptContractOwnership",
      [],
      0,
      false,
      false
    );

    log("AcceptContractOwnership: claimOwnershiptxns", JSON.stringify(claimOwnershiptxns));
    expect(claimOwnershiptxns.receipt.success).toEqual(true);
  });

  test("SetTokenVesting: When the transaction is successful.", async () => {
    log("SetTokenVesting: Transfer token to user address");
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: vestingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(10, tokenDecimal),
        },
      ],
      0,
      false,
      false
    );
    log("SetTokenVesting: tokenTransferTx", JSON.stringify(tokenTransferTx));
    expect(tokenTransferTx.receipt.success).toEqual(true);

    const currentTimeStamp = getTimestampInSeconds();

    console.log("SetTokenVesting: currentTimeStamp", currentTimeStamp);

    const formattedAdtVestingParam = await createVestingItemParam(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      `${currentTimeStamp}`,
      `${1}`
    );

    console.log(
      "SetTokenVesting: formattedAdtVestingParam",
      JSON.stringify(formattedAdtVestingParam)
    );

    const tx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "SetTokenVesting",
      [
        {
          vname: "vesting_param",
          type: `${vestingContractAddress}.VestingParam`,
          value: formattedAdtVestingParam,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `SetTokenVesting: Success tx =======>: : ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(true);
  });

  test("BatchSetTokenVesting: When the transaction is successful.", async () => {
    const vestingListParams = await createBatchVestingItemParams(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      currentTimeStamp
    );
    console.log(
      "BatchSetTokenVesting: vestingListParams",
      JSON.stringify(vestingListParams.itemList)
    );

    log("BatchSetTokenVesting: Transfer token to user address");
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: vestingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${vestingListParams.totalTokens}`,
        },
      ],
      0,
      false,
      false
    );
    log(
      "BatchSetTokenVesting: tokenTransferTx",
      JSON.stringify(tokenTransferTx)
    );
    expect(tokenTransferTx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "BatchSetTokenVesting",
      [
        {
          vname: "vesting_param_list",
          type: `List ${vestingContractAddress}.VestingParam`,
          value: vestingListParams.itemList,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `BatchSetTokenVesting: Success tx =======>: : ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(true);
  });

  test("Claim: When the contract is paused should throws RequireNotPaused and code 2", async () => {
    const vestingListParams = await createBatchVestingItemParams(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      currentTimeStamp
    );

    console.log(
      "Claim: vestingListParams",
      JSON.stringify(vestingListParams.itemList)
    );

    log("Claim: Set Token Vesting in batch");
    const AddVestingTx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "BatchSetTokenVesting",
      [
        {
          vname: "vesting_param_list",
          type: `List ${vestingContractAddress}.VestingParam`,
          value: vestingListParams.itemList,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `Claim: SetTokenVesting Tx =======>: : ${JSON.stringify(
        AddVestingTx.receipt
      )}`
    );
    expect(AddVestingTx.receipt.success).toEqual(true);

    log("Claim: Transfer token to user address");
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: vestingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${vestingListParams.totalTokens}`,
        },
      ],
      0,
      false,
      false
    );
    log("Claim: tokenTransferTx", JSON.stringify(tokenTransferTx));
    expect(tokenTransferTx.receipt.success).toEqual(true);

    log("Claim: Claim token by user from vesting contract.");
    const tx = await callContract(
      accounts[3].privateKey,
      vestingContractAddress,
      "Claim",
      [],
      0,
      false,
      true
    );

    console.log(
      `Claim: When the contract is paused => tx: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("Claim: When the transaction is successful.", async () => {
    const unPausetx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "Unpause",
      [],
      0,
      false,
      true
    );

    console.log(`UnPause: => unPausetx: ${JSON.stringify(unPausetx.receipt)}`);
    expect(unPausetx.receipt.success).toEqual(true);

    const vestingListParams = await createBatchVestingItemParams(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      currentTimeStamp
    );

    console.log(
      "Claim: vestingListParams",
      JSON.stringify(vestingListParams.itemList)
    );

    log("Claim: Set Token Vesting in batch");
    const AddVestingTx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "BatchSetTokenVesting",
      [
        {
          vname: "vesting_param_list",
          type: `List ${vestingContractAddress}.VestingParam`,
          value: vestingListParams.itemList,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `Claim: SetTokenVesting Tx =======>: : ${JSON.stringify(
        AddVestingTx.receipt
      )}`
    );
    expect(AddVestingTx.receipt.success).toEqual(true);

    log("Claim: Transfer token to user address");
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: vestingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${vestingListParams.totalTokens}`,
        },
      ],
      0,
      false,
      false
    );
    log("Claim: tokenTransferTx", JSON.stringify(tokenTransferTx));
    expect(tokenTransferTx.receipt.success).toEqual(true);

    currentTimeStamp = getTimestampInSeconds();

    console.log("Claim: currentTimeStamp", currentTimeStamp);

    log("Claim: Claim token by user from vesting contract.");
    const tx = await callContract(
      accounts[3].privateKey,
      vestingContractAddress,
      "Claim",
      [],
      0,
      false,
      true
    );

    console.log(`Claim: Success tx =======>: : ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
  });

  test("WithdrawTokens: Drain treasury balance in case of bug in the contract.", async () => {
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: vestingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(10, tokenDecimal),
        },
      ],
      0,
      false,
      false
    );
    log("SetTokenVesting: tokenTransferTx", JSON.stringify(tokenTransferTx));
    expect(tokenTransferTx.receipt.success).toEqual(true);

    const currentTimeStamp = getTimestampInSeconds();

    console.log("SetTokenVesting: currentTimeStamp", currentTimeStamp);

    const formattedAdtVestingParam = await createVestingItemParam(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      `${currentTimeStamp}`,
      `${1}`
    );

    console.log(
      "SetTokenVesting: formattedAdtVestingParam",
      JSON.stringify(formattedAdtVestingParam)
    );

    const tx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "SetTokenVesting",
      [
        {
          vname: "vesting_param",
          type: `${vestingContractAddress}.VestingParam`,
          value: formattedAdtVestingParam,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `SetTokenVesting: Success tx =======>: : ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(true);

    const withdrawTokensTx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "WithdrawTokens",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(10, tokenDecimal),
        },
      ],
      0,
      false,
      false
    );
    log("WithdrawTokens: withdrawTokensTx", JSON.stringify(withdrawTokensTx));
    expect(withdrawTokensTx.receipt.success).toEqual(true);
  });
});
