const { deployContract } = require("./utils/deploy.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const util = require("util");
const fs = require("fs");
const readFile = util.promisify(fs.readFile);
const { getMetaData } = require("./utils/helper");
const { contractDir } = getMetaData();

const randomHex = (size) =>
  [...Array(size)]
    .map(() => Math.floor(Math.random() * 16).toString(16))
    .join("");

async function deployNonFungibleToken(
  privateKey,
  deployParams,
  tokenOwnerAddress
) {
  // Check for key
  if (!privateKey || privateKey === "") {
    throw new Error("No private key was provided!");
  }

  // Generate default vars
  const address = getAddressFromPrivateKey(privateKey);
  const symbol = deployParams.symbol || `TEST-${randomHex(4).toUpperCase()}`;

  // Load code and contract initialization variables
  const code = (await readFile(`${contractDir}/zrc6.scilla`)).toString();

  const init = [
    // this parameter is mandatory for all init arrays
    {
      vname: "_scilla_version",
      type: "Uint32",
      value: "0",
    },
    {
      vname: "initial_contract_owner",
      type: "ByStr20",
      value: `${tokenOwnerAddress}`,
    },
    {
      vname: "name",
      type: "String",
      value: `${deployParams.name}`,
    },
    {
      vname: "symbol",
      type: "String",
      value: `${symbol}`,
    },
    {
      vname: "initial_base_uri",
      type: "String",
      value: `${deployParams.tokenUrl}`,
    },
  ];

  console.info(`Deploying Non Fungible token ${symbol}...`);
  return await deployContract(privateKey, address, code, init);
}

exports.deployNonFungibleToken = deployNonFungibleToken;
