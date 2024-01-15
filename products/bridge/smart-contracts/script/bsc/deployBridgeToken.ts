import { ethers } from "hardhat";

async function main() {
  const tokenManagerAddress = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";

  const tokenName = "WEB3WAR Token";
  const tokenSymbol = "FPS";
  const tokenDecimals = 12;
  const remoteToken = "0x63B6ebD476C84bFDd5DcaCB3f974794FC6C2e721";
  const remoteTokenManager = "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C";
  const remoteChainId = 33101;

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
