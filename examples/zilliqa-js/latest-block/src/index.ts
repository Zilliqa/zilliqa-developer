import { Zilliqa } from "@zilliqa-js/zilliqa";

async function main() {
  const provider = new Zilliqa("https://api.zilliqa.com/");
  const latestBlock = await provider.blockchain.getLatestTxBlock();
  console.log(latestBlock);
}
main();
