const { deployFungibleToken } = require("../scripts/deployFungibleToken.js");
const {
  deployTokenVestingContract,
} = require("../scripts/deployTokenVestingContract.js");
const {
  toToken,
  serializeClaimData,
  signData,
  getAccounts,
  getMetaData,
  fromZil,
} = require("../scripts/utils/helper");
const { BN, units } = require("@zilliqa-js/util");
const {
  callContract,
  sendZil,
  getBalance,
} = require("../scripts/utils/call.js");
const log = console.log;
const BATCH_SIZE_LIMIT = 5;

function getTimestampInSeconds() {
  return Math.floor(Date.now() / 1000);
}

const currency = {
  ZIL: 10,
  TOKEN: 10,
  COMMISSION: 3,
};

const accounts = getAccounts();
const metaData = getMetaData();

async function createVestingItemParam(
  contractAddress,
  userWalletAddress,
  amount,
  startTime,
  expirationTime
) {
  return {
    constructor: `${contractAddress.toLowerCase()}.VestingParam`,
    argtypes: [],
    arguments: [
      userWalletAddress.toLowerCase(),
      amount,
      startTime,
      expirationTime,
    ],
  };
}

async function createBatchVestingItemParams(
  vestingContractAddress,
  walletAddress,
  token,
  startTime,
  expirationTime
) {
  const vestingItemList = [];
  let totalTokens = 0;
  for (let i = 0; i < BATCH_SIZE_LIMIT; i++) {
    let newCurrentTimeStamp = expirationTime + i * 24 * 60 * 60;
    const formattedAdtVestingParam = await createVestingItemParam(
      vestingContractAddress,
      walletAddress,
      token,
      `${startTime}`,
      `${newCurrentTimeStamp}`
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

describe("Token Vesting => SetTokenVesting", () => {
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

    const [vestingContract] = await deployTokenVestingContract(
      accounts[0].privateKey,
      accounts[0].address,
      tokenAddress,
      metaData.pubKey
    );
    vestingContractAddress = vestingContract.address;
    log("vestingContractAddress =", vestingContractAddress);
    if (vestingContractAddress === undefined) {
      throw new Error("Failed to deploy token vesting contract.");
    }

    // Funds all the account to paying the gas fee. This is only executed when the zil balance is below 100ZILs
    for (let i in accounts) {
      let v = accounts[i];
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
    }
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
      `${currentTimeStamp}`
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
});

describe("Token Vesting => BatchSetTokenVesting", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T2",
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

    currentTimeStamp = getTimestampInSeconds();

    console.log("BatchSetTokenVesting: currentTimeStamp", currentTimeStamp);

    log("BatchSetTokenVesting: tokenAddress", tokenAddress);

    const [vestingContract] = await deployTokenVestingContract(
      accounts[0].privateKey,
      accounts[0].address,
      tokenAddress,
      metaData.pubKey
    );
    vestingContractAddress = vestingContract.address;
    log(
      "BatchSetTokenVesting: vestingContractAddress =",
      vestingContractAddress
    );
    if (vestingContractAddress === undefined) {
      throw new Error("Failed to deploy token vesting contract.");
    }

    // Funds all the account to paying the gas fee. This is only executed when the zil balance is below 100ZILs
    for (let i in accounts) {
      let v = accounts[i];
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
    }
  });

  test("BatchSetTokenVesting: When the transaction is successful.", async () => {
    const vestingListParams = await createBatchVestingItemParams(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      currentTimeStamp,
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
});

describe("Token Vesting => Claim", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T3",
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

    log("Claim: tokenAddress", tokenAddress);

    currentTimeStamp = getTimestampInSeconds();

    console.log("Claim: currentTimeStamp", currentTimeStamp);

    const [vestingContract] = await deployTokenVestingContract(
      accounts[0].privateKey,
      accounts[0].address,
      tokenAddress,
      metaData.pubKey
    );
    vestingContractAddress = vestingContract.address;
    log("Claim: vestingContractAddress =", vestingContractAddress);
    if (vestingContractAddress === undefined) {
      throw new Error("Failed to deploy token vesting contract.");
    }

    // Funds all the account to paying the gas fee. This is only executed when the zil balance is below 100ZILs
    for (let i in accounts) {
      let v = accounts[i];
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
    }
  });

  test("Claim: When the contract is paused should throws RequireNotPaused and code 2", async () => {
    const pauseTx = await callContract(
      accounts[0].privateKey,
      vestingContractAddress,
      "Pause",
      [],
      0,
      false,
      false
    );
    console.log(`Claim: paused  => tx: ${JSON.stringify(pauseTx.receipt)}`);
    expect(pauseTx.receipt.success).toEqual(true);

    const vestingListParams = await createBatchVestingItemParams(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      currentTimeStamp,
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

    const serialize = await serializeClaimData(currentTimeStamp);
    log("Claim: timestamp signature", serialize);

    const signedData = signData(serialize);
    log("Claim: signedData", signedData);

    log("Claim: Claim token by user from vesting contract.");
    const tx = await callContract(
      accounts[3].privateKey,
      vestingContractAddress,
      "Claim",
      [
        {
          vname: "data",
          type: "ByStr",
          value: serialize,
        },
        {
          vname: "sig_data",
          type: "ByStr64",
          value: signedData,
        },
      ],
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
    const vestingListParams = await createBatchVestingItemParams(
      vestingContractAddress,
      accounts[3].address,
      toToken(10, tokenDecimal),
      currentTimeStamp,
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

    const serialize = await serializeClaimData(currentTimeStamp);
    log("Claim: timestamp signature", serialize);

    const signedData = signData(serialize);
    log("Claim: signedData", signedData);

    log("Claim: Claim token by user from vesting contract.");
    const tx = await callContract(
      accounts[3].privateKey,
      vestingContractAddress,
      "Claim",
      [
        {
          vname: "data",
          type: "ByStr",
          value: serialize,
        },
        {
          vname: "sig_data",
          type: "ByStr64",
          value: signedData,
        },
      ],
      0,
      false,
      true
    );

    console.log(`Claim: Success tx =======>: : ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
  });
});
