import fs from "fs";

let config;

if (process.env.NODE_ENV === "dev") {
  config = {
    dbUrl: `mongodb://${process.env.DOCUMENTDB_USER}:${process.env.DOCUMENTDB_PASSWORD}@${process.env.DOCUMENTDB_HOST}:${process.env.DOCUMENTDB_PORT}/${process.env.DOCUMENTDB_DB}?authSource=admin&readPreference=primary&appname=MongoDB%20Compass%20Community&ssl=false`,
    mongooseOpts: {
      useUnifiedTopology: true,
      useNewUrlParser: true,
      useCreateIndex: true,
    },
  };
} else {
  let CA;
  try {
    CA = fs.readFileSync("./rds-combined-ca-bundle.pem");
  } catch (e) {
    console.log("Missing CA .pem in root");
  }

  const documentDbConf =
    "?ssl=true&ssl_ca_certs=rds-combined-ca-bundle.pem&replicaSet=rs0&readPreference=secondaryPreferred&retryWrites=false";

  config = {
    database: process.env.DOCUMENTDB_DB,
    dbUrl: `mongodb://${process.env.DOCUMENTDB_USER}:${process.env.DOCUMENTDB_PASSWORD}@${process.env.DOCUMENTDB_HOST}:${process.env.DOCUMENTDB_PORT}/${documentDbConf}`,
    mongooseOpts: {
      useUnifiedTopology: true,
      useNewUrlParser: true,
      useCreateIndex: true,
      sslCA: [CA],
    },
  };
}

export default config;
