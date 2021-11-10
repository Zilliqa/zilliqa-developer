import express from 'express'
import cors from 'cors'
import dayjs from 'dayjs';
import fs from 'fs-extra';
import { BN, units } from '@zilliqa-js/util';
import dotenv from 'dotenv'

dotenv.config()

import { runBackend } from './backend.js'

const app = express()
const port = process.env.PORT
app.use(cors())

const groupBy = function (xs, key) {
    return xs.reduce(function (rv, x) {
        (rv[x[key]] = rv[x[key]] || []).push(x);
        return rv;
    }, {});
};

const initServ = async () => {
    await fs.ensureFileSync('./transactions.json');
    await fs.ensureFileSync('./viewblock-1W.json');
    await fs.ensureFileSync('./viewblock-2Y.json');
}

app.get('/daily-transactions', (req, res) => {

    const txsraw = fs.readJsonSync('./transactions.json');
    const txs = txsraw.transactions.map(item => { return { ...item, date_formatted: dayjs(parseInt(item.timestamp / 1000)).format("YYYY-MM-DD") } });

    const grouped = groupBy(txs, 'date_formatted');

    const details = [];

    for (const [key, value] of Object.entries(grouped)) {
        details.push({ time: key, value: value.length });
    }

    res.json(details);
})

app.get('/zils-burnt', (req, res) => {
    const txsraw = fs.readJsonSync('./transactions.json');
    const txs = txsraw.transactions.map(item => { return { ...item, date_formatted: dayjs(parseInt(item.timestamp / 1000)).format("YYYY-MM-DD") } });

    const grouped = groupBy(txs, 'date_formatted');

    const details = [];
    for (const [key, value] of Object.entries(grouped)) {
        const zils_burnt = value.reduce((pv, cv) => {
            return pv + (parseInt(cv.gasPrice) * parseInt(cv.receipt.cumulative_gas))
        }, 0);

        details.push({ time: key, value: units.fromQa(new BN(zils_burnt.toString()), units.Units.Zil) });
    }
    res.json(details);
})

app.get('/cumulative-value', (req, res) => {
    res.send('Hello World!')
})

app.get('/total-addresses', (req, res) => {
    const vballraw = fs.readJsonSync('./viewblock-2Y.json');

    const vball = vballraw.timeData.map(item => { return { ...item, date_formatted: dayjs(item.timestamp).format("YYYY-MM-DD") } });

    const group = groupBy(vball, 'date_formatted');

    const details = [];
    for (const [key, value] of Object.entries(group)) {
        const newAddresses = value.reduce((pv, cv) => {
            return pv + cv.growthCount
        }, 0);

        details.push({ time: key, value: newAddresses });
    }
    const total = details.reduce((pv, cv) => pv + cv.value, 0)
    res.json({ 'address_count': total });
})

app.get('/new-addresses', (req, res) => {
    const vb1wraw = fs.readJsonSync('./viewblock-1W.json');

    const vb1w = vb1wraw.timeData.map(item => { return { ...item, date_formatted: dayjs(item.timestamp).format("YYYY-MM-DD") } });

    const group = groupBy(vb1w, 'date_formatted');

    const details = [];
    for (const [key, value] of Object.entries(group)) {
        const newAddresses = value.reduce((pv, cv) => {
            return pv + cv.growthCount
        }, 0);

        details.push({ time: key, value: newAddresses });
    }
    res.json(details);
})

app.get('/token/:symbol', (req, res) => {
    try {
        const tokenraw = fs.readJsonSync(`./${req.params.symbol}.json`);

        res.json({
            ...tokenraw,
            totalSupplyFormatted: (Number(tokenraw.totalSupply) / Math.pow(10, tokenraw.decimals)),
            totalValue: (Number(tokenraw.totalSupply) / Math.pow(10, tokenraw.decimals)) * tokenraw.coingeckoValue
        });
    } catch (error) {
        res.json({ error: true, message: `There has been an error trying to read ${req.params.symbol} token file.`, rawMessage: error.message })
    }
})

app.listen(port, () => {
    runBackend();
    initServ();

    setInterval(() => {
        runBackend();
    }, 3_600_000);
    console.log(`Express app listening at http://localhost:${port}`)
})