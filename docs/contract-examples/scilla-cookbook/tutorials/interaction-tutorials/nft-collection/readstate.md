---
sidebar_position: 4
---

# Fetching ZRC-6 state

Taking the example for [fetchState.js script](https://github.com/Zilliqa/Zilliqa-JavaScript-Library-Examples/blob/master/node/fetchState.js) we can extend this to fetch the state of our NFT contract to perform particular bits of logic relating to the NFT contract.

Firstly, we will need some data in the contract state to be able to read anything from it.

## Amendments made

```getStaticValue``` and ```getValueFromMapKey``` are two simple functions that return the result of a ```zilliqa.blockchain.getSmartContractSubState``` call. ```getStaticValue``` is concerned of state fields when there is only one value, like a ```Uint256``` or a ```ByStr20```, whereas ```getValueFromMapKey``` needs the key to lookup in the map to return the value result.

By checking the contract state, we can determine if a particular token is using token_uri or base_uri, both, or none.

```js
const axios = require("axios");

const { Zilliqa } = require("@zilliqa-js/zilliqa");
const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");

// Standard fungible token deployed on devnet
const nftContract = "0x18d1f737c1a1102cca966bf82dfe459e35fbd524";
const userAddress = "0xcb47d4ebc3bb5adae03b7a9c17e45a718909ce78";

async function getNFTContractState(nft_contract, token_id) {
  const balance_row = await getValueFromMapKey(nftContract, "balances", userAddress);
  const token_owner_row = await getValueFromMapKey(nftContract, "token_owners", token_id);
  const base_uri = await getStaticValue(nftContract, "base_uri");
  const turi_tokenuri_set = await getValueFromMapKey(
    nftContract,
    "token_uris",
    String(token_id)
  );
  const turi_baseuri_set = await getValueFromMapKey(
    nftContract,
    "token_uris",
    String(token_id)
  );

//is token_uri set?
  // False =>
    // is base_uri set?
      // True =>
        // use base_uri
      // False =>
        // neither are set
  // True =>
    // use token_uri

  // is token_uri set for token 2?
  turi_tokenuri_set !== undefined || turi_tokenuri_set !== null
    ? console.log(`token ${token_id} has a token_uri, and a base API is set`)
    : process.abort(`token ${token_id} exception`);

  // is base_uri set for token 12?
  turi_baseuri_set === undefined || turi_baseuri_set === null
    ? process.abort(`token ${token_id} exception`)
    : console.log(`token ${token_id} has no token_uri, as it uses a base API`);

  // is token_uri && base_uri both not set?
  turi_tokenuri_set !== undefined ||
  (turi_tokenuri_set !== null && turi_baseuri_set === undefined) ||
  turi_baseuri_set === null
    ? console.error(`token ${token_id} is valid`)
    : console.error(
        `token ${token_id} has neither a base_uri or token_URI set`
      );
}
async function getStaticValue(nftContract, contract_state_field) {
  const chainResponse = await zilliqa.blockchain.getSmartContractSubState(
    nftContract,
    contract_state_field
  );
  console.log(
    `for call ${contract_state_field} : chain response, ${JSON.stringify(
      chainResponse.result
    )}`
  );
  return JSON.stringify(chainResponse.result);
}

async function getValueFromMapKey(nftContract, contract_state_field, map_key) {
  const chainResponse = await zilliqa.blockchain.getSmartContractSubState(
    nftContract,
    contract_state_field,
    [map_key]
  );
  console.log(
    `for call ${contract_state_field} : chain response, ${JSON.stringify(
      chainResponse.result
    )}`
  );
  return JSON.stringify(chainResponse.result);
}

getNFTContractState(nftContract, 2);
getNFTContractState(nftContract, 12);
```
