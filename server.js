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

class StatsService {

    constructor() {
        fs.ensureFileSync('./transactions.json');
        fs.ensureFileSync('./viewblock-1W.json');
        fs.ensureFileSync('./viewblock-2Y.json');
    }

    syncData = async () => {
        console.log('Started sync...');
        this.txsraw = fs.readJsonSync('./transactions.json');
        this.txs = this.txsraw.transactions.map(item => { return { ...item, date_formatted: dayjs(parseInt(item.timestamp / 1000)).format("YYYY-MM-DD") } });
        this.vballraw = fs.readJsonSync('./viewblock-2Y.json');
        this.vball = this.vballraw.timeData.map(item => { return { ...item, date_formatted: dayjs(item.timestamp).format("YYYY-MM-DD") } });
        this.vb1wraw = fs.readJsonSync('./viewblock-1W.json');
        this.vb1w = this.vb1wraw.timeData.map(item => { return { ...item, date_formatted: dayjs(item.timestamp).format("YYYY-MM-DD") } });
        this.cumulativeValue = fs.readJsonSync('./cumulative.json');

        await this.getDailyTransactions();
        await this.getNewAddresses();
        await this.getZilsBurnt();
        await this.getTotalAddresses();
        console.log('Service data sync successfull.');
        return true;
    }

    getDailyTransactions = async () => {
        const grouped = groupBy(this.txs, 'date_formatted');

        const details = [];

        for (const [key, value] of Object.entries(grouped)) {
            details.push({ time: key, value: value.length });
        }

        this.dailyTransactions = details;
        return details;
    }

    getZilsBurnt = async () => {
        const grouped = groupBy(this.txs, 'date_formatted');

        const details = [];

        for (const [key, value] of Object.entries(grouped)) {
            const zils_burnt = value.reduce((pv, cv) => {
                return pv + (parseInt(cv.gasPrice) * parseInt(cv.receipt.cumulative_gas))
            }, 0);

            details.push({ time: key, value: units.fromQa(new BN(zils_burnt.toString()), units.Units.Zil) });
        }
        this.zilsBurnt = details;
        return details;
    }

    getTotalAddresses = async () => {
        const group = groupBy(this.vball, 'date_formatted');

        const details = [];
        for (const [key, value] of Object.entries(group)) {
            const newAddresses = value.reduce((pv, cv) => {
                return pv + cv.growthCount
            }, 0);

            details.push({ time: key, value: newAddresses });
        }
        const total = details.reduce((pv, cv) => pv + cv.value, 0);
        this.totalAddresses = total;
        return total;
    }

    getNewAddresses = async () => {
        const group = groupBy(this.vb1w, 'date_formatted');

        const details = [];
        for (const [key, value] of Object.entries(group)) {
            const newAddresses = value.reduce((pv, cv) => {
                return pv + cv.growthCount
            }, 0);

            details.push({ time: key, value: newAddresses });
        }

        this.newAddresses = details;
        return details;
    }
}


const service = new StatsService();

app.get('/service-raw', (req, res) => {
    res.json(service);
})

app.get('/daily-transactions', (req, res) => {
    res.json(service.dailyTransactions);
})

app.get('/zils-burnt', (req, res) => {
    res.json(service.zilsBurnt);
})

app.get('/cumulative-value', (req, res) => {
    res.json(service.cumulativeValue);
})

app.get('/total-addresses', (req, res) => {
    res.json({ 'address_count': service.totalAddresses });
})

app.get('/new-addresses', (req, res) => {
    res.json(service.newAddresses);
})

app.get('/all-stats', (req, res) => {
    res.json({
        'daily_transactions': service.dailyTransactions,
        'zils_burnt': service.zilsBurnt,
        'cumulative_value': service.cumulativeValue,
        'total_addresses': { 'address_count': service.totalAddresses },
        'new_addresses': service.newAddresses,
        'cumulative_value': service.cumulativeValue
    })
});

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

app.listen(port, async () => {
    await runBackend(service);
    await service.syncData();

    setInterval(async () => {
        await runBackend(service);
        await service.syncData();
    }, 3_600_000);

    console.log(`Express app listening at http://localhost:${port}`)
})