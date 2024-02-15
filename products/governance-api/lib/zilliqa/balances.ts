import { Zilliqa } from "@zilliqa-js/zilliqa";
import { fromBech32Address } from "@zilliqa-js/crypto";
import { cloneDeep } from "lodash";
import BN from "bn.js";

const { blockchain } = new Zilliqa("https://api.zilliqa.com");
const zilswap = "0xBa11eB7bCc0a02e947ACF03Cc651Bfaf19C9EC00";
const _100 = new BN(100000);
const _zero = new BN(0);

export async function getDEXpools(token: string) {
  const field = "pools";
  const res = await blockchain.getSmartContractSubState(zilswap, field, [
    token,
  ]);

  if (res.error) {
    throw new Error(res.error.message);
  }

  if (!res || !res.result || !res.result[field] || !res.result[field][token]) {
    return new BN("0");
  }

  const [, tokenReserve] = res.result[field][token]["arguments"];

  return new BN(tokenReserve);
}

export async function getDEXbalance(token: string) {
  const field = "balances";
  const res = await blockchain.getSmartContractSubState(zilswap, field, [
    token,
  ]);

  if (res.error) {
    throw new Error(res.error.message);
  }

  if (!res || !res.result || !res.result[field] || !res.result[field][token]) {
    return {};
  }

  return cloneDeep(res.result[field][token]);
}

export async function getBalances(token: string) {
  const base16 = fromBech32Address(token);
  const field = "balances";
  const res = await blockchain.getSmartContractSubState(base16, field);

  if (res.error) {
    throw new Error(res.error.message);
  }

  if (!res || !res.result || !res.result[field]) {
    return {};
  }

  const balances = cloneDeep(res.result[field]);
  let poolAmount = new BN("0");
  let contribution = new BN("0");
  let zwapBalance = {};

  try {
    zwapBalance = await getDEXbalance(String(base16).toLowerCase());
    contribution = await getDEXpools(String(base16).toLowerCase());

    for (const iterator of Object.values(zwapBalance)) {
      if (typeof iterator === "string") {
        const v = new BN(iterator);
        poolAmount = poolAmount.add(v);
      }
    }
  } catch {
    //
  }

  for (const key in zwapBalance) {
    if (key in balances) {
      const userContributionbalance = new BN(zwapBalance[key]);
      const contributionPercentage = userContributionbalance
        .mul(_100)
        .div(poolAmount);

      if (_zero.eq(contributionPercentage)) {
        continue;
      }

      const userValue = contribution.mul(contributionPercentage).div(_100);
      const currentBalance = new BN(balances[key]);

      balances[key] = currentBalance.add(userValue).toString();
    }
  }

  return balances;
}

export async function getBalance(token: string, address: string) {
  address = String(address).toLowerCase();
  const base16 = fromBech32Address(token);
  const field = "balances";
  const res = await blockchain.getSmartContractSubState(base16, field, [
    address,
  ]);

  if (res.error) {
    throw new Error(res.error.message);
  }

  if (res && res.result && res.result[field] && res.result[field][address]) {
    return res.result[field][address];
  }

  return "0";
}
