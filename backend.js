import { request, gql, GraphQLClient } from 'graphql-request'
import dayjs from 'dayjs'
import fs from 'fs-extra'
import dotenv from 'dotenv'
import fetch from 'node-fetch';

dotenv.config()

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const query = gql`
  query Transactions($page: Int, $perPage: Int) {
  txPagination(
    page: $page
    perPage: $perPage
    sort: TIMESTAMP_DESC
  ) {
    pageInfo {
      currentPage
      perPage
    }
    items {
      blockId
      timestamp
      amount
      gasPrice
      receipt {
        cumulative_gas
        success
        epoch_num
        accepted
      }
    }
  }
}
`

var now = dayjs().startOf('day').subtract(7, 'day').valueOf();
var transactions = [];

export const requestApollo = async (page) => {
  try {
    console.log(`APOLLO: requesting page ${page}`);

    const data = await request('https://devex-apollo.zilliqa.com', query, { page, perPage: 20000 });
    const lastItem = data.txPagination.items.at(-1);

    console.log(parseInt(lastItem.timestamp / 1000), now);
    console.log(data.txPagination.items.length);

    if (parseInt(lastItem.timestamp / 1000) > now) {
      transactions.push.apply(transactions, data.txPagination.items);
      sleep(2000)
      return requestApollo(page + 1);
    } else {
      console.log('requests successful for last 7 days.');
      return transactions;
    }
  } catch (error) {
    console.log(error.message);
    console.log('something hapened with apollo request:' + error.statusCode);
  }
}

export const requestViewblock = async (ago) => {
  const endpoint = 'https://api.viewblock.io/graphql'

  const graphQLClient = new GraphQLClient(endpoint, {
    headers: {
      'X-APIKEY': process.env.VIEWBLOCK_API_KEY
    },
  })

  const query = gql`
      query timeData($id: String!, $network: String, $ago: TimeAgo!) {
  timeData(id: $id, network: $network, ago: $ago) {
    timestamp
    creationCount
    tokenCreationCount
    txCount
    growthCount
    reward
    staked
    buffer
    tokens {
      hash
      decimals
      volume
      symbol
    }
    networkVolumeRune
    networkVolumeUSD
  }
}
    `

  const data = await graphQLClient.request(query, {
    "id": "zilliqa",
    "network": "mainnet",
    "ago": ago
  });

  fs.removeSync(`viewblock-${ago}.json`);
  fs.outputJsonSync(`viewblock-${ago}.json`, data);

  console.log(`viewblock data saved to viewblock-${ago}.json`);
}

export const requestCoingecko = async (id) => {
  console.log(`Requesting coingecko for ${id}`);
  const response = await fetch(`https://api.coingecko.com/api/v3/simple/price?ids=${id}&vs_currencies=usd`);
  const data = await response.json();

  return data[id].usd;
}

export const requestViewblockTokens = async (symbol, hash, coingeckoId) => {
  await fs.ensureFileSync(`./${symbol}.json`);

  const endpoint = 'https://api.viewblock.io/graphql'

  const graphQLClient = new GraphQLClient(endpoint, {
    headers: {
      'X-APIKEY': process.env.VIEWBLOCK_API_KEY
    },
  })

  const query = gql`
      query tokens($chain: String!, $network: String!, $symbol: String,  $page: Float) {
  tokens(chain: $chain, network: $network,  symbol: $symbol, page: $page) {
    docs {
      symbol
      totalSupply
      name
      hash
      decimals
      txCount
    }
  }
}`

  const data = await graphQLClient.request(query, {
    chain: "zilliqa",
    network: "mainnet",
    symbol: symbol,
    page: 1
  });

  const details = data.tokens.docs.find(item => item.hash === hash);

  const coingeckoValue = await requestCoingecko(coingeckoId);

  fs.removeSync(`${symbol}.json`);
  fs.outputJsonSync(`${symbol}.json`, { ...details, coingeckoId, coingeckoValue });

  console.log(`viewblock token data saved to ${symbol}.json`);
}

export const runBackend = async (statsService) => {
  now = dayjs().startOf('day').subtract(7, 'day').valueOf();
  transactions = [];

  await requestViewblock('2Y');
  await sleep(2000);
  await requestViewblock('1W');
  await requestApollo(1);

  await requestViewblockTokens('zETH', 'zil19j33tapjje2xzng7svslnsjjjgge930jx0w09v', 'ethereum');
  await sleep(2000);
  await requestViewblockTokens('zUSDT', 'zil1sxx29cshups269ahh5qjffyr58mxjv9ft78jqy', 'tether');
  await sleep(2000);
  await requestViewblockTokens('zWBTC', 'zil1wha8mzaxhm22dpm5cav2tepuldnr8kwkvmqtjq', 'wrapped-bitcoin');
  await sleep(2000);
  await requestViewblockTokens('XCAD', 'zil1z5l74hwy3pc3pr3gdh3nqju4jlyp0dzkhq2f5y', 'xcad-network');

  const filtered = transactions.filter(i => parseInt(i.timestamp / 1000) > now);

  const cumulative = fs.readJsonSync('cumulative.json');
  const cfiltered = transactions.filter(i => parseInt(i.timestamp / 1000) > cumulative.timestamp)
  const sumup = cfiltered.reduce((pv, cv) => {
    return BigInt(cv.amount) + BigInt(pv)
  }, BigInt(cumulative.value));

  fs.removeSync('cumulative.json');
  fs.outputJsonSync('cumulative.json', {
    timestamp: parseInt(cfiltered.at(-1).timestamp / 1000),
    value: (BigInt(cumulative.value) + BigInt(sumup)).toString(),
    tx_count: cfiltered.length + cumulative.tx_count
  });

  fs.removeSync('transactions.json');
  fs.outputJsonSync('transactions.json', { transactions: filtered });

  console.log(`${filtered.length} txs saved to transactions.json`);

  return true;
}