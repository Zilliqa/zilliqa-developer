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

task("get-relayed", "Get every relayed event from chain").setAction(
  async (_, hre) => {
    if (hre.network.name === "bsc") {
      // Ensure to use quicknode RPC for BSC, can be updated in hardhat.config.ts
      // BSC quicknode limits query block interval to 10_000
      const chainGateway = await hre.ethers.getContractAt(
        "ChainGatewayUpgradeable",
        config.bsc.chainGateway
      );
      const latestBlockNumber = await hre.ethers.provider.getBlockNumber();
      const interval = 10_000;
      const deployedBlock = config.bsc.gatewayDeployedBlock;

      const filter = chainGateway.filters.Relayed;
      for (let i = deployedBlock; i <= latestBlockNumber; i += interval) {
        const events = await chainGateway.queryFilter(filter, i, i + 10000);
        if (events.length > 0) {
          events.forEach((e) => {
            console.log(`Block: ${e.blockNumber} | Tx: ${e.blockHash}`);
            console.log(`SourceChainId: ${e.args[0]}`);
            console.log(`Target: ${e.args[1]}`);
            console.log(`Call: ${e.args[2]}`);
            console.log(`GasLimit: ${e.args[3]}`);
            console.log(`Nonce: ${e.args[4]}\n`);
          });
        }
      }
      return;
    } else if (hre.network.name === "zq") {
      // ZQ only persists the latest 100 blocks of events
      const chainGateway = await hre.ethers.getContractAt(
        "ChainGatewayUpgradeable",
        config.zq.chainGateway
      );
      const filter = chainGateway.filters.Relayed;
      const latestBlockNumber = await hre.ethers.provider.getBlockNumber();
      const events = await chainGateway.queryFilter(
        filter,
        latestBlockNumber - 100,
        latestBlockNumber
      );
      // ZQ sometimes repeats some events when queried
      if (events.length > 0) {
        events.forEach((e) => {
          console.log(`Block: ${e.blockNumber} | Tx: ${e.blockHash}`);
          console.log(`SourceChainId: ${e.args[0]}`);
          console.log(`Target: ${e.args[1]}`);
          console.log(`Call: ${e.args[2]}`);
          console.log(`GasLimit: ${e.args[3]}`);
          console.log(`Nonce: ${e.args[4]}\n`);
        });
      }
      return;
    }

    throw new Error("Invalid source chain");
  }
);
