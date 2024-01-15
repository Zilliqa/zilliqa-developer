import { ethers } from "hardhat";
import { config } from "../config";

async function main() {
  const tokenManagerAddress = config.bsc.tokenManager;
  const localToken = config.bsc.token;
  const remoteChainID = config.bsc.remoteChainId;

  const tokenManager = await ethers.getContractAt(
    "MintAndBurnTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const res = await tokenManager.getRemoteTokens(localToken, remoteChainID);
  console.log(res);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
