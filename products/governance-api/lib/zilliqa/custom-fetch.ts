// import { cloneDeep } from 'lodash';
import BN from "bn.js";
import { RPCMethod } from "@zilliqa-js/core";

export enum TokenFields {
  TotalSupply = "total_supply",
  Balances = "balances",
}
export enum DexFields {
  ZWAPPools = "pools",
  ZWAPBalances = "balances",
  XCADbalances = "xbalances",
  XCADPools = "xpools",
}

export class blockchain {
  private _http = `https://api.zilliqa.com/`;
  private _zilswap = "459cb2d3baf7e61cfbd5fe362f289ae92b2babb0";
  private _xcad = "1fb1a4fd7ba94b1617641d6022ba48cafa77eef0";
  private _xcadToken = "0x153feaddc48871108e286de3304b9597c817b456";
  private _100 = new BN(100000);
  private _zero = new BN(0);

  public async getLiquidity(token: string, address: string) {
    const batch = [
      {
        method: RPCMethod.GetSmartContractSubState,
        params: [this._zilswap, DexFields.ZWAPPools, [token]],
        id: 1,
        jsonrpc: `2.0`,
      },
      {
        method: RPCMethod.GetSmartContractSubState,
        params: [this._zilswap, DexFields.ZWAPBalances, [token]],
        id: 1,
        jsonrpc: `2.0`,
      },
      {
        method: RPCMethod.GetSmartContractSubState,
        params: [this._xcad, DexFields.XCADPools, []],
        id: 1,
        jsonrpc: `2.0`,
      },
      {
        method: RPCMethod.GetSmartContractSubState,
        params: [
          this._xcad,
          DexFields.XCADbalances,
          [`${this._xcadToken},${token}`],
        ],
        id: 1,
        jsonrpc: `2.0`,
      },
      {
        method: RPCMethod.GetSmartContractSubState,
        params: [this._toHex(token), TokenFields.Balances, []],
        id: 1,
        jsonrpc: `2.0`,
      },
      {
        method: RPCMethod.GetSmartContractSubState,
        params: [this._toHex(token), TokenFields.TotalSupply, []],
        id: 1,
        jsonrpc: `2.0`,
      },
    ];
    const res = await this._send(batch);
    let tokenBalances = res[4]["result"][TokenFields.Balances];
    const totalSupply = res[5]["result"][TokenFields.TotalSupply];

    tokenBalances = this._parseZilSwap(res, token, tokenBalances);
    tokenBalances = this._parseXcad(res, token, tokenBalances);

    const userBalance = tokenBalances[address];

    return {
      balances: tokenBalances,
      totalSupply,
      userBalance,
    };
  }

  private _parseXcad(res: object[], token: string, tokenBalances: object) {
    try {
      const xBalance =
        res[3]["result"][DexFields.XCADbalances][`${this._xcadToken},${token}`][
          this._xcadToken
        ];
      const [, , , tokenReserve] =
        res[2]["result"][DexFields.XCADPools][`${this._xcadToken},${token}`][
          "arguments"
        ];

      let poolAmount = new BN("0");
      let contribution = new BN(tokenReserve);

      for (const iterator of Object.values(xBalance)) {
        if (typeof iterator === "string") {
          const v = new BN(iterator);
          poolAmount = poolAmount.add(v);
        }
      }

      for (const key in xBalance) {
        if (key in tokenBalances) {
          const userContributionbalance = new BN(xBalance[key]);
          const contributionPercentage = userContributionbalance
            .mul(this._100)
            .div(poolAmount);

          if (this._zero.eq(contributionPercentage)) {
            continue;
          }

          const userValue = contribution
            .mul(contributionPercentage)
            .div(this._100);
          const currentBalance = new BN(tokenBalances[key]);

          tokenBalances[key] = currentBalance.add(userValue).toString();
        }
      }
    } catch (err) {
      console.log("parse-xcad", err);
    }

    return tokenBalances;
  }

  private _parseZilSwap(res: object[], token: string, tokenBalances: object) {
    try {
      const [, tokenReserve] =
        res[0]["result"][DexFields.ZWAPPools][token]["arguments"];
      const zBalance = res[1]["result"][DexFields.ZWAPBalances][token];

      let poolAmount = new BN("0");
      let contribution = new BN(tokenReserve);

      for (const iterator of Object.values(zBalance)) {
        if (typeof iterator === "string") {
          const v = new BN(iterator);
          poolAmount = poolAmount.add(v);
        }
      }

      for (const key in zBalance) {
        if (key in tokenBalances) {
          const userContributionbalance = new BN(zBalance[key]);
          const contributionPercentage = userContributionbalance
            .mul(this._100)
            .div(poolAmount);

          if (this._zero.eq(contributionPercentage)) {
            continue;
          }

          const userValue = contribution
            .mul(contributionPercentage)
            .div(this._100);
          const currentBalance = new BN(tokenBalances[key]);

          tokenBalances[key] = currentBalance.add(userValue).toString();
        }
      }
    } catch (err) {
      console.log("zilswap-parse", err);
    }

    return tokenBalances;
  }

  private _toHex(address: string) {
    return String(address).replace("0x", "").toLowerCase();
  }

  private async _send(batch: object[]) {
    const res = await fetch(this._http, {
      method: `POST`,
      headers: {
        "Content-Type": `application/json`,
      },
      body: JSON.stringify(batch),
    });
    return res.json();
  }
}

// new blockchain().getLiquidity('0xa845c1034cd077bd8d32be0447239c7e4be6cb21', '0x837Eb6850BB3A1172Eb94B557762e474A8e9Ac73'.toLowerCase());
