import { getRelayEvents } from "../helpers";
import { config } from "../config";

async function main() {
  const chainGatewayAddress = config.bsc.chainGateway;

  const events = await getRelayEvents(chainGatewayAddress);

  console.log(events.map((e) => e.args));
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
