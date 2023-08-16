let config;

if (process.env.NODE_ENV === "dev") {
  config = {
    dbUrl: `mongodb://${process.env.DOCUMENTDB_USER}:${process.env.DOCUMENTDB_PASSWORD}@${process.env.DOCUMENTDB_HOST}:${process.env.DOCUMENTDB_PORT}/${process.env.DOCUMENTDB_DB}?authSource=${process.env.DOCUMENTDB_DB}&readPreference=primary&appname=MongoDB%20Compass%20Community&ssl=false`,
    mongooseOpts: {
      useUnifiedTopology: true,
      useNewUrlParser: true,
      useCreateIndex: true,
    },
  };
} else {
  const documentDbConf =
    "?authSource=" + process.env.DOCUMENTDB_DB + "&replicaSet=rs0";

  config = {
    database: process.env.DOCUMENTDB_DB,
    dbUrl: `mongodb://${process.env.DOCUMENTDB_USER}:${process.env.DOCUMENTDB_PASSWORD}@${process.env.DOCUMENTDB_HOST}:${process.env.DOCUMENTDB_PORT}/${documentDbConf}`,
    mongooseOpts: {
      useUnifiedTopology: true,
      useNewUrlParser: true,
      useCreateIndex: true,
    },
  };
}

export default config;
