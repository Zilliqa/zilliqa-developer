const express = require('express');
const bodyParser = require("body-parser");
const { BN, Long, bytes, units } = require('@zilliqa-js/util');
const { Zilliqa } = require('@zilliqa-js/zilliqa');

require('dotenv').config()

const {
    toBech32Address,
    getAddressFromPrivateKey,
} = require('@zilliqa-js/crypto');

const chainId = 333; // chainId of the developer testnet
const msgVersion = 1; // current msgVersion
const VERSION = bytes.pack(chainId, msgVersion);

const zilliqa = new Zilliqa(process.env.ISOLATED_URL);

zilliqa.wallet.addByPrivateKey(process.env.OWNER_PRIVATEKEY);

const app = express();

app.use(bodyParser.urlencoded({ extended: false }));
app.use(bodyParser.json());

const port = 5556;

app.post('/', (req, res) => {
    const address = requesy.body.address;


});

app.listen(port, () => console.log(`Faucet listening on port ${port}!`))