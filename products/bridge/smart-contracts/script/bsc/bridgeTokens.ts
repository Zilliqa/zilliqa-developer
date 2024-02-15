import { ethers } from "hardhat";
import { getRelayEvents } from "../helpers";
import { config } from "../config";

async function main() {
  let tx, receipt;
  const tokenManagerAddress = config.bsc.tokenManager;
  const chainGatewayAddress = config.bsc.chainGateway;

  const tokenAddress = config.bsc.token;
  const remoteChainID = config.bsc.remoteChainId;
  const remoteRecipient = config.bsc.remoteRecipient;
  const amount = config.bsc.amount;

  const tokenManager = await ethers.getContractAt(
    "MintAndBurnTokenManagerUpgradeable",
    tokenManagerAddress
  );

  const token = await ethers.getContractAt("ERC20", tokenAddress);
  tx = await token.approve(tokenManagerAddress, ethers.MaxUint256);
  receipt = await tx.wait();
  console.log("Approve Tx", receipt?.hash);

  tx = await tokenManager.transfer(
    tokenAddress,
    remoteChainID,
    remoteRecipient,
    amount
  );
  receipt = await tx.wait();
  console.log("Transfer Tx", receipt?.hash);

  const events = await getRelayEvents(chainGatewayAddress);
  console.log(
    "Events",
    events.map((e) => e.args)
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
