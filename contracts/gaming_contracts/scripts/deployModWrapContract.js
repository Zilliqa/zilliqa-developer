const { deployContract } = require("./utils/deploy.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const util = require("util");
const fs = require("fs");
const readFile = util.promisify(fs.readFile);
const { getMetaData } = require("./utils/helper");
const { contractDir } = getMetaData();

async function deployModWrapContract(
  privateKey,
  pub_key,
  tokenOwnerAddress,
  revenueAddress
) {
  // Check for key
  if (!privateKey || privateKey === "") {
    throw new Error("No private key was provided!");
  }

  // Generate default vars
  const address = getAddressFromPrivateKey(privateKey);

  // Load code and contract initialization variables
  const code = (await readFile(`${contractDir}/mod_wrap.scilla`)).toString();

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
      vname: "initial_revenue_recipient",
      type: "ByStr20",
      value: `${revenueAddress}`,
    },
    {
      vname: "pub_key",
      type: "ByStr33",
      value: `${pub_key}`,
    },
  ];

  console.info(`Deploying Mod Wrap token.`);
  return await deployContract(privateKey, address, code, init);
}

exports.deployModWrapContract = deployModWrapContract;
