import graphqlCompose from "graphql-compose";
import graphqlComposeMongoose from "graphql-compose-mongoose";
import { TxnModel, TxBlockModel } from "../mongodb/model.js";
import { stripHexPrefix, addHexPrefix } from "../util.js";

const { schemaComposer } = graphqlCompose;
const { composeWithMongoose } = graphqlComposeMongoose;

const customizationOptions = {};
const TxnTC = composeWithMongoose(TxnModel, customizationOptions);
const TxBlockTC = composeWithMongoose(TxBlockModel, customizationOptions);

// Resolver to get txns for a TxBlock
TxBlockTC.addRelation("txns", {
  resolver: () => TxnTC.getResolver("findByTxBlock"),
  prepareArgs: {
    txBlockNum: (source) => {
      return source.customId.split("_")[1];
    },
  },
  projection: { customId: true },
});

// Resolver to find txBlock by customId
TxBlockTC.addResolver({
  name: "findByCustomId",
  type: [TxBlockTC],
  args: { customId: "String!" },
  resolve: async ({ args, context }) => {
    const { customId } = args;
    const {
      models: { TxBlockModel },
    } = context;
    return await TxBlockModel.find({ customId: customId });
  },
});

TxnTC.addResolver({
  name: "findByCustomId",
  type: [TxnTC],
  args: { customId: "String!" },
  resolve: async ({ args, context }) => {
    const { customId } = args;
    const {
      models: { TxnModel },
    } = context;
    return await TxnModel.find({ customId: customId });
  },
});

TxnTC.addResolver({
  name: "findByTxBlock",
  type: [TxnTC],
  args: { txBlockNum: "String!" },
  resolve: async ({ args, context }) => {
    const { txBlockNum } = args;
    const {
      models: { TxnModel },
    } = context;
    return await TxnModel.find({ "receipt.epoch_num": txBlockNum });
  },
});

TxnTC.addResolver({
  name: "findByAddr",
  type: [TxnTC],
  args: { addr: "String!", offset: "Int", limit: "Int" },
  resolve: async ({ args, context }) => {
    const { addr } = args;
    let { offset, limit } = args;
    offset = offset || 0;
    limit = limit || 20;
    const {
      models: { TxnModel },
    } = context;
    const totalCount = await TxnModel.countDocuments({
      $or: [
        { from: stripHexPrefix(addr) },
        { toAddr: stripHexPrefix(addr) },
        { "receipt.transitions.addr": addHexPrefix(addr) },
        { "receipt.transitions.msg._recipient": addHexPrefix(addr) },
      ],
    });
    const documents = await TxnModel.find({
      $or: [
        { from: stripHexPrefix(addr) },
        { toAddr: stripHexPrefix(addr) },
        { "receipt.transitions.addr": addHexPrefix(addr) },
        { "receipt.transitions.msg._recipient": addHexPrefix(addr) },
      ],
    })
      .skip(offset)
      .limit(limit);

    return {
      ...documents,
      totalCount,
    };
  },
});

schemaComposer.Query.addFields({
  txBlocks: TxBlockTC.getResolver("findMany"),
  txns: TxnTC.getResolver("findMany"),
  txBlocksByCustomId: TxBlockTC.getResolver("findByCustomId"),
  txnByCustomId: TxnTC.getResolver("findByCustomId"),
  txnsByAddr: TxnTC.mongooseResolvers.pagination(),
});

const GQLSchema = schemaComposer.buildSchema();

export default GQLSchema;
