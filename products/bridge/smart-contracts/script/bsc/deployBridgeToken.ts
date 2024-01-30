import { ethers } from "hardhat";
import { config } from "../config";

async function main() {
  const tokenManagerAddress = config.bsc.tokenManager;

  const tokenName = "WEB3WAR Token";
  const tokenSymbol = "FPS";
  const tokenDecimals = 12;
  const remoteToken = config.bsc.remoteToken;
  const remoteTokenManager = config.bsc.remoteTokenManager;
  const remoteChainId = config.bsc.remoteChainId;

  const tokenManager = await ethers.getContractAt(
    "MintAndBurnTokenManagerUpgradeable",
    tokenManagerAddress
  );
  const tx = await tokenManager[
    "deployToken(string,string,uint8,address,address,uint256)"
  ](
    tokenName,
    tokenSymbol,
    tokenDecimals,
    remoteToken,
    remoteTokenManager,
    remoteChainId
  );
  const receipt = await tx.wait();
  console.log(receipt);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
