import { request, gql, GraphQLClient } from 'graphql-request'
import dayjs from 'dayjs'
import fs from 'fs-extra'
import dotenv from 'dotenv'

dotenv.config()

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

const now = dayjs().startOf('day').subtract(7, 'day').valueOf();

const transactions = [];

export const requestApollo = async (page) => {
  console.log(`requesting page ${page}`);

  const data = await request('https://devex-apollo.zilliqa.com', query, { page, perPage: 50000 });

  const lastItem = data.txPagination.items.at(-1);

  console.log(parseInt(lastItem.timestamp / 1000), now);
  console.log(data.txPagination.items.length);

  if (parseInt(lastItem.timestamp / 1000) > now) {
    transactions.push.apply(transactions, data.txPagination.items);
    console.log(transactions.length);
    await requestApollo(page + 1);
  } else {
    console.log('requests successful for last 7 days.');
    return transactions;
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

  fs.writeJsonSync(`viewblock-${ago}.json`, data);

  console.log(`viewblock data saved to viewblock-${ago}.json`);
}

export const runBackend = async () => {
  await requestViewblock('2Y');
  await requestViewblock('1W');
  await requestApollo(1);

  const filtered = transactions.filter(i => parseInt(i.timestamp / 1000) > now);

  fs.writeJsonSync('transactions.json', { transactions: filtered });

  console.log(`${filtered.length} txs saved to transactions.json`);
}