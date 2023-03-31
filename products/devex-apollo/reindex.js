import apollo from "apollo-server-express";
import cors from "cors";
import express from "express";
import mongoose from "mongoose";
import { range } from "./util.js";

import GQLSchema from "./gql/schema2.js";
import Api from "./datasource/api.js";
import config from "./config/config.js";

import {
  txBlockReducer,
  txnReducer,
  transitionReducer,
} from "./mongodb/reducer.js";
import { TxBlockModel, TxnModel, TransitionModel } from "./mongodb/model.js";

mongoose.connect(config.dbUrl, { ...config.mongooseOpts });

let connection = mongoose.connection;

connection.once("open", function () {
  console.log("MongoDB database connection established successfully");
  TxnModel.collection.reIndex((finished) => {
    console.log("finished re indexing TxnModel");
  });
  TxBlockModel.collection.reIndex((finished) => {
    console.log("finished re indexing TxBlockModel");
  });

  TxnModel.collection
    .getIndexes({ full: true })
    .then((indexes) => {
      console.log("TxnModel indexes:", indexes);
    })
    .catch(console.error);

  TxBlockModel.collection
    .getIndexes({ full: true })
    .then((indexes) => {
      console.log("TxBlockModel indexes:", indexes);
    })
    .catch(console.error);
});
