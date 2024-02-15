import { Router } from "express";

import { message } from "./message";
import { spacesRouter } from "./spaces";
import relayer from "../zilliqa/relayer";

import pkg from "../../package.json";

const ENV = process.env.NODE_ENV || "development";

const router = Router();
const dev = ENV === "development" || ENV === "test";
let network = "mainnet";

if (dev) {
  network = "testnet";
}

router.use(message);
router.use(spacesRouter);

router.get("/", (_, res) => {
  return res.json({
    network,
    name: pkg.name,
    version: pkg.version,
    tag: "alpha",
    relayer: relayer ? relayer.bech32Address : null,
  });
});

export default router;
