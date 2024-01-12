import { ethers } from "hardhat";

async function main() {
  const address = "0x2Af4AdC637dB3C19127E8bAb41B9539A93FcB2ed";
  const ZRC2Proxy = await ethers.deployContract("EIP20ZRC2Proxy", [address]);

  await ZRC2Proxy.waitForDeployment();

  console.log("ZRC2Proxy deployed to:", ZRC2Proxy.target);
  console.log(await ZRC2Proxy.name());
  console.log(await ZRC2Proxy.totalSupply());
  console.log(await ZRC2Proxy.decimals());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
