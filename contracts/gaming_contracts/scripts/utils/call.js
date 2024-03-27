const { TESTNET_VERSION, zilliqa, useKey } = require("./zilliqa");
const { BN, Long, units } = require("@zilliqa-js/util");
const { config } = require("../../test.config");
const { getDefaultNetwork } = require("./helper");
const network = getDefaultNetwork();

async function getBalance(address) {
  const balanceRes = await zilliqa.blockchain.getBalance(address);
  if (balanceRes && balanceRes.result && balanceRes.result.balance) {
    return balanceRes.result.balance;
  }

  return 0;
}

async function callContract(
  privateKey,
  contractAddress,
  transition,
  args,
  zilsToSend = 0
) {
  const contract = await zilliqa.contracts.at(contractAddress);
  // Check for key
  if (!privateKey || privateKey === "") {
    throw new Error("No private key was provided!");
  }
  useKey(privateKey);

  const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();

  console.info(`Calling: ${transition}: ${JSON.stringify(args)}`);

  const callTx = await contract.call(
    transition,
    args,
    {
      version: TESTNET_VERSION,
      amount: new BN(zilsToSend),
      gasPrice: new BN(minGasPrice.result),
      gasLimit: Long.fromNumber(25000),
    },
    33,
    1000,
    true
  );

  console.log(`Transaction Id ${callTx.id}`);
  if (!callTx.id) {
    throw new Error(JSON.stringify("Failed to get tx id!", null, 2));
  }

  return callTx;
}

async function getCurrentBlockNumber() {
  if (
    config.defaultNetwork === "local_isolated_server" ||
    config.defaultNetwork === "isolated_server"
  ) {
    const body = {
      id: "1",
      jsonrpc: "2.0",
      method: "GetBlocknum",
      params: [""],
    };
    const res = await fetch(network.url, {
      method: "POST",
      body: JSON.stringify(body),
      headers: { "Content-Type": "application/json" },
    });
    const data = await res.json();
    return Number(data.result);
  } else {
    console.log(`Fetching current block`);
    const numTxBlock = await zilliqa.blockchain.getNumTxBlocks();
    const currentBlock = numTxBlock.result;
    console.log(`${JSON.stringify(currentBlock)}`);
    return Number(currentBlock);
  }
}

async function getSmartContractInit(tokenAddress) {
  const smartContractInit = await zilliqa.blockchain.getSmartContractInit(
    tokenAddress
  );
  console.log(`getSmartContractInit: ${JSON.stringify(smartContractInit)}`);
  let result = null;
  if (smartContractInit) {
    if (smartContractInit.error) {
      throw smartContractInit.error.message;
    }
    const result = smartContractInit.result;

    const contractInit = {};
    for (const entry of result) {
      contractInit[entry.vname] = entry.value;
    }
    result = contractInit;
  }
  return result;
}

async function getSmartContractState(tokenAddress) {
  const smartContractState = await zilliqa.blockchain.getSmartContractState(
    tokenAddress
  );
  console.log(`getSmartContractState: ${JSON.stringify(smartContractState)}`);
  let result = null;
  if (smartContractState) {
    if (smartContractState.error) {
      throw smartContractState.error.message;
    }
    result = smartContractState.result;
  }
  return result;
}

async function sendZil(
  privateKey,
  recipientAddress,
  sendingAmount,
  gasLimit = 25000
) {
  let blockchainTxnId = null;
  try {
    useKey(privateKey);
    const minGasPrice = await zilliqa.blockchain.getMinimumGasPrice();
    let tx = zilliqa.transactions.new({
      version: TESTNET_VERSION,
      toAddr: recipientAddress,
      amount: units.toQa(sendingAmount, units.Units.Zil),
      gasPrice: new BN(minGasPrice.result),
      gasLimit: Long.fromNumber(gasLimit),
    });
    // Send a transaction to the network
    tx = await zilliqa.blockchain.createTransactionWithoutConfirm(tx);
    blockchainTxnId = tx.id;
    console.log("The sendZil transaction id is:", tx.id);

    console.log("Waiting transaction be confirmed");
    return await tx.confirm(tx.id, 33, 2000);
  } catch (err) {
    console.log("sendZil error:");
    console.log(err);
    throw err.error;
  }
}

exports.callContract = callContract;
exports.getBalance = getBalance;
exports.getCurrentBlockNumber = getCurrentBlockNumber;
exports.getSmartContractInit = getSmartContractInit;
exports.getSmartContractState = getSmartContractState;
exports.sendZil = sendZil;
