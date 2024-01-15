import { ethers } from "hardhat";
import { config } from "../config";

async function main() {
  const tokenManagerAddress = config.zq.tokenManager;

  const localToken = config.zq.token;
  const remoteToken = config.zq.remoteToken;
  const remoteTokenManager = config.zq.remoteTokenManager;
  const remoteChainId = config.zq.remoteChainId;

  const tokenManager = await ethers.getContractAt(
    "LockAndReleaseTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const tx = await tokenManager.registerToken(
    localToken,
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
