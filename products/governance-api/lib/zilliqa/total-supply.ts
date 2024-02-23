import { Zilliqa } from "@zilliqa-js/zilliqa";
import { fromBech32Address } from "@zilliqa-js/crypto";

export async function getTotalSupply(token: string) {
  const { blockchain } = new Zilliqa("https://api.zilliqa.com");
  const base16 = fromBech32Address(token);
  const field = "total_supply";
  const res = await blockchain.getSmartContractSubState(base16, field);

  if (res.error) {
    throw new Error(res.error.message);
  }

  if (res && res.result && res.result[field]) {
    return res.result[field];
  }

  return "0";
}
