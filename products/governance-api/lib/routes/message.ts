import { Router } from "express";
import BN from "bn.js";
import spaces from "@snapshot-labs/snapshot-spaces";
import { verifySignature, pinJson } from "../utils";
import { verifyEVMSignature, zilliqaAddressFromEVMSignature } from "../utils/verify-evm-signature";
import { Message } from "../models";
import { blockchain } from "../zilliqa/custom-fetch";

import pkg from "../../package.json";

import { ErrorCodes } from "../config";
import { fromBech32Address } from "@zilliqa-js/zilliqa";
import { validation } from "@zilliqa-js/util";

export const message = Router();
const gZIL = "zil14pzuzq6v6pmmmrfjhczywguu0e97djepxt8g3e";
const blk = new blockchain();



const proposal = (res: any, msg: any) => {
  if (msg.type !== "proposal") {
    return null;
  }

  if (
    Object.keys(msg.payload).length !== 9 ||
    !msg.payload.choices ||
    msg.payload.choices.length < 2 ||
    !msg.payload.metadata
  ) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_PROPOSAL_FORMAT,
      error_description: "incorect proposal format",
    });
  }

  if (isNaN(msg.payload.snapshot) || Number(msg.payload.snapshot) === 0) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_PROPOSAL_FORMAT,
      error_description: "incorect snapshot blocknumber",
    });
  }

  if (!msg.payload.quorum || Number(msg.payload.quorum) > 100) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_QUORUM,
      error_description: "incorect quorum",
    });
  }

  if (
    !msg.payload.name ||
    msg.payload.name.length > 256 ||
    !msg.payload.body ||
    msg.payload.body.length > 4e4
  ) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_PROPOSAL_SIZE,
      error_description: "incorect proposal size",
    });
  }

  if (
    typeof msg.payload.metadata !== "object" ||
    JSON.stringify(msg.payload.metadata).length > 2e4
  ) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_PROPOSAL_METADATA,
      error_description: "incorect proposal metadata",
    });
  }

  if (
    !msg.payload.start ||
    // ts > msg.payload.start ||
    !msg.payload.end ||
    msg.payload.start >= msg.payload.end
  ) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_PROPOSAL_PERIOD,
      error_description: "incorect proposal period",
    });
  }
};
const vote = async (res: any, msg: any, ts: string, log: any) => {
  if (msg.type !== "vote") {
    return null;
  }

  if (
    Object.keys(msg.payload).length !== 3 ||
    !msg.payload.proposal ||
    !msg.payload.choice ||
    !msg.payload.metadata
  ) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_VOTE_FORMAT,
      error_description: "incorect vote format",
    });
  }

  if (
    typeof msg.payload.metadata !== "object" ||
    JSON.stringify(msg.payload.metadata).length > 1e4
  ) {
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_VOTE_METADATA,
      error_description: "incorect vote metadata",
    });
  }

  const proposal = await Message.findOne({
    where: {
      token: msg.token,
      author_ipfs_hash: msg.payload.proposal,
    },
  });

  if (!proposal) {
    log.error({ error_code: ErrorCodes.INCORRECT_PROPOSAL_FORMAT, token: msg.token }, "Proposal not found");
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_PROPOSAL_FORMAT,
      error_description: "incorect vote proposal",
    });
  }
  const payload = JSON.parse(proposal.payload);

  if (Number(ts) > Number(payload.end) || Number(payload.start) > Number(ts)) {
    log.error({ error_code: ErrorCodes.INCORRECT_VOTE_FORMAT, address: msg.address }, "Not in voting window");
    return res.status(400).json({
      code: ErrorCodes.INCORRECT_VOTE_FORMAT,
      error_description: "not in voting window",
    });
  }
};

message.post("/message", async (req, res) => {
  const log = (req as any).log;
  try {
    const body = req.body;
    const msg = JSON.parse(body.msg);
    const ts = (Date.now() / 1e3).toFixed();

    if (!body || !body.address || !body.msg || !body.sig) {
      log.error({ error_code: ErrorCodes.INCORRECT_DATA, address: body && body.address }, "incorrect message body");
      return res.status(400).json({
        code: ErrorCodes.INCORRECT_DATA,
        error_description: "incorect message body",
      });
    }

    const spaceKey = body.space;
    if (!spaceKey || !spaces[spaceKey]) {
      log.error({ error_code: ErrorCodes.UNKNOWN_SPACE, address: body && body.address }, "unknown space");
      return res.status(400).json({
        code: ErrorCodes.UNKNOWN_SPACE,
        error_description: "unknown space",
      });
    }

    if (spaces[spaceKey].token !== msg.token) {
      log.error({ error_code: ErrorCodes.UNKNOWN_SPACE, address: body && body.address }, "token does not match space");
      return res.status(400).json({
        code: ErrorCodes.UNKNOWN_SPACE,
        error_description: "token does not match space",
      });
    }

    msg.timestamp = Number(msg.timestamp);

    if (!msg.timestamp || isNaN(msg.timestamp) || msg.timestamp > ts + 30) {
      log.error({ error_code: ErrorCodes.INCORRECT_DATA, address: body && body.address }, "wrong timestamp");
      return res.status(400).json({
        code: ErrorCodes.INCORRECT_DATA,
        error_description: "wrong timestamp",
      });
    }

    if (!msg.version || msg.version !== pkg.version) {
      log.error({ error_code: ErrorCodes.INCORRECT_VER, address: body && body.address }, "incorrect version");
      return res.status(400).json({
        code: ErrorCodes.INCORRECT_VER,
        error_description: "incorrect version",
      });
    }

    if (!msg.type || !["proposal", "vote"].includes(msg.type)) {
      log.error({ error_code: ErrorCodes.INCORRECT_TYPE, address: body && body.address }, "incorrect type");
      return res.status(400).json({
        code: ErrorCodes.INCORRECT_TYPE,
        error_description: "incorrect type",
      });
    }

    // Normalise EVM addresses to always have the 0x prefix
    if (body.sigType === 'evm' && body.address && !body.address.startsWith('0x')) {
      body.address = '0x' + body.address;
    }

    try {
      let checked: boolean;
      if (body.sigType === 'evm') {
        checked = verifyEVMSignature(
          body.sig.message,
          body.sig.signature,
          body.address
        );
      } else {
        // Default to Schnorr for ZilPay and legacy submissions
        checked = verifySignature(
          body.sig.message,
          body.sig.publicKey,
          body.sig.signature,
          body.address
        );
      }
      if (!checked) throw new Error('signature mismatch');
    } catch (err) {
      log.error({ error_code: ErrorCodes.INCORRECT_SIGNATURE, address: body.address }, "Signature verification failed");
      return res.status(400).json({
        code: ErrorCodes.INCORRECT_SIGNATURE,
        error_description: "incorrect signature",
      });
    }

    log.info({ address: body.address, sigType: body.sigType || "schnorr" }, "Signature verified");

    // EVM users sign with their 0x (Keccak) address, but their gZIL/ZRC2 balances and space
    // membership are keyed by their Zilliqa (SHA256) address. Replace body.address with the
    // canonical Zilliqa address recovered from the signature so the gZIL gate, the pinned
    // voter-scoring snapshot, and the members/score checks all resolve against the user's real
    // Zilliqa identity. Applies to both proposals and votes (same handler).
    if (body.sigType === "evm") {
      body.address = zilliqaAddressFromEVMSignature(body.sig.message, body.sig.signature);
      log.info({ zilAddress: body.address }, "EVM identity normalized to Zilliqa address");
    }

    proposal(res, msg);
    await vote(res, msg, ts, log);

    const space = spaceKey;
    let authorIpfsRes: any | null = null;

    if (msg.type === "proposal") {
      const base16Token = fromBech32Address(msg.token).toLowerCase();
      const base16owner = validation.isBech32(body.address)
        ? fromBech32Address(body.address)
        : String(body.address).toLowerCase();
      const { balances, userBalance, totalSupply } = await blk.getLiquidity(
        base16Token,
        base16owner
      );

      log.info({ token: base16Token, address: base16owner, userBalance }, "Zilliqa liquidity fetched");

      const _balance = new BN(userBalance || "0");
      const _minGZIL = new BN("30000000000000000");

      if (msg.token == gZIL && _balance.lt(_minGZIL)) {
        log.error({ error_code: ErrorCodes.MIN_BALANCE_ERROR, balance: _balance.toString(), threshold: _minGZIL.toString() }, "Balance below minimum gZIL");
        return res.status(400).json({
          code: ErrorCodes.MIN_BALANCE_ERROR,
          error_description:
            "You require 30 $gZIL or more to submit a proposal.",
        });
      }

      authorIpfsRes = await pinJson({
        balances,
        totalSupply,
        address: body.address,
        msg: body.msg,
        sig: body.sig,
        version: "2",
      });
      await Message.create({
        space,
        token: msg.token,
        author_ipfs_hash: authorIpfsRes,
        address: body.address,
        version: msg.version,
        timestamp: msg.timestamp,
        type: "proposal",
        payload: JSON.stringify(msg.payload),
        sig: JSON.stringify(body.sig),
      });
      log.info({ type: msg.type, ipfsHash: authorIpfsRes, address: body.address }, "DB record created");
    }

    if (msg.type === "vote") {
      authorIpfsRes = await pinJson({
        address: body.address,
        msg: body.msg,
        sig: body.sig,
        version: "2",
      });
      await Message.create({
        space,
        token: msg.token,
        author_ipfs_hash: authorIpfsRes,
        address: body.address,
        version: msg.version,
        timestamp: msg.timestamp,
        type: "vote",
        proposal_id: msg.payload.proposal,
        payload: JSON.stringify(msg.payload),
        sig: JSON.stringify(body.sig),
      });
      log.info({ type: msg.type, ipfsHash: authorIpfsRes, address: body.address }, "DB record created");
    }

    log.info({ address: body.address, token: msg.token, type: msg.type, ipfsHash: authorIpfsRes }, "Message processed successfully");

    return res.json({ ipfsHash: authorIpfsRes });
  } catch (err) {
    log.error({ err, error_code: 500 }, "Unhandled error in message handler");
    return res.status(400).json({
      code: 500,
      error_description: err.message,
    });
  }
});
