import express from "express";
import * as bodyParser from "body-parser";
import errorhandler from "strong-error-handler";
import pinoHttp from "pino-http";
import router from "./routes";
import { logger } from "./logger";

export const app = express();

app.use(
  pinoHttp({
    logger,
    customLogLevel: (_req, res, _err) => {
      if (res.statusCode >= 400) return "error";
      return "info";
    },
  })
);

// middleware for parsing application/x-www-form-urlencoded
app.use(bodyParser.urlencoded({ extended: true }));

// middleware for json body parsing
app.use(bodyParser.json({ limit: "1mb" }));

// enable corse for all origins
app.use((req, res, next) => {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Expose-Headers", "x-total-count");
  res.header("Access-Control-Allow-Methods", "GET,PUT,POST,DELETE,PATCH");
  res.header("Access-Control-Allow-Headers", "Content-Type,authorization");

  next();
});

app.use("/api", router);

app.use(
  errorhandler({
    debug: process.env.ENV !== "prod",
    log: true,
  })
);
