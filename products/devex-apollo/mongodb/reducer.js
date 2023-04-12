// Reduces api results into models recognised by mongodb,
// at the same time doing some primary formatting

import { pubKeyToHex } from "../util.js";

export const txBlockReducer = (txBlock) => {
  return {
    ...txBlock,
    customId: parseInt(txBlock.header.BlockNum),
    txnHashes: txBlock.txnHashes,
  };
};

export const transitionReducer = async (txn) => {
  let transitions = false;
  if (txn.receipt.transitions) {
    transitions = txn.receipt.transitions.map((transition) => {
      const params = {};

      transition.msg.params.map((p) => {
        params[p.vname] = p.value;
        return true;
      });

      const msg = {
        _amount: transition.msg._amount,
        _recipient: transition.msg._recipient,
        _tag: transition.msg._tag,
        params: params,
      };

      return {
        accepted: transition.accepted,
        addr: transition.addr,
        depth: transition.depth,
        msg,
      };
    });
  }

  return transitions;
};

export const txnReducer = (txn) => {
  let type = "payment";

  if (txn.toAddr === "0000000000000000000000000000000000000000") {
    type = "contract-creation";
  }

  let transitions = [];

  if (txn.receipt.transitions) {
    transitions = txn.receipt.transitions.map((transition) => {
      const params = {};

      transition.msg.params.map((p) => {
        params[p.vname] = p.value;
        return true;
      });

      const msg = {
        _amount: transition.msg._amount,
        _recipient: transition.msg._recipient,
        _tag: transition.msg._tag,
        params: params,
      };

      return {
        accepted: transition.accepted,
        addr: transition.addr,
        depth: transition.depth,
        msg,
      };
    });
  }

  return {
    ...txn,
    customId: txn.ID,
    blockId: parseInt(txn.receipt.epoch_num),
    toAddr: `0x${txn.toAddr}`,
    fromAddr: `0x${pubKeyToHex(txn.senderPubKey)}`,
    type,
    transitions,
  };
};
