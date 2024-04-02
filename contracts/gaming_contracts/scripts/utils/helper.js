const {
  bytes,
  getAddressFromPrivateKey,
  getPubKeyFromPrivateKey,
} = require("@zilliqa-js/zilliqa");
const { SHA256, enc } = require("crypto-js");
const elliptic = require("elliptic");
const { BN, units } = require("@zilliqa-js/util");
const { config } = require("../../test.config");

function toHexArray(input) {
  const hexArr = [];
  for (let n = 0, l = input.length; n < l; n++) {
    const hex = Number(input.charCodeAt(n)).toString(16);
    hexArr.push(hex);
  }
  return hexArr.join("");
}

function toToken(num, decimal) {
  let s = num.toString();
  let i = 0;
  while (i < decimal) {
    s = s + "0";
    i++;
  }
  return s;
}

function toZil(num) {
  return String(new BN(units.toQa(num, units.Units.Zil)));
}

function fromZil(num) {
  return String(units.fromQa(new BN(num), units.Units.Zil));
}

function serializeLinearSwapData(
  fromAmount,
  toAmount,
  commissionAmount,
  currentBlockNumber,
  transition
) {
  const currentBlockNumberHexArray = bytes.intToHexArray(
    currentBlockNumber,
    32
  );

  const fromAmountHexArray = bytes.intToHexArray(fromAmount, 16);

  const toAmountHexArray = bytes.intToHexArray(toAmount, 16);

  const commissionAmountHexArray = bytes.intToHexArray(commissionAmount, 16);

  const transitionIsHexArray = bytes.intToHexArray(transition, 8);

  // Concat data to serialize
  const serializeData = currentBlockNumberHexArray
    .concat(fromAmountHexArray)
    .concat(toAmountHexArray)
    .concat(commissionAmountHexArray)
    .concat(transitionIsHexArray)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

function serializeClaimData(timestamp) {
  const currentTimestampHexArray = bytes.intToHexArray(timestamp, 32);

  // Concat data to serialize
  const serializeData = currentTimestampHexArray.join("");

  console.log("currentTimestampHexArray", currentTimestampHexArray);

  return Promise.resolve(`0x${serializeData}`);
}

function toHexStringHash(input) {
  let onlyHexStr = input;
  if (input.startsWith("0x")) {
    onlyHexStr = input.substring(2);
  }

  const word = enc.Hex.parse(onlyHexStr);

  const shaArray = SHA256(word);

  return shaArray.toString();
}

function verifySignedData(serializeDataHash, signature) {
  const ec = new elliptic.ec("secp256k1");
  const privateKeyDecimal = ec
    .keyFromPrivate(serializeDataHash, "hex")
    .getPrivate();

  const recoveredPubKey = ec.recoverPubKey(
    privateKeyDecimal,
    signature,
    signature.recoveryParam,
    "hex"
  );

  console.log(
    "Recovered Public Key\n",
    `0x${recoveredPubKey.encodeCompressed("hex")}`
  );

  const valid = ec.verify(privateKeyDecimal, signature, recoveredPubKey);
  return valid;
}

function signData(serializeData) {
  const dataHash = toHexStringHash(serializeData);
  console.log("Data Hash\n", dataHash);
  const ec = new elliptic.ec("secp256k1");

  const metaData = getMetaData();

  let privateKey = metaData.privKey;
  if (privateKey?.startsWith("0x")) {
    privateKey = privateKey.substring(2);
  }

  const keyPair = ec.keyFromPrivate(privateKey);
  const signature = ec.sign(dataHash, keyPair, "hex", { canonical: true });

  // Verify signature before return
  const validSignature = verifySignedData(dataHash, signature);

  if (!validSignature) {
    throw "Unable to generate valid signature to mint";
  }

  const signature64Bytes = Buffer.concat([
    signature.r.toArrayLike(Buffer, "be", 32),
    signature.s.toArrayLike(Buffer, "be", 32),
  ]);

  const signatureHexString = signature64Bytes.toString("hex");
  return `0x${signatureHexString}`;
}

function serializeMintingData(data, currentBlockNumber) {
  //const currentBlockNumberStr = currentBlockNumber?.toString();
  const currentBlockNumberHexArray = bytes.intToHexArray(
    currentBlockNumber,
    32
  );

  // Convert wallet address to hex array
  let walletAddress = data.OwnerWalletAddress;
  if (walletAddress.startsWith("0x")) {
    walletAddress = walletAddress.substring(2);
  }

  const walletAddressHexArray = [walletAddress];

  // Convert ZRC6 smart contract address to hex array
  let contractAddress = data.ZRC6ContractAddress;
  if (contractAddress.startsWith("0x")) {
    contractAddress = contractAddress.substring(2);
  }

  const contractAddressHexArray = [contractAddress];

  // Convert token URI to hex string
  const tokenURIHexString = toHexArray(data.TokenURI);

  // Concat data to serialize
  let serializeData = currentBlockNumberHexArray
    .concat(walletAddressHexArray)
    .concat(contractAddressHexArray)
    .concat(tokenURIHexString)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

function serializePurchaseData(data, currentBlockNumber) {
  const currentBlockNumberHexArray = bytes.intToHexArray(
    currentBlockNumber,
    32
  );

  // Convert wallet address to hex array
  let walletAddress = data.OwnerWalletAddress;
  if (walletAddress.startsWith("0x")) {
    walletAddress = walletAddress.substring(2);
  }

  const walletAddressHexArray = [walletAddress];

  // Convert ZRC6 smart contract address to hex array
  let contractAddress = data.ZRC6ContractAddress;
  if (contractAddress.startsWith("0x")) {
    contractAddress = contractAddress.substring(2);
  }

  const contractAddressHexArray = [contractAddress];

  // Convert payment token address to hex array
  let paymentTokenAddress = data.PaymentToken;
  if (paymentTokenAddress.startsWith("0x")) {
    paymentTokenAddress = paymentTokenAddress.substring(2);
  }

  const paymentTokenAddressHexArray = [paymentTokenAddress];

  // Convert payment amount to hex array
  let paymentAmount = data.PaymentAmount;
  const paymentAmountHexArray = bytes.intToHexArray(paymentAmount, 16);

  // Convert token URI to hex string
  const tokenURIHexString = toHexArray(data.TokenURI);

  // Concat data to serialize
  let serializeData = currentBlockNumberHexArray
    .concat(walletAddressHexArray)
    .concat(contractAddressHexArray)
    .concat(paymentTokenAddressHexArray)
    .concat(paymentAmountHexArray)
    .concat(tokenURIHexString)
    .join("");

  return Promise.resolve(`0x${serializeData}`);
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function addInterval(date, units, interval) {
  if (!(date instanceof Date)) return undefined;

  var ret = new Date(date); //don't change original date
  var checkRollover = function () {
    if (ret.getDate() != date.getDate()) ret.setDate(0);
  };
  switch (String(interval).toLowerCase()) {
    case "year":
      ret.setFullYear(ret.getFullYear() + units);
      checkRollover();
      break;
    case "quarter":
      ret.setMonth(ret.getMonth() + 3 * units);
      checkRollover();
      break;
    case "month":
      ret.setMonth(ret.getMonth() + units);
      checkRollover();
      break;
    case "week":
      ret.setDate(ret.getDate() + 7 * units);
      break;
    case "day":
      ret.setDate(ret.getDate() + units);
      break;
    case "hour":
      ret.setTime(ret.getTime() + units * 3600000);
      break;
    case "minute":
      ret.setTime(ret.getTime() + units * 60000);
      break;
    case "second":
      ret.setTime(ret.getTime() + units * 1000);
      break;
    default:
      ret = undefined;
      break;
  }
  return ret;
}

function getDefaultNetwork() {
  return config.networks[config.defaultNetwork];
}

function getAccounts() {
  const network = getDefaultNetwork();
  const accounts = [];
  network.accounts.map((v) => {
    accounts.push({
      address: getAddressFromPrivateKey(v),
      publicKey: getPubKeyFromPrivateKey(v),
      privateKey: v,
    });
  });
  return accounts;
}

function getMetaData() {
  const network = getDefaultNetwork();
  return network.metaData;
}

exports.toToken = toToken;
exports.toZil = toZil;
exports.serializeClaimData = serializeClaimData;
exports.serializeLinearSwapData = serializeLinearSwapData;
exports.sleep = sleep;
exports.toToken = toToken;
exports.serializeMintingData = serializeMintingData;
exports.serializePurchaseData = serializePurchaseData;
exports.signData = signData;
exports.toHexArray = toHexArray;
exports.addInterval = addInterval;
exports.getAccounts = getAccounts;
exports.getDefaultNetwork = getDefaultNetwork;
exports.fromZil = fromZil;
exports.getMetaData = getMetaData;
