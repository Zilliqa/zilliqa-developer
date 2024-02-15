import { Router } from "express";
import fromentries from "object.fromentries";
import spaces from "@snapshot-labs/snapshot-spaces";
import { Message } from "../models";

export const spacesRouter = Router();

spacesRouter.get("/spaces/:key?", (req, res) => {
  const { key } = req.params;

  return res.status(201).json(key ? spaces[key] : spaces);
});

spacesRouter.get("/:space/proposals", async (req, res) => {
  const { space } = req.params;
  const messages = await Message.findAll({
    where: {
      space,
      type: "proposal",
    },
    order: [["timestamp", "DESC"]],
  });
  const spaces = messages.map((message) => {
    return [
      message.id,
      {
        address: message.address,
        msg: {
          version: message.version,
          timestamp: String(message.timestamp),
          token: message.token,
          type: message.type,
          payload: JSON.parse(message.payload),
        },
        sig: message.sig,
        authorIpfsHash: message.author_ipfs_hash,
      },
    ];
  });

  return res.status(201).json(fromentries(spaces));
});

spacesRouter.get("/:space/proposal/:id", async (req, res) => {
  const { space, id } = req.params;
  const messages = await Message.findAll({
    where: {
      space,
      proposal_id: id,
      type: "vote",
    },
    order: [["timestamp", "DESC"]],
  });
  const spaces = messages.map((message) => {
    return [
      message.address,
      {
        address: message.address,
        msg: {
          version: message.version,
          timestamp: message.timestamp.toString(),
          token: message.token,
          type: message.type,
          payload: JSON.parse(message.payload),
        },
        sig: message.sig,
        authorIpfsHash: message.author_ipfs_hash,
      },
    ];
  });

  return res.status(201).json(fromentries(spaces));
});
