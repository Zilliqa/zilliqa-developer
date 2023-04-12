import apollo from "apollo-server-express";
import cors from "cors";
import express from "express";
import mongoose from "mongoose";
import { range } from "./util.js";

import GQLSchema from "./gql/schema2.js";
import Api from "./datasource/api.js";
import config from "./config/config.js";

import { txBlockReducer, txnReducer } from "./mongodb/reducer.js";
import { TxBlockModel, TxnModel } from "./mongodb/model.js";

const { ApolloServer } = apollo;

console.log("NODE_ENV: " + process.env.NODE_ENV);

const BLOCKS_PER_REQUEST = process.env.BLOCKS_PER_REQUEST;

// Set up apollo express server
const app = express();
app.use(cors());

const api = new Api(process.env.NETWORK_URL);

const server = new ApolloServer({
  schema: GQLSchema,
  context: {
    models: {
      TxBlockModel,
      TxnModel,
    },
    api: api,
  },
});

server.applyMiddleware({ app, path: "/" });

mongoose.connect(config.dbUrl, { ...config.mongooseOpts });

let connection = mongoose.connection;

const loadData = async (start, end) => {
  let blocksPerRequest = BLOCKS_PER_REQUEST;
  if (start > end) {
    if (start - blocksPerRequest < end) {
      blocksPerRequest = start - end;
    }
    console.log(
      `Trying to sync from ${start} to ${start - blocksPerRequest}...`
    );
    try {
      const blocksRange = [...range(start - blocksPerRequest, start)];

      const txBlocks = await api.getTxBlocks(blocksRange);

      const reducedBlocks = txBlocks.map((block) => txBlockReducer(block));

      const blocksWithTxs = reducedBlocks.filter(
        (block) => block.header.NumTxns !== 0
      );

      const txns = await api.getTxnBodiesByTxBlocks(blocksWithTxs);

      const reducedTxns = txns.map((txn) => txnReducer(txn));

      const contractsChecked = await api.checkIfContracts(reducedTxns);

      const finalTxns = contractsChecked.map((txn) => {
        const blockDetails = reducedBlocks.find((block) => {
          return parseInt(block.header.BlockNum) === txn.blockId;
        });

        return {
          ...txn,
          timestamp: parseInt(blockDetails.header.Timestamp),
        };
      });

      const finalBlocksAdded = await new Promise((resolve, reject) =>
        TxBlockModel.insertMany(
          reducedBlocks,
          { ordered: false },
          function (err, result) {
            if (err) {
              if (err.code === 11000) {
                resolve(err.result.result.insertedIds);
              } else {
                reject(err);
              }
            }
            resolve(result); // Otherwise resolve
          }
        )
      );

      const finalTxsAdded = await new Promise((resolve, reject) =>
        TxnModel.insertMany(
          finalTxns,
          { ordered: false },
          function (err, result) {
            if (err) {
              if (err.code === 11000) {
                resolve(err.result.result.insertedIds);
              } else {
                reject(err);
              }
            }
            resolve(result); // Otherwise resolve
          }
        )
      );

      console.log(
        `Added ${finalTxsAdded.length}/${finalTxns.length} txs and ${finalBlocksAdded.length}/${reducedBlocks.length} blocks.`
      );
    } catch (error) {
      if (error.code !== 11000) {
        // 11000 stands for duplicate entry
        console.log(error.message);
      }
    } finally {
      console.log(
        `Synced blocks from ${start} to ${start - blocksPerRequest}.`
      );
      if (start - blocksPerRequest > end) {
        setTimeout(() => {
          loadData(start - BLOCKS_PER_REQUEST, end);
        }, 5000);
      }
    }
  } else {
    console.log(`${end} is lower than ${start}`);
  }
};

connection.once("open", function () {
  console.log("MongoDB database connection established successfully");
  api.getLatestTxBlock().then((latestBlock) => {
    try {
      if (process.env.FAST_SYNC === false) {
        loadData(latestBlock - 1, 0);
      }

      setInterval(async () => {
        const latestBlockInNetwork = await api.getLatestTxBlock();
        TxBlockModel.findOne({ customId: { $lt: 20000000 } })
          .sort({ customId: -1 })
          .limit(1)
          .exec((err, res) => {
            const latestBlockInDB = res === null ? 0 : res.customId;
            console.log(
              `Blocks need to be synced from ${latestBlockInNetwork} to ${latestBlockInDB}`
            );
            loadData(latestBlockInNetwork, latestBlockInDB);
          });
      }, 60000);
    } catch (error) {
      console.error(error);
      return;
    }
  });
});

app.listen(5000, () => {
  console.log("🚀  Server ready at port 5000");
});
