import 'babel-polyfill';
import express from "express";
import bodyParser from "body-parser";
import fs from "fs-extra";
import { registerUser, deployFaucet, getState } from './faucet';

require('dotenv').config()

const FAUCET_PORT = process.env.FAUCET_PORT;

const app = express();

app.use(bodyParser.urlencoded({ extended: false }));
app.use(bodyParser.json());

app.post('/register-account', async (req, res) => {
    const address = req.body.address;

    try {
        const result = await registerUser(address);
        return res.send(result);
    } catch (error) {
        return res.send(error);
    }
});

app.get('/faucet-state', (req, res) => {
    try {
        const state = getState();

        return res.send(state);
    } catch (error) {
        return res.send(error);
    }
});

app.listen(FAUCET_PORT, () => console.log(`Faucet listening on port ${FAUCET_PORT}!`))

// Check if faucet contract exists, if not, deploy one.
if (!fs.existsSync('./faucet-state.json')) {
    console.log('Faucet state file does not exist. Deploying new contract...');

    deployFaucet();
} else {
    console.log('Faucet state:');
    const state = getState();
    console.log(state);
}

