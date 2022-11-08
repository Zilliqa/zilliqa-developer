require("dotenv").config();
var fs = require("fs");
var path = require("path");

module.exports = {
  // Network JSON-RPC API where contract needs to be deployed
  networkUrl: "",
  // Private Key of the account used to deploy.
  // PRIVATE KEY should never be hardcoded in any file.
  accountPrivateKey: process.env.ACCOUNT_PRIVATE_KEY,
  // Scilla file used for deployment
  contractFile: "hello-world.scilla",
  // Init params used in deployment transaction (can be replaced with a JSON Object)
  init: JSON.parse(
    fs.readFileSync(path.join(__dirname, "./init.json"), "utf8")
  ),
};
