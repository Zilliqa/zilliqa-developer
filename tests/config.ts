import { bytes, units } from "@zilliqa-js/util";
import { Long, BN } from "@zilliqa-js/util";

export const CONTAINER = process.env["CONTAINER"];

//export const API = `http://localhost:${process.env["PORT"]}`; // Zilliqa Isolated Server
export const API = 'https://xcad-isolated-server.zilliqa.com/'
export const CHAIN_ID = 222;
export const MSG_VERSION = 1;
export const VERSION = bytes.pack(CHAIN_ID, MSG_VERSION);
export const asyncNoop = async () => undefined;
export const CONTRACTS = {
  staking_contract: {
    path: "staking.scilla",
  },

  token0: {
    path: "zrc2.scilla",
    name: "token0",
    symbol: "token0",
    decimals: "12",
    init_supply: "100000000000000000000"
  },

  token1: {
    path: "zrc2.scilla",
    name: "token1",
    symbol: "token1",
    decimals: "12",
    init_supply: "100000000000000000000"
  },

  token2: {
    path: "zrc2.scilla",
    name: "token2",
    symbol: "token2",
    decimals: "12",
    init_supply: "100000000000000000000"
  }
};

export const GAS_LIMIT = Long.fromNumber(100000);
export const GAS_PRICE = units.toQa("2000", units.Units.Li);

export const TX_PARAMS = {
  version: VERSION,
  amount: new BN(0),
  gasPrice: GAS_PRICE,
  gasLimit: GAS_LIMIT,
};

export const FAUCET_PARAMS = {
  version: VERSION,
  amount: new BN(units.toQa("100000000", units.Units.Zil)),
  gasPrice: GAS_PRICE,
  gasLimit: Long.fromNumber(50),
};

export const STAKING_ERROR = {
  UserHasUnclaimedReward: -7,
};

export const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";