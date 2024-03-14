const { deployContract } = require("./utils/deploy.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const util = require("util");
const fs = require("fs");
const readFile = util.promisify(fs.readFile);
const { getMetaData } = require("./utils/helper");
const { contractDir } = getMetaData();

async function deployStakingWithTimestampContract(
  deployerPrivateKey,
  initialOwnerAddress,
  initialStakingToken
) {
  // Check for key
  if (!deployerPrivateKey || deployerPrivateKey === "") {
    throw new Error("No private key was provided!");
  }

  // Default vars
  const address = getAddressFromPrivateKey(deployerPrivateKey);

  // Load code and contract initialization variables
  const code = (
    await readFile(`${contractDir}/staking-timestamp.scilla`)
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
      vname: "initial_staking_token_address",
      type: "ByStr20",
      value: initialStakingToken,
    },
  ];

  console.info("Deploying Staking Contract...");
  return deployContract(deployerPrivateKey, address, code, init);
}

exports.deployStakingWithTimestampContract = deployStakingWithTimestampContract;
