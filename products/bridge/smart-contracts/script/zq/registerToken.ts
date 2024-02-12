import { ethers } from "hardhat";
import { config } from "../config";
import { ITokenManagerStructs } from "../../typechain-types/contracts/periphery/LockAndReleaseTokenManagerUpgradeable.sol/ILockAndReleaseTokenManager";

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

  const remoteTokenStruct: ITokenManagerStructs.RemoteTokenStruct = {
    token: remoteToken,
    tokenManager: remoteTokenManager,
    chainId: remoteChainId,
  };

  console.log("start register");
  const tx = await tokenManager.registerToken(localToken, remoteTokenStruct);
  const receipt = await tx.wait();
  console.log(receipt);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
