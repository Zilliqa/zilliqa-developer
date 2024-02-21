import "@nomicfoundation/hardhat-toolbox";
import { config } from "../script/config";
import { task } from "hardhat/config";

task("latest-nonce", "Check the latest nonce of the chain gateway").setAction(
  async (_, hre) => {
    let chainGatewayAddress;
    let targetChainId;
    let targetChainName;

    switch (hre.network.name) {
      case "bsc":
        chainGatewayAddress = config.bsc.chainGateway;
        targetChainId = config.bsc.remoteChainId;
        targetChainName = "ZQ";
        break;
      case "zq":
        chainGatewayAddress = config.zq.chainGateway;
        targetChainId = config.zq.remoteChainId;
        targetChainName = "BSC";
        break;
      default:
        throw new Error("Invalid source chain");
    }

    const chainGateway = await hre.ethers.getContractAt(
      "ChainGatewayUpgradeable",
      chainGatewayAddress
    );

    console.log(
      `${
        hre.network.name
      } latest nonce relayed to ${targetChainName} ${await chainGateway.nonce(
        targetChainId
      )}`
    );
  }
);

task(
  "verify-dispatched",
  "Check that every nonce has been dispatched on the target chain"
)
  .addParam("latestNonce", "Latest nonce on the target chain")
  .setAction(async ({ latestNonce }, hre) => {
    hre.network.name;
    let chainGatewayAddress;
    let targetChainId;

    switch (hre.network.name) {
      case "bsc":
        chainGatewayAddress = config.bsc.chainGateway;
        targetChainId = config.bsc.remoteChainId;
        break;
      case "zq":
        chainGatewayAddress = config.zq.chainGateway;
        targetChainId = config.zq.remoteChainId;
        break;
      default:
        throw new Error("Invalid source chain");
    }

    const chainGateway = await hre.ethers.getContractAt(
      "ChainGatewayUpgradeable",
      chainGatewayAddress
    );

    let notDispatched = 0;

    for (let i = 1; i <= latestNonce; i++) {
      if (!(await chainGateway.dispatched(targetChainId, i))) {
        console.log("Tx with nonce: ", i, "not dispatched");
        notDispatched++;
      }
    }

    if (notDispatched === 0) {
      return console.log(
        `All transactions up to nonce ${latestNonce} dispatched`
      );
    }

    console.log(`${notDispatched} transactions not dispatched`);
  });

task("txn-payload", "Get the txn payload from relay event from source chain")
  .addParam("nonce", "Nonce of the txn to retrieve")
  .setAction(async ({ latestNonce }, hre) => {
    hre.network.name;
    let chainGatewayAddress;
    let targetChainId;

    switch (hre.network.name) {
      case "bsc":
        chainGatewayAddress = config.bsc.chainGateway;
        targetChainId = config.bsc.remoteChainId;
        break;
      case "zq":
        chainGatewayAddress = config.zq.chainGateway;
        targetChainId = config.zq.remoteChainId;
        break;
      default:
        throw new Error("Invalid source chain");
    }

    const chainGateway = await hre.ethers.getContractAt(
      "ChainGatewayUpgradeable",
      chainGatewayAddress
    );
  });
