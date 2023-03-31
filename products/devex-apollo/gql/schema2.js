import mongoose from "mongoose";
import pkg from "graphql-compose-mongoose";
const { composeMongoose } = pkg;

import pkg2 from "graphql-compose";
const { schemaComposer } = pkg2;

import { TxnModel, TransitionModel } from "../mongodb/model.js";
import { stripHexPrefix, addHexPrefix } from "../util.js";

const customizationOptions = {}; // left it empty for simplicity, described below
const TxTC = composeMongoose(TxnModel, customizationOptions);
const TransitionTC = composeMongoose(TransitionModel, customizationOptions);

TxTC.addResolver({
  name: "findByCustomId",
  type: [TxTC],
  args: { customId: "String!" },
  resolve: async ({ args, context }) => {
    const { customId } = args;
    const {
      models: { TxnModel },
    } = context;
    return await TxnModel.find({ customId: customId }).maxTime(30000);
  },
});

schemaComposer.Query.addFields({
  txFindByCustomId: TxTC.getResolver("findByCustomId"),
  txCount: TxTC.mongooseResolvers.count(),
  transactions: TxTC.mongooseResolvers.findMany(),
  transitions: TransitionTC.mongooseResolvers.findMany(),
  txConnection: TxTC.mongooseResolvers.connection(),
  txPagination: TxTC.mongooseResolvers.pagination(),
});

const graphqlSchema = schemaComposer.buildSchema();
export default graphqlSchema;
