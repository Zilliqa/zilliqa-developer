import mongoose from "mongoose";
import config from "./config/config.js";
import { TxBlockModel, TxnModel } from "./mongodb/model.js";

mongoose.connect(config.dbUrl, { ...config.mongooseOpts });

let connection = mongoose.connection;

connection.once("open", function () {
  console.log("MongoDB database connection established successfully");
  TxnModel.collection.reIndex((finished) => {
    console.log("finished re indexing TxnModel", finished);
  });
  TxBlockModel.collection.reIndex((finished) => {
    console.log("finished re indexing TxBlockModel", finished);
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
