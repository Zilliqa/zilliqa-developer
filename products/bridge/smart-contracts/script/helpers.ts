import { ethers } from "hardhat";

export async function getRelayEvents(chainGatewayAddress: string) {
  const chainGateway = await ethers.getContractAt(
    "ChainGateway",
    chainGatewayAddress
  );

  const nonce = await chainGateway.nonce();
  console.log(nonce);

  const blockNumber = await ethers.provider.getBlock("latest");
  console.log("Current block number", blockNumber);

  const filter = chainGateway.filters.Relayed();
  const events = await chainGateway.queryFilter(
    filter,
    (blockNumber?.number || 0) - 1000,
    blockNumber?.number
  );
  console.log(events);

  return events;
}
