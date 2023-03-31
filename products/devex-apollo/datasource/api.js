/* 
  API for Zilliqa network
  
  Available async functions:
  1) getLatestTxBlock(): Number
  2) getTxBlock(blockNum: Number): TxBlockObj
  3) getTxnBodiesByTxBlock(blockNum: Number): Array<TransactionObj>
  4) isContractAddr(addr: String): Boolean
*/

import fetch from "node-fetch";
import { stripHexPrefix } from "../util.js";

import zilp from "@zilliqa-js/zilliqa";
const { Zilliqa } = zilp;

class Api {
  constructor(networkUrl = "https://api.zilliqa.com/") {
    this.networkUrl = networkUrl;
    this.Zilliqa = new Zilliqa(this.networkUrl);
  }

  // Get latest tx block number
  async getLatestTxBlock() {
    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "1",
        jsonrpc: "2.0",
        method: "GetNumTxBlocks",
        params: [""],
      }),
    });
    const parsedRes = await response.json();
    return parsedRes.result;
  }

  // Get tx block with transactions
  async getTxBlock(blockNum) {
    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "1",
        jsonrpc: "2.0",
        method: "GetTxBlock",
        params: [`${blockNum}`],
      }),
    });
    const parsedRes = await response.json();
    return parsedRes.result;
  }

  async checkIfContracts(txns) {
    const data = txns.map((txn) => {
      return {
        id: "1",
        jsonrpc: "2.0",
        method: "GetSmartContractInit",
        params: [`${stripHexPrefix(txn.toAddr)}`],
      };
    });

    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    const parsedRes = await response.json();

    return txns.map((txn, index) => {
      return {
        ...txn,
        type:
          txn.type !== "contract-creation" && !parsedRes[index].error
            ? "contract-call"
            : "payment",
      };
    });
  }

  async getTxBlocks(blocks) {
    const data = blocks.map((block) => {
      return {
        id: "1",
        jsonrpc: "2.0",
        method: "GetTxBlock",
        params: [`${block}`],
      };
    });

    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    const parsedRes = await response.json();
    return parsedRes.map((item) => {
      return item.result;
    });
  }
  // Get transaction bodies by tx block
  async getTxnBodiesByTxBlock(blockNum) {
    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "1",
        jsonrpc: "2.0",
        method: "GetTxnBodiesForTxBlock",
        params: [`${blockNum}`],
      }),
    });
    const parsedRes = await response.json();
    return parsedRes.result;
  }

  // Get transaction bodies by tx block
  async getTxnBodiesByTxBlocks(blocks) {
    const data = blocks.map((block) => {
      return {
        id: "1",
        jsonrpc: "2.0",
        method: "GetTxnBodiesForTxBlock",
        params: [`${block.header.BlockNum}`],
      };
    });
    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    const parsedRes = await response.json();
    const reducedTxs = parsedRes
      .map((txresult) => {
        return txresult.result;
      })
      .flat();

    return reducedTxs;
  }

  /* Until we find a better way to differentiate an account address from a smart contract address, we will differentiate based
  on the the response error message if any */
  async isContractAddr(addr) {
    const response = await fetch(this.networkUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: "1",
        jsonrpc: "2.0",
        method: "GetSmartContractInit",
        params: [`${stripHexPrefix(addr)}`],
      }),
    });
    const parsedRes = await response.json();
    if (!parsedRes.error) return true;
    else if (parsedRes.error.message === "Address not contract address")
      return false;
    else return false;
  }
}

export default Api;
