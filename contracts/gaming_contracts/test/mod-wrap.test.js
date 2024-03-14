const {
  deployNonFungibleToken,
} = require("../scripts/deployNonFungibleToken.js");
const {
  deployModWrapContract,
} = require("../scripts/deployModWrapContract.js");
const { deployFungibleToken } = require("../scripts/deployFungibleToken.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const {
  serializeMintingData,
  serializePurchaseData,
  signData,
  toZil,
  toToken,
  getAccounts,
  getMetaData,
  fromZil,
} = require("../scripts/utils/helper");
const {
  callContract,
  getCurrentBlockNumber,
  getBalance,
  sendZil,
} = require("../scripts/utils/call.js");
const { BN } = require("@zilliqa-js/util");
const log = console.log;

const accounts = getAccounts();
const metaData = getMetaData();

const currency = {
  ZIL: 100,
  TOKEN: 10,
  COMMISSION: 3,
};

async function createPairADT(address, string) {
  return {
    constructor: "Pair",
    argtypes: ["ByStr20", "String"],
    arguments: [address, string],
  };
}

async function createDataSignDataPairADT(data, sigData) {
  return {
    constructor: "Pair",
    argtypes: ["ByStr", "ByStr64"],
    arguments: [data, sigData],
  };
}

describe("Mod wrap => Test the claim mint flow.", () => {
  beforeEach(async () => {
    // Contract Deployments
    const nonFungibleTokenDeployParams = {
      name: "Test T1",
      symbol: "T1",
      tokenUrl: "https://example.com",
    };
    const [deployedToken] = await deployNonFungibleToken(
      accounts[0].privateKey,
      nonFungibleTokenDeployParams,
      accounts[0].address
    );
    tokenAddress = deployedToken.address;

    if (tokenAddress === undefined) {
      throw new Error("Failed to deploy zrc6 contract.");
    }

    log("tokenAddress", tokenAddress);

    const [modWrapContract] = await deployModWrapContract(
      accounts[0].privateKey,
      metaData.pubKey,
      accounts[0].address,
      accounts[1].address
    );
    modWrapContractAddress = modWrapContract.address;
    log("modWrapContractAddress =", modWrapContractAddress);
    if (modWrapContractAddress === undefined) {
      throw new Error("Failed to deploy mod wrap contract.");
    }

    log("Add mod wrap contract as minter in ZRC6 contract");
    await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "AddMinter",
      [
        {
          vname: "minter",
          type: "ByStr20",
          value: modWrapContractAddress,
        },
      ],
      0,
      false,
      false
    );

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

  test("Mod wrap: Test the transfer ownership feature", async () => {
    const setOwnertxns = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
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

    log("ClaimMint: setOwnertxns", JSON.stringify(setOwnertxns));
    expect(setOwnertxns.receipt.success).toEqual(true);

    const claimOwnershiptxns = await callContract(
      accounts[1].privateKey,
      modWrapContractAddress,
      "AcceptContractOwnership",
      [],
      0,
      false,
      false
    );

    log("ClaimMint: claimOwnershiptxns", JSON.stringify(claimOwnershiptxns));
    expect(claimOwnershiptxns.receipt.success).toEqual(true);
  });

  test("Mod wrap: Test the transfer ownership feature by incorrect users, should fail.", async () => {
    const setOwnertxns = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
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

    log("ClaimMint: setOwnertxns", JSON.stringify(setOwnertxns));
    expect(setOwnertxns.receipt.success).toEqual(true);

    const claimOwnershiptxns = await callContract(
      accounts[3].privateKey,
      modWrapContractAddress,
      "AcceptContractOwnership",
      [],
      0,
      false,
      false
    );

    log("ClaimMint: claimOwnershiptxns", JSON.stringify(claimOwnershiptxns));
    expect(claimOwnershiptxns.receipt.success).toEqual(false);
  });

  test("Mod wrap: Test using the same signature. Should fail.", async () => {
    const input = {
      OwnerWalletAddress: accounts[0].address,
      TokenURI:
        "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/",
      ZRC6ContractAddress: tokenAddress,
      Mode: "NFT",
    };

    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    const serialize = await serializeMintingData(input, currentBlockNum);

    const signedData = signData(serialize);

    const callMintFirstTx = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
      "ClaimMint",
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
      false
    );

    log("ClaimMint: callMintFirstTx", JSON.stringify(callMintFirstTx));
    expect(callMintFirstTx.receipt.success).toEqual(true);

    const callMintSecondTx = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
      "ClaimMint",
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
      false
    );
    log("ClaimMint: callMintSecondTx", JSON.stringify(callMintSecondTx));
    expect(callMintSecondTx.receipt.success).toEqual(false);
  });

  test("Mod wrap: Test single claim mint.", async () => {
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    const input = {
      OwnerWalletAddress: accounts[0].address,
      TokenURI: tokenUrl,
      ZRC6ContractAddress: tokenAddress,
      Mode: "NFT",
    };

    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    const serialize = await serializeMintingData(input, currentBlockNum);

    const signedData = signData(serialize);

    const callMintTx = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
      "ClaimMint",
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
      false
    );
    log("ClaimMint: callMintTx", JSON.stringify(callMintTx));
    expect(callMintTx.receipt.success).toEqual(true);

    const event_logs = callMintTx.receipt.event_logs.filter(
      (row) => row._eventname == "MintSuccess"
    );

    console.log("event_logs", JSON.stringify(event_logs));

    expect(event_logs).toEqual([
      {
        _eventname: "MintSuccess",
        address: modWrapContractAddress.toLowerCase(),
        params: [
          {
            type: "String",
            value: "Mint Successful",
            vname: "status",
          },
          {
            type: "String",
            value: tokenUrl,
            vname: "token_uri",
          },
          {
            type: "ByStr20",
            value: tokenAddress.toLowerCase(),
            vname: "nft_smart_contract",
          },
          {
            type: "Uint128",
            value: String(currentBlockNum),
            vname: "block_number_uint128",
          },
          {
            type: "ByStr20",
            value: accounts[0].address.toLowerCase(),
            vname: "token_owner",
          },
        ],
      },
    ]);
  });
  test("Mod wrap: Test batch claim mint.", async () => {
    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    let toDataSignDataPairArr = [];
    let toTokenPairArr = [];
    for (let i = 0; i < 50; i++) {
      const tokenUrl =
        "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
      const input = {
        OwnerWalletAddress: accounts[0].address,
        TokenURI: tokenUrl,
        ZRC6ContractAddress: tokenAddress,
        Mode: "NFT",
      };

      const serialize = await serializeMintingData(input, currentBlockNum);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairArr.push(pair);

      const pair1 = await createPairADT(
        accounts[0].address.toLowerCase(),
        "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/"
      );

      toTokenPairArr.push(pair1);
    }

    console.log("toDataSignDataPairArr", JSON.stringify(toDataSignDataPairArr));

    const tx = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
      "BatchClaimMint",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toDataSignDataPairArr,
        },
      ],
      0,
      false,
      false
    );

    console.log("tx.receipt", JSON.stringify(tx.receipt));
    expect(tx.receipt.success).toEqual(true);

    const event_logs = tx.receipt.event_logs.filter(
      (row) => row._eventname == "BatchClaimMintSuccess"
    );

    console.log("event_logs", JSON.stringify(event_logs));

    expect(event_logs).toEqual([
      {
        _eventname: "BatchClaimMintSuccess",
        address: modWrapContractAddress.toLowerCase(),
        params: [
          {
            type: "String",
            value: "Batch Mint Successful",
            vname: "status",
          },
          {
            type: "List (Pair (ByStr20) (String))",
            value: toTokenPairArr,
            vname: "to_token_uri_pair_list",
          },
          {
            type: "ByStr20",
            value: tokenAddress.toLowerCase(),
            vname: "nft_smart_contract",
          },
          {
            type: "Uint128",
            value: String(currentBlockNum),
            vname: "block_number_uint128",
          },
          {
            type: "ByStr20",
            value: accounts[0].address.toLowerCase(),
            vname: "token_owner",
          },
        ],
      },
    ]);
  });
  test("Mod wrap: Test purchase flow with ZILs.", async () => {
    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    const zil_address = "0x0000000000000000000000000000000000000000";
    const payment_amount = parseInt(toZil(currency.ZIL));
    const input = {
      OwnerWalletAddress: accounts[3].address,
      TokenURI: tokenUrl,
      ZRC6ContractAddress: tokenAddress,
      Mode: "NFT",
      PaymentToken: zil_address,
      PaymentAmount: payment_amount,
    };

    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    const serialize = await serializePurchaseData(input, currentBlockNum);

    const signedData = signData(serialize);

    const callMintTx = await callContract(
      accounts[3].privateKey,
      modWrapContractAddress,
      "Purchase",
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
      toZil(currency.ZIL),
      false,
      false
    );
    log("ClaimMint: callMintTx", JSON.stringify(callMintTx));
    expect(callMintTx.receipt.success).toEqual(true);

    const event_logs = callMintTx.receipt.event_logs.filter(
      (row) => row._eventname == "PurchaseSuccess"
    );

    console.log("event_logs", JSON.stringify(event_logs));

    expect(event_logs).toEqual([
      {
        _eventname: "PurchaseSuccess",
        address: modWrapContractAddress.toLowerCase(),
        params: [
          {
            type: "String",
            value: "Purchase Successful",
            vname: "status",
          },
          {
            type: "String",
            value: tokenUrl,
            vname: "token_uri",
          },
          {
            type: "ByStr20",
            value: tokenAddress.toLowerCase(),
            vname: "nft_smart_contract",
          },
          {
            type: "Uint128",
            value: String(currentBlockNum),
            vname: "block_number_uint128",
          },
          {
            type: "ByStr20",
            value: accounts[3].address.toLowerCase(),
            vname: "token_owner",
          },
          {
            type: "ByStr20",
            value: accounts[1].address.toLowerCase(),
            vname: "revenue_recipient",
          },
          {
            type: "ByStr20",
            value: zil_address,
            vname: "payment_token",
          },
          {
            type: "Uint128",
            value: String(payment_amount),
            vname: "payment_amount",
          },
        ],
      },
    ]);
  });

  test("Mod wrap: Test purchase flow with ZRC2 token.", async () => {
    const fungibleTokenDeployParams = {
      name: "Test T2",
      symbol: null,
      decimals: 6,
      supply: new BN("10000000000000000"),
    };
    const [deployedToken] = await deployFungibleToken(
      accounts[0].privateKey,
      fungibleTokenDeployParams,
      accounts[0].address
    );
    const zrc2tokenAddress = deployedToken.address;
    const tokenDecimal = fungibleTokenDeployParams.decimals;
    if (zrc2tokenAddress === undefined) {
      throw new Error("Failed to deploy zrc2 contract.");
    }

    log("zrc2tokenAddress", zrc2tokenAddress);

    log("Transfer token to user address");
    await callContract(
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
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      false
    );

    log("Give allowance to the token swap contract", accounts[3].address);
    await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: modWrapContractAddress,
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

    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    const payment_amount = toToken(currency.TOKEN, tokenDecimal);
    const input = {
      OwnerWalletAddress: accounts[3].address,
      TokenURI: tokenUrl,
      ZRC6ContractAddress: tokenAddress,
      Mode: "NFT",
      PaymentToken: zrc2tokenAddress,
      PaymentAmount: parseInt(payment_amount),
    };

    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    const serialize = await serializePurchaseData(input, currentBlockNum);

    const signedData = signData(serialize);

    const callMintTx = await callContract(
      accounts[3].privateKey,
      modWrapContractAddress,
      "Purchase",
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
      false
    );
    log("ClaimMint: callMintTx", JSON.stringify(callMintTx));
    expect(callMintTx.receipt.success).toEqual(true);

    const event_logs = callMintTx.receipt.event_logs.filter(
      (row) => row._eventname == "PurchaseSuccess"
    );

    console.log("event_logs", JSON.stringify(event_logs));

    expect(event_logs).toEqual([
      {
        _eventname: "PurchaseSuccess",
        address: modWrapContractAddress.toLowerCase(),
        params: [
          {
            type: "String",
            value: "Purchase Successful",
            vname: "status",
          },
          {
            type: "String",
            value: tokenUrl,
            vname: "token_uri",
          },
          {
            type: "ByStr20",
            value: tokenAddress.toLowerCase(),
            vname: "nft_smart_contract",
          },
          {
            type: "Uint128",
            value: String(currentBlockNum),
            vname: "block_number_uint128",
          },
          {
            type: "ByStr20",
            value: accounts[3].address.toLowerCase(),
            vname: "token_owner",
          },
          {
            type: "ByStr20",
            value: accounts[1].address.toLowerCase(),
            vname: "revenue_recipient",
          },
          {
            type: "ByStr20",
            value: zrc2tokenAddress.toLowerCase(),
            vname: "payment_token",
          },
          {
            type: "Uint128",
            value: String(payment_amount),
            vname: "payment_amount",
          },
        ],
      },
    ]);
  });

  test("Mod wrap: Test batch purchase flow with ZILs.", async () => {
    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    const zil_address = "0x0000000000000000000000000000000000000000";
    const payment_amount = parseInt(toZil(currency.ZIL));
    let toDataSignDataPairArr = [];
    let toTokenPairArr = [];
    for (let i = 0; i < 5; i++) {
      const input = {
        OwnerWalletAddress: accounts[0].address,
        TokenURI: tokenUrl,
        ZRC6ContractAddress: tokenAddress,
        Mode: "NFT",
        PaymentToken: zil_address,
        PaymentAmount: payment_amount,
      };
      console.log("input", input);
      const serialize = await serializePurchaseData(input, currentBlockNum);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairArr.push(pair);

      const pair1 = await createPairADT(
        accounts[0].address.toLowerCase(),
        tokenUrl
      );

      toTokenPairArr.push(pair1);
    }

    console.log("toDataSignDataPairArr", JSON.stringify(toDataSignDataPairArr));

    const tx = await callContract(
      accounts[0].privateKey,
      modWrapContractAddress,
      "BatchPurchase",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toDataSignDataPairArr,
        },
      ],
      payment_amount,
      false,
      false
    );

    console.log("tx.receipt", JSON.stringify(tx.receipt));
    expect(tx.receipt.success).toEqual(true);

    const event_logs = tx.receipt.event_logs.filter(
      (row) => row._eventname == "BatchPurchaseMintSuccess"
    );

    console.log("event_logs", JSON.stringify(event_logs));

    expect(event_logs).toEqual([
      {
        _eventname: "BatchPurchaseMintSuccess",
        address: modWrapContractAddress.toLowerCase(),
        params: [
          {
            type: "String",
            value: "Batch Purchase Successful",
            vname: "status",
          },
          {
            type: "List (Pair (ByStr20) (String))",
            value: toTokenPairArr,
            vname: "to_token_uri_pair_list",
          },
          {
            type: "ByStr20",
            value: tokenAddress.toLowerCase(),
            vname: "nft_smart_contract",
          },
          {
            type: "Uint128",
            value: String(currentBlockNum),
            vname: "block_number_uint128",
          },
          {
            type: "ByStr20",
            value: accounts[0].address.toLowerCase(),
            vname: "token_owner",
          },
          {
            type: "ByStr20",
            value: accounts[1].address.toLowerCase(),
            vname: "revenue_recipient",
          },
          {
            type: "ByStr20",
            value: zil_address,
            vname: "payment_token",
          },
          {
            type: "Uint128",
            value: String(payment_amount),
            vname: "payment_amount",
          },
        ],
      },
    ]);
  });

  test("Mod wrap: Test batch purchase flow with ZRC2.", async () => {
    const fungibleTokenDeployParams = {
      name: "Test T2",
      symbol: null,
      decimals: 6,
      supply: new BN("10000000000000000"),
    };
    const [deployedToken] = await deployFungibleToken(
      accounts[0].privateKey,
      fungibleTokenDeployParams,
      accounts[0].address
    );
    const zrc2tokenAddress = deployedToken.address;
    const tokenDecimal = fungibleTokenDeployParams.decimals;
    if (zrc2tokenAddress === undefined) {
      throw new Error("Failed to deploy zrc2 contract.");
    }

    log("zrc2tokenAddress", zrc2tokenAddress);

    log("Transfer token to user address");
    await callContract(
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
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      false
    );

    log("Give allowance to the token swap contract", accounts[3].address);
    await callContract(
      accounts[3].privateKey,
      zrc2tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: modWrapContractAddress,
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

    const currentBlockNumRes = await getCurrentBlockNumber();
    const currentBlockNum = currentBlockNumRes + 10;
    console.log("currentBlockNum", currentBlockNum);

    const tokenUrl =
      "https://bafkreicj6xawtyzexxmvfbth2div73tlv33ojh3uc2masw7ealn5nlavny.ipfs.nftstorage.link/";
    const payment_amount = parseInt(toToken(currency.TOKEN, tokenDecimal));
    let toDataSignDataPairArr = [];
    let toTokenPairArr = [];
    for (let i = 0; i < 5; i++) {
      const input = {
        OwnerWalletAddress: accounts[3].address,
        TokenURI: tokenUrl,
        ZRC6ContractAddress: tokenAddress,
        Mode: "NFT",
        PaymentToken: zrc2tokenAddress,
        PaymentAmount: payment_amount,
      };
      console.log("input", input);
      const serialize = await serializePurchaseData(input, currentBlockNum);

      const signedData = signData(serialize);

      // Batch-mint some NFTs
      const pair = await createDataSignDataPairADT(serialize, signedData);
      toDataSignDataPairArr.push(pair);

      const pair1 = await createPairADT(
        accounts[3].address.toLowerCase(),
        tokenUrl
      );

      toTokenPairArr.push(pair1);
    }

    console.log("toDataSignDataPairArr", JSON.stringify(toDataSignDataPairArr));

    const tx = await callContract(
      accounts[3].privateKey,
      modWrapContractAddress,
      "BatchPurchase",
      [
        {
          vname: "data_sig_pair_list",
          type: "List (Pair ByStr ByStr64)",
          value: toDataSignDataPairArr,
        },
      ],
      0,
      false,
      false
    );

    console.log("tx.receipt", JSON.stringify(tx.receipt));
    expect(tx.receipt.success).toEqual(true);

    const event_logs = tx.receipt.event_logs.filter(
      (row) => row._eventname == "BatchPurchaseSuccess"
    );

    console.log("event_logs", JSON.stringify(event_logs));

    expect(event_logs).toEqual([
      {
        _eventname: "BatchPurchaseSuccess",
        address: modWrapContractAddress.toLowerCase(),
        params: [
          {
            type: "String",
            value: "Batch Purchase Successful",
            vname: "status",
          },
          {
            type: "List (Pair (ByStr20) (String))",
            value: toTokenPairArr,
            vname: "to_token_uri_pair_list",
          },
          {
            type: "ByStr20",
            value: tokenAddress.toLowerCase(),
            vname: "nft_smart_contract",
          },
          {
            type: "Uint128",
            value: String(currentBlockNum),
            vname: "block_number_uint128",
          },
          {
            type: "ByStr20",
            value: accounts[3].address.toLowerCase(),
            vname: "token_owner",
          },
          {
            type: "ByStr20",
            value: accounts[1].address.toLowerCase(),
            vname: "revenue_recipient",
          },
          {
            type: "ByStr20",
            value: zrc2tokenAddress.toLowerCase(),
            vname: "payment_token",
          },
          {
            type: "Uint128",
            value: String(payment_amount),
            vname: "payment_amount",
          },
        ],
      },
    ]);
  });
});
