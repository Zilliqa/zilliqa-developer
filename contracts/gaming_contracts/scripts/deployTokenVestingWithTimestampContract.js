const { deployContract } = require("./utils/deploy.js");
const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
const util = require("util");
const fs = require("fs");
const readFile = util.promisify(fs.readFile);
const { getMetaData } = require("./utils/helper");
const { contractDir } = getMetaData();

async function deployTokenVestingWithTimestampContract(
  deployerPrivateKey,
  initialOwnerAddress = null,
  initialTokenContract = null,
  withToken = false,
) {
  // Check for key
  if (!deployerPrivateKey || deployerPrivateKey === "") {
    throw new Error("No private key was provided!");
  }

  // Default vars
  const address = getAddressFromPrivateKey(deployerPrivateKey);

  const fileName = !withToken ? "token-vesting-timestamp" : "token-vesting-timestamp-with-token";
  console.log("Filename:", fileName);

  // Load code and contract initialization variables
  const code = (
    await readFile(`${contractDir}/${fileName}.scilla`)
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
  ];

  console.info("Deploying Token Vesting Contract...");
  return deployContract(deployerPrivateKey, address, code, init);
}

exports.deployTokenVestingWithTimestampContract =
  deployTokenVestingWithTimestampContract;
