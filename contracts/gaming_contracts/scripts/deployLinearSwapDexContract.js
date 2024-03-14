const { deployContract } = require("./utils/deploy.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const util = require("util");
const fs = require("fs");
const readFile = util.promisify(fs.readFile);
const { getMetaData } = require("./utils/helper");
const { contractDir } = getMetaData();

async function deployLinearSwapDexContract(
  deployerPrivateKey,
  { initialOwnerAddress = null, initialTokenContract = null, pubKey = null }
) {
  // Check for key
  if (!deployerPrivateKey || deployerPrivateKey === "") {
    throw new Error("No private key was provided!");
  }

  // Default vars
  const address = getAddressFromPrivateKey(deployerPrivateKey);

  // Load code and contract initialization variables
  const code = (
    await readFile(`${contractDir}/linear-swap-dex.scilla`)
  ).toString();
  const init = [
    // this parameter is mandatory for all init arrays
    {
      vname: "_scilla_version",
      type: "Uint32",
      value: "0",
    },
    {
      vname: "initial_owner",
      type: "ByStr20",
      value: initialOwnerAddress,
    },
    {
      vname: "initial_token_contract",
      type: "ByStr20",
      value: initialTokenContract,
    },
    {
      vname: "pub_key",
      type: "ByStr33",
      value: pubKey,
    },
  ];

  console.info("Deploying Token Swap Contract...");
  return deployContract(deployerPrivateKey, address, code, init);
}

exports.deployLinearSwapDexContract = deployLinearSwapDexContract;
