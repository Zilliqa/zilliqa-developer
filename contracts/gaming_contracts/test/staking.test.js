const { deployFungibleToken } = require("../scripts/deployFungibleToken.js");
const {
  deployNonFungibleToken,
} = require("../scripts/deployNonFungibleToken.js");
const {
  deployStakingContract,
} = require("../scripts/deployStakingContract.js");
const {
  deployStakingProxyContract,
} = require("../scripts/deployStakingProxyContract.js");
const {
  toToken,
  sleep,
  signData,
  toHexArray,
  getAccounts,
  getMetaData,
  fromZil,
} = require("../scripts/utils/helper");
const { BN, bytes } = require("@zilliqa-js/util");
const {
  callContract,
  getCurrentBlockNumber,
  getSmartContractState,
  getBalance,
  sendZil,
} = require("../scripts/utils/call.js");
const log = console.log;
const accounts = getAccounts();
const metaData = getMetaData();

const currency = {
  ZIL: 100,
  TOKEN: 10,
  COMMISSION: 3,
};

function serializeAddStakeData(data) {
  console.log("data", data);
  let paymentAmountHexArray;
  if (data.Amount) {
    paymentAmountHexArray = bytes.intToHexArray(data.Amount, 32);
  }

  // Convert wallet address to hex array
  let walletAddress = data.OwnerWalletAddress;
  if (walletAddress.startsWith("0x")) {
    walletAddress = walletAddress.substring(2);
  }

  const walletAddressHexArray = [walletAddress];

  // Convert wallet address to hex array
  let stakingContractAddress = data.StakingContractAddress;
  if (stakingContractAddress.startsWith("0x")) {
    stakingContractAddress = stakingContractAddress.substring(2);
  }

  const stakingContractHexArray = [stakingContractAddress];

  // Convert ZRC6 smart contract address to hex array
  let zrc6ContractAddress = data.ZRC6ContractAddress;
  if (zrc6ContractAddress.startsWith("0x")) {
    zrc6ContractAddress = zrc6ContractAddress.substring(2);
  }

  const zrc6AddressHexArray = [zrc6ContractAddress];

  let expiryBlockHexArray;
  if (data.ExpirationBnum) {
    expiryBlockHexArray = bytes.intToHexArray(data.ExpirationBnum, 32);
  }

  let penaltyFeeBpsHexArray;
  if (data.PenaltyFeeBps) {
    penaltyFeeBpsHexArray = bytes.intToHexArray(data.PenaltyFeeBps, 32);
  }

  let mintNftHexArray;
  if (data.MintNft != null || data.MintNft != undefined) {
    mintNftHexArray = bytes.intToHexArray(data.MintNft, 4);
  }

  // Convert token URI to hex string
  const tokenURIHexString = toHexArray(data.TokenURI);

  // Concat data to serialize
  let serializeData = paymentAmountHexArray
    .concat(walletAddressHexArray)
    .concat(stakingContractHexArray)
    .concat(zrc6AddressHexArray)
    .concat(expiryBlockHexArray)
    .concat(penaltyFeeBpsHexArray)
    .concat(mintNftHexArray)
    .concat(tokenURIHexString)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

function serializeRemoveStakeData(data) {
  console.log("serializeRemoveStakeData:", JSON.stringify(data));
  let paymentAmountHexArray;
  if (data.Amount) {
    paymentAmountHexArray = bytes.intToHexArray(data.Amount, 32);
  }

  // Convert wallet address to hex array
  let walletAddress = data.OwnerWalletAddress;
  if (walletAddress.startsWith("0x")) {
    walletAddress = walletAddress.substring(2);
  }

  const walletAddressHexArray = [walletAddress];

  // Convert wallet address to hex array
  let stakingContractAddress = data.StakingContractAddress;
  if (stakingContractAddress.startsWith("0x")) {
    stakingContractAddress = stakingContractAddress.substring(2);
  }

  const stakingContractHexArray = [stakingContractAddress];

  // Convert ZRC6 smart contract address to hex array
  let zrc6ContractAddress = data.ZRC6ContractAddress;
  if (zrc6ContractAddress.startsWith("0x")) {
    zrc6ContractAddress = zrc6ContractAddress.substring(2);
  }

  const zrc6AddressHexArray = [zrc6ContractAddress];

  let tokenId;
  if (data.TokenId) {
    tokenId = bytes.intToHexArray(data.TokenId, 16);
  }

  let burnNftHexArray;
  if (data.BurnNft != null || data.BurnNft != undefined) {
    burnNftHexArray = bytes.intToHexArray(data.BurnNft, 4);
  }

  // Concat data to serialize
  let serializeData = paymentAmountHexArray
    .concat(walletAddressHexArray)
    .concat(stakingContractHexArray)
    .concat(zrc6AddressHexArray)
    .concat(tokenId)
    .concat(burnNftHexArray)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

function serializeClaimData(data) {
  console.log("serializeClaimData:", JSON.stringify(data));

  // Convert wallet address to hex array
  let walletAddress = data.OwnerWalletAddress;
  if (walletAddress.startsWith("0x")) {
    walletAddress = walletAddress.substring(2);
  }

  const walletAddressHexArray = [walletAddress];

  // Convert wallet address to hex array
  let stakingContractAddress = data.StakingContractAddress;
  if (stakingContractAddress.startsWith("0x")) {
    stakingContractAddress = stakingContractAddress.substring(2);
  }

  const stakingContractHexArray = [stakingContractAddress];

  // Concat data to serialize
  let serializeData = walletAddressHexArray
    .concat(stakingContractHexArray)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

async function createDataSignDataPairADT(data, sigData) {
  return {
    constructor: "Pair",
    argtypes: ["ByStr", "ByStr64"],
    arguments: [data, sigData],
  };
}

let current_block;
let current_timestamp;
describe("Admin Transitions: Test admin configuration transition in staking contract", () => {
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
    log("zrc2tokenAddress", zrc2tokenAddress);

    const [stakingContract] = await deployStakingContract(
      accounts[0].privateKey,
      accounts[0].address,
      zrc2tokenAddress,
      "262800"
    );
    stakingContractAddress = stakingContract.address;
    log("stakingContractAddress =", stakingContractAddress);
    if (stakingContractAddress === undefined) {
      throw new Error("Failed to deploy token staking contract.");
    }

    const [stakingProxyContract] = await deployStakingProxyContract(
      accounts[0].privateKey,
      metaData.pubKey,
      accounts[0].address
    );
    stakingProxyContractAddress = stakingProxyContract.address;
    log("stakingProxyContractAddress =", stakingProxyContractAddress);
    if (stakingProxyContractAddress === undefined) {
      throw new Error("Failed to deploy token staking Proxy contract.");
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

  test("S1: Add reward token in staking smart contract.", async () => {
    const apr = `${1000}`;
    const treasury_fee = `${100}`;
    const tx = await callContract(
      accounts[0].privateKey,
      stakingContractAddress,
      "AddRewardToken",
      [
        {
          vname: "reward_token_address",
          type: "ByStr20",
          value: zrc2tokenAddress,
        },
        {
          vname: "apr",
          type: "Uint128",
          value: apr,
        },
        {
          vname: "treasury_fee",
          type: "Uint128",
          value: treasury_fee,
        },
      ],
      0,
      false,
      true
    );

    console.log(`AddRewardToken: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);

    const rewardPairs = (await getSmartContractState(stakingContractAddress))
      .reward_pairs;
    const value = rewardPairs[zrc2tokenAddress.toLowerCase()][0].arguments;
    // Check correct apr is set
    expect(apr).toEqual(value[0]);
    // Check correct treasury_fee is set
    expect(treasury_fee).toEqual(value[1]);
    // Check correct whether the block number is zero
    expect(String(0)).toEqual(value[3]);
  });

  test("S2: Remove reward token from staking smart contract.", async () => {
    const apr = `${1000}`;
    const treasury_fee = `${100}`;
    const addRewardToken = await callContract(
      accounts[0].privateKey,
      stakingContractAddress,
      "AddRewardToken",
      [
        {
          vname: "reward_token_address",
          type: "ByStr20",
          value: zrc2tokenAddress,
        },
        {
          vname: "apr",
          type: "Uint128",
          value: `${1000}`,
        },
        {
          vname: "treasury_fee",
          type: "Uint128",
          value: `${100}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `RemoveRewardToken: => addRewardToken: ${JSON.stringify(
        addRewardToken.receipt
      )}`
    );
    expect(addRewardToken.receipt.success).toEqual(true);

    const removeRewardToken = await callContract(
      accounts[0].privateKey,
      stakingContractAddress,
      "RemoveRewardToken",
      [
        {
          vname: "reward_token_address",
          type: "ByStr20",
          value: zrc2tokenAddress,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `RemoveRewardToken: => removeRewardToken: ${JSON.stringify(
        removeRewardToken
      )}`
    );
    expect(removeRewardToken.receipt.success).toEqual(true);

    const rewardPairs = (await getSmartContractState(stakingContractAddress))
      .reward_pairs;
    const value = rewardPairs[zrc2tokenAddress.toLowerCase()][0].arguments;
    // Check correct apr is set
    expect(apr).toEqual(value[0]);
    // Check correct treasury_fee is set
    expect(treasury_fee).toEqual(value[1]);
    // Check whether current block number is update against the reward pair
    expect(value[3]).not.toBe(String(0));
  });
});

describe("User transitions: Test user all transition like add, remove and claim stakes in staking smart contracts.", () => {
  beforeEach(async () => {
    current_block = await getCurrentBlockNumber();
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
    log("zrc2tokenAddress", zrc2tokenAddress);

    const nonFungibleTokenDeployParams = {
      name: "Test T1",
      symbol: "T1",
      tokenUrl: "https://example.com",
    };
    const [zrc6deployedToken] = await deployNonFungibleToken(
      accounts[0].privateKey,
      nonFungibleTokenDeployParams,
      accounts[0].address
    );
    zrc6tokenAddress = zrc6deployedToken.address;
    if (zrc6tokenAddress === undefined) {
      throw new Error("Failed to deploy zrc6 contract.");
    }
    log("zrc6tokenAddress", zrc6tokenAddress);

    const [stakingContract] = await deployStakingContract(
      accounts[0].privateKey,
      accounts[0].address,
      zrc2tokenAddress,
      "262800"
    );
    stakingContractAddress = stakingContract.address;
    log("stakingContractAddress =", stakingContractAddress);
    if (stakingContractAddress === undefined) {
      throw new Error("Failed to deploy token staking contract.");
    }

    const [stakingProxyContract] = await deployStakingProxyContract(
      accounts[0].privateKey,
      metaData.pubKey,
      accounts[0].address
    );
    stakingProxyContractAddress = stakingProxyContract.address;
    log("stakingProxyContractAddress =", stakingProxyContractAddress);
    if (stakingProxyContractAddress === undefined) {
      throw new Error("Failed to deploy token staking Proxy contract.");
    }

    log("Add mod wrap contract as minter in ZRC6 contract");
    await callContract(
      accounts[0].privateKey,
      zrc6tokenAddress,
      "AddMinter",
      [
        {
          vname: "minter",
          type: "ByStr20",
          value: stakingProxyContractAddress,
        },
      ],
      0,
      false,
      false
    );

    const pauseTx = await callContract(
      accounts[0].privateKey,
      stakingContractAddress,
      "UnPause",
      [],
      0,
      false,
      false
    );
    console.log(`Staking: UnPause  => tx: ${JSON.stringify(pauseTx.receipt)}`);
    if (!pauseTx.receipt.success) {
      throw new Error("Failed to unpaused the contract.");
    }

    const addRewardTokenTxns = await callContract(
      accounts[0].privateKey,
      stakingContractAddress,
      "AddRewardToken",
      [
        {
          vname: "reward_token_address",
          type: "ByStr20",
          value: zrc2tokenAddress,
        },
        {
          vname: "apr",
          type: "Uint128",
          value: `${1000}`,
        },
        {
          vname: "treasury_fee",
          type: "Uint128",
          value: `${100}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `AddStake: => tx: ${JSON.stringify(addRewardTokenTxns.receipt)}`
    );
    if (!addRewardTokenTxns.receipt.success) {
      throw new Error("Failed to add reward pairs in staking contract.");
    }

    const tokenTransferToStakeContractTx = await callContract(
      accounts[0].privateKey,
      zrc2tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: stakingContractAddress,
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
    log(
      "Staking: tokenTransferToStakeContractTx",
      JSON.stringify(tokenTransferToStakeContractTx)
    );
    if (!tokenTransferToStakeContractTx.receipt.success) {
      throw new Error("Failed to transfer funds to stake contract.");
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

  test("S3: Add Stake in staking smart contract.", async () => {
    const amountToBeStaked = parseInt(toToken(currency.TOKEN, tokenDecimal));
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    // User giving allowance to staking contract
    const increaseAllowanceTxns = await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: stakingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `AddStake  increaseAllowanceTxns tx: ${JSON.stringify(
        increaseAllowanceTxns.receipt
      )}`
    );
    expect(increaseAllowanceTxns.receipt.success).toEqual(true);

    // Transfer funds to user so that he can stake
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      zrc2tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: accounts[3].address,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    log(
      "Add Stake: Transfer funds to user so that he can stake",
      JSON.stringify(tokenTransferTx)
    );
    expect(tokenTransferTx.receipt.success).toEqual(true);

    // Generate array of serialize data to be signed
    let toDataSignDataPairArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        ExpirationBnum: current_block + 50,
        TokenURI: tokenUrl,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        PenaltyFeeBps: 1000,
        MintNft: 1,
      };

      // Serializing the input data
      const serialize = await serializeAddStakeData(input);

      // Signing the Serialized data
      const signedData = signData(serialize);

      // Generating pair the serialized data and signature
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairArr.push(pair);
    }
    console.log(
      "ToDataSignDataPairArr:",
      JSON.stringify(toDataSignDataPairArr)
    );

    // User trying to stake in staking contract through proxy contract
    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );
    const tx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "AddStake",
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

    console.log(`AddStake: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);

    const fundsTransferLog = tx.receipt.event_logs.filter(
      (row) => row._eventname === "TransferFromSuccess"
    );
    expect(fundsTransferLog).toMatchObject([
      {
        _eventname: "TransferFromSuccess",
        address: zrc2tokenAddress.toLowerCase(),
        params: [
          {
            type: "ByStr20",
            value: stakingContractAddress.toLowerCase(),
            vname: "initiator",
          },
          {
            type: "ByStr20",
            value: accounts[3].address.toLowerCase(),
            vname: "sender",
          },
          {
            type: "ByStr20",
            value: stakingContractAddress.toLowerCase(),
            vname: "recipient",
          },
          {
            type: "Uint128",
            value: `${amountToBeStaked}`,
            vname: "amount",
          },
        ],
      },
    ]);

    const batchMintLog = tx.receipt.event_logs.filter(
      (row) => row._eventname === "BatchMint"
    );
    expect(batchMintLog).toMatchObject([
      {
        _eventname: "BatchMint",
        address: zrc6tokenAddress.toLowerCase(),
        params: [
          {
            type: "List (Pair (ByStr20) (String))",
            value: [
              {
                argtypes: ["ByStr20", "String"],
                arguments: [accounts[3].address.toLowerCase(), tokenUrl],
                constructor: "Pair",
              },
            ],
            vname: "to_token_uri_pair_list",
          },
          {
            type: "Uint256",
            value: "1",
            vname: "start_id",
          },
          {
            type: "Uint256",
            value: "1",
            vname: "end_id",
          },
        ],
      },
    ]);

    // Check if rewards NFT is minted after stake
    const zrc6ContractStateAfterStake = await getSmartContractState(
      zrc6tokenAddress
    );
    const userNftBalanceAfterStake =
      zrc6ContractStateAfterStake.balances[accounts[3].address.toLowerCase()];
    expect(userNftBalanceAfterStake).toEqual("1");

    // Check the amount is staked correctly
    const stakingContractAddressStateAfterStake = await getSmartContractState(
      stakingContractAddress
    );
    const [stakedAmountAfterStake] =
      stakingContractAddressStateAfterStake.stakes &&
      stakingContractAddressStateAfterStake.stakes[
        accounts[3].address.toLowerCase()
      ] &&
      stakingContractAddressStateAfterStake.stakes[
        accounts[3].address.toLowerCase()
      ].arguments
        ? stakingContractAddressStateAfterStake.stakes[
            accounts[3].address.toLowerCase()
          ].arguments
        : 0;

    expect(Number(stakedAmountAfterStake)).toBe(Number(amountToBeStaked));
  });

  test("S4: ReStake in staking smart contract.", async () => {
    const amountToBeStaked = parseInt(toToken(currency.TOKEN, tokenDecimal));
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    // User giving allowance to staking contract
    const increaseAllowanceTxns = await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: stakingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked + amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `AddStake  increaseAllowanceTxns tx: ${JSON.stringify(
        increaseAllowanceTxns.receipt
      )}`
    );
    expect(increaseAllowanceTxns.receipt.success).toEqual(true);

    // Transfer funds to user so that he can stake
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      zrc2tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: accounts[3].address,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked + amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    log(
      "Add Stake: Transfer funds to user so that he can stake",
      JSON.stringify(tokenTransferTx)
    );
    expect(tokenTransferTx.receipt.success).toEqual(true);

    // Generate array of serialize data to be signed for initial stake transaction
    let toDataSignDataPairStakeTransactionArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        ExpirationBnum: current_block + 50,
        TokenURI: tokenUrl,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        PenaltyFeeBps: 1000,
        MintNft: 1,
      };

      // Serializing the input data
      const serialize = await serializeAddStakeData(input);

      // Signing the Serialized data
      const signedData = signData(serialize);

      // Generating pair the serialized data and signature
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairStakeTransactionArr.push(pair);
    }
    console.log(
      "toDataSignDataPairStakeTransactionArr:",
      JSON.stringify(toDataSignDataPairStakeTransactionArr)
    );

    // User trying to stake in staking contract through proxy contract
    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );
    const stakeTxns = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "AddStake",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toDataSignDataPairStakeTransactionArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(`AddStake: => stakeTxns: ${JSON.stringify(stakeTxns.receipt)}`);
    expect(stakeTxns.receipt.success).toEqual(true);

    const fundsTransferLogForStake = stakeTxns.receipt.event_logs.filter(
      (row) => row._eventname === "TransferFromSuccess"
    );
    expect(fundsTransferLogForStake).toMatchObject([
      {
        _eventname: "TransferFromSuccess",
        address: zrc2tokenAddress.toLowerCase(),
        params: [
          {
            type: "ByStr20",
            value: stakingContractAddress.toLowerCase(),
            vname: "initiator",
          },
          {
            type: "ByStr20",
            value: accounts[3].address.toLowerCase(),
            vname: "sender",
          },
          {
            type: "ByStr20",
            value: stakingContractAddress.toLowerCase(),
            vname: "recipient",
          },
          {
            type: "Uint128",
            value: `${amountToBeStaked}`,
            vname: "amount",
          },
        ],
      },
    ]);

    const batchMintLogForStake = stakeTxns.receipt.event_logs.filter(
      (row) => row._eventname === "BatchMint"
    );
    expect(batchMintLogForStake).toMatchObject([
      {
        _eventname: "BatchMint",
        address: zrc6tokenAddress.toLowerCase(),
        params: [
          {
            type: "List (Pair (ByStr20) (String))",
            value: [
              {
                argtypes: ["ByStr20", "String"],
                arguments: [accounts[3].address.toLowerCase(), tokenUrl],
                constructor: "Pair",
              },
            ],
            vname: "to_token_uri_pair_list",
          },
          {
            type: "Uint256",
            value: "1",
            vname: "start_id",
          },
          {
            type: "Uint256",
            value: "1",
            vname: "end_id",
          },
        ],
      },
    ]);

    // Check if rewards NFT is minted after stake
    const zrc6ContractStateAfterStake = await getSmartContractState(
      zrc6tokenAddress
    );
    const userNftBalanceAfterStake =
      zrc6ContractStateAfterStake.balances[accounts[3].address.toLowerCase()];
    expect(userNftBalanceAfterStake).toEqual("1");

    // Check the amount is staked correctly
    const stakingContractAddressStateAfterStake = await getSmartContractState(
      stakingContractAddress
    );
    const [stakedAmountAfterStake] =
      stakingContractAddressStateAfterStake.stakes &&
      stakingContractAddressStateAfterStake.stakes[
        accounts[3].address.toLowerCase()
      ] &&
      stakingContractAddressStateAfterStake.stakes[
        accounts[3].address.toLowerCase()
      ].arguments
        ? stakingContractAddressStateAfterStake.stakes[
            accounts[3].address.toLowerCase()
          ].arguments
        : 0;

    expect(Number(stakedAmountAfterStake)).toBe(Number(amountToBeStaked));

    // Generate array of serialize data to be signed for restake transaction
    let toDataSignDataPairReStakeTransactionArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        ExpirationBnum: current_block + 50,
        TokenURI: tokenUrl,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        PenaltyFeeBps: 1000,
        MintNft: 0,
      };

      // Serializing the input data
      const serialize = await serializeAddStakeData(input);

      // Signing the Serialized data
      const signedData = signData(serialize);

      // Generating pair the serialized data and signature
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairReStakeTransactionArr.push(pair);
    }
    console.log(
      "toDataSignDataPairReStakeTransactionArr:",
      JSON.stringify(toDataSignDataPairReStakeTransactionArr)
    );

    // User trying to stake in staking contract through proxy contract
    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );
    const reStakeTxns = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "AddStake",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toDataSignDataPairReStakeTransactionArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `AddStake: => reStakeTxns: ${JSON.stringify(reStakeTxns.receipt)}`
    );
    expect(reStakeTxns.receipt.success).toEqual(true);

    const fundsTransferLogForReStake = reStakeTxns.receipt.event_logs.filter(
      (row) => row._eventname === "TransferFromSuccess"
    );
    expect(fundsTransferLogForReStake).toMatchObject([
      {
        _eventname: "TransferFromSuccess",
        address: zrc2tokenAddress.toLowerCase(),
        params: [
          {
            type: "ByStr20",
            value: stakingContractAddress.toLowerCase(),
            vname: "initiator",
          },
          {
            type: "ByStr20",
            value: accounts[3].address.toLowerCase(),
            vname: "sender",
          },
          {
            type: "ByStr20",
            value: stakingContractAddress.toLowerCase(),
            vname: "recipient",
          },
          {
            type: "Uint128",
            value: `${amountToBeStaked}`,
            vname: "amount",
          },
        ],
      },
    ]);

    const batchMintLogForReStake = reStakeTxns.receipt.event_logs.filter(
      (row) => row._eventname === "BatchMint"
    );
    expect(batchMintLogForReStake).toMatchObject([]);

    // Check if rewards NFT balance remains the same after restake
    const zrc6ContractStateAfterReStake = await getSmartContractState(
      zrc6tokenAddress
    );
    const userNftBalanceAfterReStake =
      zrc6ContractStateAfterReStake.balances[accounts[3].address.toLowerCase()];
    expect(userNftBalanceAfterReStake).toEqual("1");

    // Check the amount is restaked correctly
    const stakingContractAddressReStateAfterStake = await getSmartContractState(
      stakingContractAddress
    );
    const [stakedAmountAfterReStake] =
      stakingContractAddressReStateAfterStake.stakes &&
      stakingContractAddressReStateAfterStake.stakes[
        accounts[3].address.toLowerCase()
      ] &&
      stakingContractAddressReStateAfterStake.stakes[
        accounts[3].address.toLowerCase()
      ].arguments
        ? stakingContractAddressReStateAfterStake.stakes[
            accounts[3].address.toLowerCase()
          ].arguments
        : 0;
    expect(Number(stakedAmountAfterReStake)).toBe(
      Number(amountToBeStaked + amountToBeStaked)
    );
  });

  test("S5: Remove partial staked amount from staking smart contract.", async () => {
    const amountToBeStaked = parseInt(toToken(currency.TOKEN, tokenDecimal));
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    // Transfer funds to user so that he can stake
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      zrc2tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: accounts[3].address,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    log(
      "Add Stake: Transfer funds to user so that he can stake",
      JSON.stringify(tokenTransferTx)
    );
    expect(tokenTransferTx.receipt.success).toEqual(true);

    const increaseAllowanceTxns = await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: stakingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `RemoveStake: IncreaseAllowance => tx: ${JSON.stringify(
        increaseAllowanceTxns.receipt
      )}`
    );
    expect(increaseAllowanceTxns.receipt.success).toEqual(true);

    let toAddStakeSignDataPairArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        ExpirationBnum: current_block + 10,
        TokenURI: tokenUrl,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        PenaltyFeeBps: 1000,
        MintNft: 1,
      };

      const serialize = await serializeAddStakeData(input);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toAddStakeSignDataPairArr.push(pair);
    }
    console.log(
      "toAddStakeSignDataPairArr",
      JSON.stringify(toAddStakeSignDataPairArr)
    );

    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );

    const addStaketx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "AddStake",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toAddStakeSignDataPairArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `RemoveStake: AddStake => tx: ${JSON.stringify(addStaketx.receipt)}`
    );
    expect(addStaketx.receipt.success).toEqual(true);

    log("Add proxy staking contract as Operator in zrc6 contract");
    const addOperatorTx = await callContract(
      accounts[3].privateKey,
      zrc6tokenAddress,
      "AddOperator",
      [
        {
          vname: "operator",
          type: "ByStr20",
          value: stakingProxyContractAddress,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `RemoveStake: AddOperator => tx: ${JSON.stringify(addOperatorTx.receipt)}`
    );
    expect(addOperatorTx.receipt.success).toEqual(true);

    console.log("Wait to unstake.");
    await sleep(120000);

    const partialAmount = parseInt(toToken(currency.TOKEN - 5, tokenDecimal));
    let toRemoveStakeSignDataPairArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked - partialAmount,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        TokenId: i + 1,
        BurnNft: 0,
      };

      const serialize = await serializeRemoveStakeData(input);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toRemoveStakeSignDataPairArr.push(pair);
    }
    console.log(
      "toRemoveStakeSignDataPairArr",
      JSON.stringify(toRemoveStakeSignDataPairArr)
    );

    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );

    const removeStaketx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "RemoveStake",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toRemoveStakeSignDataPairArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(`RemoveStake: => tx: ${JSON.stringify(removeStaketx.receipt)}`);
    expect(removeStaketx.receipt.success).toEqual(true);

    // Check if user has remaining staked amount
    const stakingContractAddressAfterRemoveState = await getSmartContractState(
      stakingContractAddress
    );
    const [stakedAmountAfterRemoveStakeInContract] =
      stakingContractAddressAfterRemoveState.stakes &&
      stakingContractAddressAfterRemoveState.stakes[
        accounts[3].address.toLowerCase()
      ] &&
      stakingContractAddressAfterRemoveState.stakes[
        accounts[3].address.toLowerCase()
      ].arguments
        ? stakingContractAddressAfterRemoveState.stakes[
            accounts[3].address.toLowerCase()
          ].arguments
        : 0;
    expect(Number(stakedAmountAfterRemoveStakeInContract)).toBe(
      Number(amountToBeStaked - partialAmount)
    );

    // Check if NFT token balance remains the same after partial unstake
    const zrc6ContractStateAfterRemoveStake = await getSmartContractState(
      zrc6tokenAddress
    );
    const userNftBalanceAfterRemoveStake =
      zrc6ContractStateAfterRemoveStake.balances[
        accounts[3].address.toLowerCase()
      ];
    expect(userNftBalanceAfterRemoveStake).toEqual("1");

    // Check if appropriate rewards are calculated after partial unstake
    const rewardAmountAfterRemoveStake =
      stakingContractAddressAfterRemoveState.rewards &&
      stakingContractAddressAfterRemoveState.rewards[
        zrc2tokenAddress.toLowerCase()
      ] &&
      stakingContractAddressAfterRemoveState.rewards[
        zrc2tokenAddress.toLowerCase()
      ][accounts[3].address.toLowerCase()]
        ? stakingContractAddressAfterRemoveState.rewards[
            zrc2tokenAddress.toLowerCase()
          ][accounts[3].address.toLowerCase()]
        : 0;
    console.log("rewardAmountAfterRemoveStake", rewardAmountAfterRemoveStake);
    expect(Number(rewardAmountAfterRemoveStake)).toBeGreaterThan(0);
  });

  test("S5: Remove all staked from staking smart contract.", async () => {
    const amountToBeStaked = parseInt(toToken(currency.TOKEN, tokenDecimal));
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    // Transfer funds to user so that he can stake
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      zrc2tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: accounts[3].address,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    log(
      "Add Stake: Transfer funds to user so that he can stake",
      JSON.stringify(tokenTransferTx)
    );
    expect(tokenTransferTx.receipt.success).toEqual(true);

    const increaseAllowanceTxns = await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: stakingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `RemoveStake: IncreaseAllowance => tx: ${JSON.stringify(
        increaseAllowanceTxns.receipt
      )}`
    );
    expect(increaseAllowanceTxns.receipt.success).toEqual(true);

    let toAddStakeSignDataPairArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        ExpirationBnum: current_block + 10,
        TokenURI: tokenUrl,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        PenaltyFeeBps: 1000,
        MintNft: 1,
      };

      const serialize = await serializeAddStakeData(input);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toAddStakeSignDataPairArr.push(pair);
    }
    console.log(
      "toAddStakeSignDataPairArr",
      JSON.stringify(toAddStakeSignDataPairArr)
    );

    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );

    const addStaketx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "AddStake",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toAddStakeSignDataPairArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `RemoveStake: AddStake => tx: ${JSON.stringify(addStaketx.receipt)}`
    );
    expect(addStaketx.receipt.success).toEqual(true);

    log("Add proxy staking contract as Operator in zrc6 contract");
    const addOperatorTx = await callContract(
      accounts[3].privateKey,
      zrc6tokenAddress,
      "AddOperator",
      [
        {
          vname: "operator",
          type: "ByStr20",
          value: stakingProxyContractAddress,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `RemoveStake: AddOperator => tx: ${JSON.stringify(addOperatorTx.receipt)}`
    );
    expect(addOperatorTx.receipt.success).toEqual(true);

    console.log("Wait to unstake.");
    await sleep(120000);

    let toRemoveStakeSignDataPairArr = [];
    for (let i = 0; i < 1; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        TokenId: i + 1,
        BurnNft: 1,
      };

      const serialize = await serializeRemoveStakeData(input);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toRemoveStakeSignDataPairArr.push(pair);
    }
    console.log(
      "toRemoveStakeSignDataPairArr",
      JSON.stringify(toRemoveStakeSignDataPairArr)
    );

    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );

    const removeStaketx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "RemoveStake",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toRemoveStakeSignDataPairArr,
        },
      ],
      0,
      false,
      true
    );

    console.log(`RemoveStake: => tx: ${JSON.stringify(removeStaketx.receipt)}`);
    expect(removeStaketx.receipt.success).toEqual(true);

    // Check if staked object is empty for the user
    const stakingContractAddressState = await getSmartContractState(
      stakingContractAddress
    );
    const stakedAmountAfterUnstakeInContract =
      stakingContractAddressState.stakes &&
      stakingContractAddressState.stakes[accounts[3].address.toLowerCase()] &&
      stakingContractAddressState.stakes[accounts[3].address.toLowerCase()]
        .arguments
        ? stakingContractAddressState.stakes[accounts[3].address.toLowerCase()]
            .arguments
        : {};
    expect(stakedAmountAfterUnstakeInContract).toMatchObject({});

    // Check if NFT token is burned
    const zrc6ContractState = await getSmartContractState(zrc6tokenAddress);
    const userNftBalanceAfterUnStake =
      zrc6ContractState.balances[accounts[3].address.toLowerCase()];
    expect(userNftBalanceAfterUnStake).toEqual("0");

    const rewardAmountAfterUnStakeInContract =
      stakingContractAddressState.rewards &&
      stakingContractAddressState.rewards[zrc2tokenAddress.toLowerCase()] &&
      stakingContractAddressState.rewards[zrc2tokenAddress.toLowerCase()][
        accounts[3].address.toLowerCase()
      ]
        ? stakingContractAddressState.rewards[zrc2tokenAddress.toLowerCase()][
            accounts[3].address.toLowerCase()
          ]
        : 0;
    console.log(
      "rewardAmountAfterUnStakeInContract",
      rewardAmountAfterUnStakeInContract
    );
    expect(Number(rewardAmountAfterUnStakeInContract)).toBeGreaterThan(0);
  });

  test("S6: Remove all staked from staking smart contract before the expiry time, penalty should be charged.", async () => {});
  test("S7: Claim rewards from staking smart contract.", async () => {
    const amountToBeStaked = parseInt(toToken(currency.TOKEN, tokenDecimal));
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    // Transfer funds to user so that he can stake
    const tokenTransferTx = await callContract(
      accounts[0].privateKey,
      zrc2tokenAddress,
      "Transfer",
      [
        {
          vname: "to",
          type: "ByStr20",
          value: accounts[3].address,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    log(
      "Add Stake: Transfer funds to user so that he can stake",
      JSON.stringify(tokenTransferTx)
    );
    expect(tokenTransferTx.receipt.success).toEqual(true);

    const increaseAllowanceTxns = await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: stakingContractAddress,
        },
        {
          vname: "amount",
          type: "Uint128",
          value: `${amountToBeStaked}`,
        },
      ],
      0,
      false,
      false
    );
    console.log(
      `IncreaseAllowance  increaseAllowanceTxns tx: ${JSON.stringify(
        increaseAllowanceTxns.receipt
      )}`
    );
    expect(increaseAllowanceTxns.receipt.success).toEqual(true);

    let toDataSignDataPairArr = [];
    for (let i = 0; i < 2; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        Amount: amountToBeStaked,
        ExpirationBnum: current_block + 50,
        TokenURI: tokenUrl,
        StakingContractAddress: stakingContractAddress,
        ZRC6ContractAddress: zrc6tokenAddress,
        PenaltyFeeBps: 1000,
        MintNft: 1,
      };

      const serialize = await serializeAddStakeData(input);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairArr.push(pair);
    }
    console.log("toDataSignDataPairArr", JSON.stringify(toDataSignDataPairArr));

    console.log(
      `User with wallet address: ${accounts[3].address} is trying to stake.`
    );

    const tx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "AddStake",
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

    console.log(`AddStake: => tx: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);

    // console.log("Wait to claim.");
    // await sleep(120000);

    const inputClaimData = {
      OwnerWalletAddress: accounts[3].address,
      StakingContractAddress: stakingContractAddress,
    };

    const serialize = await serializeClaimData(inputClaimData);

    const signedData = signData(serialize);

    const claimStaketx = await callContract(
      accounts[3].privateKey,
      stakingProxyContractAddress,
      "ClaimRewards",
      [
        {
          vname: "data",
          type: "ByStr",
          value: serialize,
        },
        {
          vname: "sigData",
          type: "ByStr64",
          value: `${signedData}`,
        },
      ],
      0,
      false,
      true
    );

    console.log(`ClaimRewards: => tx: ${JSON.stringify(claimStaketx.receipt)}`);
    expect(claimStaketx.receipt.success).toEqual(true);
  });
});
