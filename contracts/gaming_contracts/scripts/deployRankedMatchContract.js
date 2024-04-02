const { deployContract } = require("./utils/deploy.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const util = require("util");
const fs = require("fs");
const readFile = util.promisify(fs.readFile);
const { getMetaData } = require("./utils/helper");
const { contractDir } = getMetaData();

async function deployRankedMatchContract(privateKey, deployParams) {
  // Check for key
  if (!privateKey || privateKey === "") {
    throw new Error("No private key was provided!");
  }

  // Generate default vars
  const address = getAddressFromPrivateKey(privateKey);

  // Load code and contract initialization variables
  const code = (
    await readFile(`${contractDir}/ranked_match.scilla`)
  ).toString();

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
      value: `${address}`,
    },
    {
      vname: "initial_entry_fee",
      type: "Uint128",
      value: `${deployParams.entryFee}`,
    },
    {
      vname: "initial_token_contract",
      type: "ByStr20",
      value: `${deployParams.tokenContract}`,
    },
    {
      vname: "pub_key",
      type: "ByStr33",
      value: `${deployParams.pubKey}`,
    },
  ];

  console.info(`Deploying Ranked Match contract...`);
  return await deployContract(privateKey, address, code, init);
}

module.exports.deployRankedMatchContract = deployRankedMatchContract;
