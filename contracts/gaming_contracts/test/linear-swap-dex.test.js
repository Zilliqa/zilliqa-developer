const { deployFungibleToken } = require("../scripts/deployFungibleToken.js");
const {
  deployLinearSwapDexContract,
} = require("../scripts/deployLinearSwapDexContract.js");
const {
  fromZil,
  toToken,
  toZil,
  serializeLinearSwapData,
  signData,
  getAccounts,
  getMetaData,
} = require("../scripts/utils/helper");
const { BN, units } = require("@zilliqa-js/util");
const {
  callContract,
  getCurrentBlockNumber,
  getBalance,
} = require("../scripts/utils/call.js");
const log = console.log;

const accounts = getAccounts();
const metaData = getMetaData();

const currency = {
  ZIL: 10,
  TOKEN: 10,
  COMMISSION: 3,
};

let tokenAddress;
let tokenDecimal;
let dexAddress;

describe("Token Swap => Add Zil", () => {
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

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }

    log("Give allowance to the token swap contract");
    await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: dexAddress,
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

  test("AddZil: Before admin assignment should throws NotContractOwnerOrAdminError and code 4", async () => {
    const tx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(
      `AddZil: Before admin assignment => tx: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(false);
    expect(tx.receipt.exceptions).toEqual([
      {
        line: 1,
        message:
          'Exception thrown: (Message [(_exception : (String "Error")) ; (code : (Int32 -4))])',
      },
      { line: 1, message: "Raised from AddZil" },
    ]);
  });

  test("AddZil: After assignment of an admin should be successful.", async () => {
    const assignAdminTx = await callContract(
      accounts[0].privateKey,
      dexAddress,
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

    expect(assignAdminTx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(
      `AddZil: after the admin is assigned tx ====>: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "AddZilSuccess",
        address: `${dexAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${accounts[1].address.toLowerCase()}`,
            vname: "sender_address",
          },
          {
            type: "Uint128",
            value: toZil(currency.ZIL),
            vname: "balance",
          },
        ],
      },
    ]);
  });

  test("AddZil:EnsureAmountIsNotZero => When trying to add zero zils should throw InvalidAmountPassed an code 7.", async () => {
    const tx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddZil",
      [],
      0,
      false,
      true
    );

    console.log(
      `AddZil: EnsureAmountIsNotZero tx===> : ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(false);
    expect(tx.receipt.exceptions).toEqual([
      {
        line: 1,
        message:
          'Exception thrown: (Message [(_exception : (String "Error")) ; (code : (Int32 -7))])',
      },
      {
        line: 1,
        message: "Raised from RequireContractOwnerOrAdmin",
      },
      {
        line: 1,
        message: "Raised from AddZil",
      },
    ]);
  });

  test("AddZil: When the transaction is successful.", async () => {
    const tx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(`AddZil: Success tx =======>: : ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "AddZilSuccess",
        address: `${dexAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${accounts[0].address.toLowerCase()}`,
            vname: "sender_address",
          },
          {
            type: "Uint128",
            value: toZil(currency.ZIL),
            vname: "balance",
          },
        ],
      },
    ]);
  });
});

describe("Token Swap => Withdraw Zil", () => {
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

    log("tokenAddress", tokenAddress);

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }

    log("Give allowance to the token swap contract");
    await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: dexAddress,
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
  });

  test("WithdrawZil: Before admin assignment should throws NotContractOwnerOrAdminError and code 4", async () => {
    const AddZiltx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(`AddZiltx: ${JSON.stringify(AddZiltx.receipt)}`);
    expect(AddZiltx.receipt.success).toEqual(false);

    const tx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "WithdrawZil",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toZil(currency.ZIL),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawZil: Before admin assignment tx =====>: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("WithdrawZil: After assignment of an admin should be successful.", async () => {
    const assignAdminTx = await callContract(
      accounts[0].privateKey,
      dexAddress,
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

    expect(assignAdminTx.receipt.success).toEqual(true);

    const AddZiltx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(`AddZiltx: ${JSON.stringify(AddZiltx.receipt)}`);
    expect(AddZiltx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "WithdrawZil",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toZil(currency.ZIL),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawZil: After admin assignment tx =====>: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "WithdrawZilSuccess",
        address: `${dexAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${accounts[1].address.toLowerCase()}`,
            vname: "recipient_address",
          },
          {
            type: "Uint128",
            value: "0",
            vname: "balance",
          },
        ],
      },
    ]);
  });

  test("WithdrawZil: Success", async () => {
    const AddZiltx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(`AddZiltx: ${JSON.stringify(AddZiltx.receipt)}`);
    expect(AddZiltx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "WithdrawZil",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toZil(currency.ZIL),
        },
      ],
      0,
      false,
      true
    );

    console.log(`WithdrawZil: Success tx ====>: ${JSON.stringify(tx.receipt)}`);
    expect(tx.receipt.success).toEqual(true);
    expect(tx.receipt.event_logs).toEqual([
      {
        _eventname: "WithdrawZilSuccess",
        address: `${dexAddress.toLowerCase()}`,
        params: [
          {
            type: "ByStr20",
            value: `${accounts[0].address.toLowerCase()}`,
            vname: "recipient_address",
          },
          {
            type: "Uint128",
            value: "0",
            vname: "balance",
          },
        ],
      },
    ]);
  });
});

describe("Token Swap => Add Token", () => {
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

    log("tokenAddress", tokenAddress);

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }

    await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: dexAddress,
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
  });

  test("AddToken: When the transaction is successful.", async () => {
    const tx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `AddToken: Success tx =======>: : ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(true);
  });
});

describe("Token Swap => Withdraw Token", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T4",
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

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }

    log("Give allowance to the token swap contract");
    await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: dexAddress,
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
  });

  test("WithdrawToken: Before admin assignment should throws NotContractOwnerOrAdminError and code 4", async () => {
    const AddTokentx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "AddToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawToken: AddTokentx: ${JSON.stringify(AddTokentx.receipt)}`
    );
    expect(AddTokentx.receipt.success).toEqual(false);

    const tx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "WithdrawToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawToken: Before admin assignment tx ====>: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("WithdrawToken: After assignment of an admin should be successful.", async () => {
    const assignAdminTx = await callContract(
      accounts[0].privateKey,
      dexAddress,
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
    console.log(
      `WithdrawToken: assignAdminTx: ${JSON.stringify(assignAdminTx.receipt)}`
    );
    expect(assignAdminTx.receipt.success).toEqual(true);

    const AddTokentx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawToken: AddTokentx: ${JSON.stringify(AddTokentx.receipt)}`
    );
    expect(AddTokentx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[1].privateKey,
      dexAddress,
      "WithdrawToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawToken: After assignment of an admin tx ====>: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(true);
  });

  test("WithdrawToken: Success", async () => {
    const AddTokentx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawToken: AddTokentx: ${JSON.stringify(AddTokentx.receipt)}`
    );
    expect(AddTokentx.receipt.success).toEqual(true);

    const tx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "WithdrawToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN),
        },
      ],
      0,
      false,
      true
    );

    console.log(
      `WithdrawToken: Success tx ====>: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(true);
  });
});

describe("Token Swap => Accept Contract Ownership Transfer", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T5",
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

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }
  });

  test("AcceptContractOwnershipTransfer: When non ownership recipient calls to accept the ownership", async () => {
    const AddContractOwnershipRecipienttx = await callContract(
      accounts[0].privateKey,
      dexAddress,
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
      accounts[3].privateKey,
      dexAddress,
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
    expect(tx.receipt.success).toEqual(false);
    expect(tx.receipt.exceptions).toEqual([
      {
        line: 1,
        message:
          'Exception thrown: (Message [(_exception : (String "Error")) ; (code : (Int32 -14))])',
      },
      {
        line: 1,
        message: "Raised from AcceptContractOwnershipTransfer",
      },
    ]);
  });

  test("AcceptContractOwnershipTransfer: Success", async () => {
    const AddContractOwnershipRecipienttx = await callContract(
      accounts[0].privateKey,
      dexAddress,
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
      dexAddress,
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
        address: `${dexAddress.toLowerCase()}`,
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
});

describe("Token Swap => Swap Zil For Token", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T6",
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

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }

    log("Give allowance to the token swap contract");
    await callContract(
      accounts[0].privateKey,
      tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: dexAddress,
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

    log("Add Token Liquidity");
    await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddToken",
      [
        {
          vname: "amount",
          type: "Uint128",
          value: toToken(currency.TOKEN, tokenDecimal),
        },
      ],
      0,
      false,
      true
    );

    log("Add Commission Recipient Address");
    await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddCommissionRecipientAddress",
      [
        {
          vname: "address",
          type: "ByStr20",
          value: accounts[2].address,
        },
      ],
      0,
      false,
      true
    );
  });

  test("SwapZilForToken: When the contract is paused should throws RequireNotPaused and code 2", async () => {
    const pauseTx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "Pause",
      [],
      0,
      false,
      false
    );
    console.log(
      `SwapZilForToken: paused  => tx: ${JSON.stringify(pauseTx.receipt)}`
    );
    expect(pauseTx.receipt.success).toEqual(true);

    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(toZil(currency.COMMISSION));

    const minZilAmountWithoutCommission = minZilAmount - commissionAmount;

    const serialize = await serializeLinearSwapData(
      minZilAmountWithoutCommission,
      minTokenAmount,
      commissionAmount,
      deadLineBlock,
      1
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapZilForToken",
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
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(
      `SwapZilForToken: When the contract is paused => tx: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("SwapZilForToken: When incorrect block number sent should throws IsBlockNumberWithinRange and code 6", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const newDeadLineBlock = deadLineBlock - 10;

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(toZil(currency.COMMISSION));

    const minZilAmountWithoutCommission = minZilAmount - commissionAmount;

    const serialize = await serializeLinearSwapData(
      minZilAmountWithoutCommission,
      minTokenAmount,
      commissionAmount,
      newDeadLineBlock,
      1
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapZilForToken",
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
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(
      `SwapZilForToken: When incorrect block number sent => tx: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("SwapZilForToken: When zil is deposited without commission", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(toZil(currency.COMMISSION));

    const minZilAmountWithoutCommission = minZilAmount - commissionAmount;

    const serialize = await serializeLinearSwapData(
      minZilAmount,
      minTokenAmount,
      commissionAmount,
      deadLineBlock,
      1
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapZilForToken",
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
      minZilAmountWithoutCommission,
      false,
      true
    );

    console.log(
      `SwapZilForToken: When zil is deposited with commission => tx: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("SwapZilForToken: Test passing the signature of SwapTokenForZil transition to SwapZilForToken. Should Failed.", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(toZil(currency.COMMISSION));

    const serialize = await serializeLinearSwapData(
      minZilAmount,
      minTokenAmount,
      commissionAmount,
      deadLineBlock,
      2
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapZilForToken",
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
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(
      `SwapZilForToken: Success => tx: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("SwapZilForToken: Success", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(toZil(currency.COMMISSION));
    console.log(
      "-===============================>",
      minZilAmount,
      minTokenAmount,
      commissionAmount,
      deadLineBlock
    );
    const serialize = await serializeLinearSwapData(
      minZilAmount,
      minTokenAmount,
      commissionAmount,
      deadLineBlock,
      1
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapZilForToken",
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
      toZil(currency.ZIL),
      false,
      true
    );

    console.log(
      `SwapZilForToken: Success => tx: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(true);
  });
});
/* Swap Token for Zil */
describe("Token Swap => Swap Token For Zil", () => {
  beforeEach(async () => {
    // Contract Deployments
    const fungibleTokenDeployParams = {
      name: "Test T7",
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

    const [dexContract] = await deployLinearSwapDexContract(
      accounts[0].privateKey,
      {
        initialOwnerAddress: accounts[0].address,
        initialTokenContract: tokenAddress,
        pubKey: metaData.pubKey,
      }
    );
    dexAddress = dexContract.address;
    log("dexAddress =", dexAddress);
    if (dexAddress === undefined) {
      throw new Error("Failed to deploy token swap contract.");
    }

    log("Transfer token to user address");
    await callContract(
      accounts[0].privateKey,
      tokenAddress,
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

    log("Give allowance to the token swap contract");
    await callContract(
      accounts[3].privateKey,
      tokenAddress,
      "IncreaseAllowance",
      [
        {
          vname: "spender",
          type: "ByStr20",
          value: dexAddress,
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

    log("Add Zil Liquidity to token swap contract");
    await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddZil",
      [],
      toZil(currency.ZIL),
      false,
      true
    );

    log("Add Commission Recipient Address");
    await callContract(
      accounts[0].privateKey,
      dexAddress,
      "AddCommissionRecipientAddress",
      [
        {
          vname: "address",
          type: "ByStr20",
          value: accounts[2].address,
        },
      ],
      0,
      false,
      true
    );
  });

  test("SwapTokenForZil: When the contract is paused should throws RequireNotPaused and code 2", async () => {
    const pauseTx = await callContract(
      accounts[0].privateKey,
      dexAddress,
      "Pause",
      [],
      0,
      false,
      false
    );
    console.log(
      `SwapTokenForZil: paused  => tx: ${JSON.stringify(pauseTx.receipt)}`
    );
    expect(pauseTx.receipt.success).toEqual(true);

    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(
      toToken(currency.COMMISSION, tokenDecimal)
    );

    const minTokenAmountWithoutCommission = minTokenAmount - commissionAmount;

    const serialize = await serializeLinearSwapData(
      minTokenAmountWithoutCommission,
      minZilAmount,
      commissionAmount,
      deadLineBlock,
      2
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapTokenForZil",
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
      `SwapTokenForZil: When the contract is paused => tx: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("SwapTokenForZil: When incorrect block number sent should throws IsBlockNumberWithinRange and code 6", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const newDeadLineBlock = deadLineBlock - 10;

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(
      toToken(currency.COMMISSION, tokenDecimal)
    );

    const serialize = await serializeLinearSwapData(
      minTokenAmount,
      minZilAmount,
      commissionAmount,
      newDeadLineBlock,
      2
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapTokenForZil",
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
      `SwapTokenForZil: When incorrect block number sent => tx: ${JSON.stringify(
        tx.receipt
      )}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  test("SwapTokenForZil: Test passing the signature of SwapZilForToken transition to SwapTokenForZil. Should Failed.", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(
      toToken(currency.COMMISSION, tokenDecimal)
    );

    const serialize = await serializeLinearSwapData(
      minTokenAmount,
      minZilAmount,
      commissionAmount,
      deadLineBlock,
      1
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapTokenForZil",
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
      `SwapTokenForZil: Success => tx: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(false);
  });

  /* Swap Token with zils */
  test("SwapTokenForZil: Success", async () => {
    const deadLineBlock = await getCurrentBlockNumber();

    const minZilAmount = parseInt(toZil(currency.ZIL));

    const minTokenAmount = parseInt(toToken(currency.TOKEN, tokenDecimal));

    const commissionAmount = parseInt(
      toToken(currency.COMMISSION, tokenDecimal)
    );

    const serialize = await serializeLinearSwapData(
      minTokenAmount,
      minZilAmount,
      commissionAmount,
      deadLineBlock,
      2
    );

    const signedData = signData(serialize);

    const tx = await callContract(
      accounts[3].privateKey,
      dexAddress,
      "SwapTokenForZil",
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
      `SwapTokenForZil: Success => tx: ${JSON.stringify(tx.receipt)}`
    );
    expect(tx.receipt.success).toEqual(true);
  });
});
