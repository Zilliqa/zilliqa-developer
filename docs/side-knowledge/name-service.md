---
tags:
  - Name
  - Service
  - Domain
  - Unstoppable
---

# Zilliqa name service

Developers may choose to integrate with a Name Service which maps a users Zilliqa Domain (testdomain.zil) to a Bech32 address(0xexample), when funds are sent to testdomain.zil - the function hashes the domain and looks it up on the Unstoppable Domain registry contract. Once the function has a result for a user domain, a key is used to call the the Unstoppable Domain resolver contract to send to via an API call. Developers may choose to implement this lookup feature to support ZNS in their application.

## Example domain call

This test function queries the unstoppable domain contract state for a user domain.

| Test domains     | Expected result             |
|------------------|-----------------------------|
| brad.zil         | resolves without any errors |
| johnnyjumper.zil | domain has no BTC record    |
| unregistered.zil | domain is not registered    |
| paulalcock.zil   | domain is not configured    |

```js
const ZILLIQA_API = "https://api.zilliqa.com/";
const UD_REGISTRY_CONTRACT_ADDRESS = "9611c53BE6d1b32058b2747bdeCECed7e1216793";
const fetch = (...args) => import('node-fetch').then(({default: fetch}) => fetch(...args));
const sha256 = require('js-sha256');

function namehash(name) {
  const hashArray = hash(name);
  return arrayToHex(hashArray);
}

function hash(name) {

  if (!name) {
      return new Uint8Array(32);
  }
  const [label, ...remainder] = name.split('.');
  const labelHash = sha256.array(label);
  const remainderHash = hash(remainder.join('.'));
  return sha256.array(new Uint8Array([...remainderHash, ...labelHash]));
}

function arrayToHex(arr) {
  return '0x' + Array.prototype.map.call(arr, x => ('00' + x.toString(16)).slice(-2)).join('');
}

async function fetchZilliqa(params) {
  const body = {
    method: "GetSmartContractSubState",
    id: "1",
    jsonrpc: "2.0",
    params
  };

  return await fetch(ZILLIQA_API, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(body),
  }).then(res => res.json());
}

function displayError(message, cleanDom) {
    console.log(message)
  return ;
}

function displayResolution(resolution) {
  const {
    ownerAddress,
    resolverAddress,
    records
  } = resolution;

  console.log(`ownerAddress: ${ownerAddress}`);
  console.log(`resolverAddress: ${resolverAddress}`);

  Object.entries(records).map(([key, value]) => {
    console.log(`${key} : ${value}`);
  });
}

async function ResolveZilDomain(userInput) {
  console.log(data)
  return data;
}

async function getAddressFromDomain(user_address)
{
  if (!userInput.endsWith(".zil")) {
    return {'error' : 'domain not registered'};
  }

  const token = namehash(userInput);
  const registryState =
    await fetchZilliqa(['0x9611c53BE6d1b32058b2747bdeCECed7e1216793', "records", [token]]);

  if (registryState.result == null) {
    console.log('error :  domain not registered')
  }

  const [ownerAddress, resolverAddress] = 
    registryState.result.records[token].arguments;
  
  if (resolverAddress === "0x0000000000000000000000000000000000000000") {
    console.log('error :  resolver not configured')
  }

  const recordResponse = await fetchZilliqa([
    resolverAddress.replace("0x", ""),
    "records",
    []
  ]);

  data = {
    ownerAddress,
    resolverAddress,
    records: recordResponse.result.records
  };

  if(typeof(data.records) != "undefined")
  {
    console.log(JSON.stringify(data.records)
  }
  else
  {
    console.log('error : domain registered, not configured')
  }
}
```

## Example Response

```js
{
  "crypto.BCH.address": "qrq4sk49ayvepqz7j7ep8x4km2qp8lauvcnzhveyu6",
  "crypto.BTC.address": "1EVt92qQnaLDcmVFtHivRJaunG2mf2C3mB",
  "crypto.DASH.address": "XnixreEBqFuSLnDSLNbfqMH1GsZk7cgW4j",
  "crypto.ETH.address": "0x45b31e01AA6f42F0549aD482BE81635ED3149abb",
  "crypto.LTC.address": "LetmswTW3b7dgJ46mXuiXMUY17XbK29UmL",
  "crypto.XMR.address": "447d7TVFkoQ57k3jm3wGKoEAkfEym59mK96Xw5yWamDNFGaLKW5wL2qK5RMTDKGSvYfQYVN7dLSrLdkwtKH3hwbSCQCu26d",
  "crypto.ZEC.address": "t1h7ttmQvWCSH1wfrcmvT4mZJfGw2DgCSqV",
  "crypto.ZIL.address": "zil1yu5u4hegy9v3xgluweg4en54zm8f8auwxu0xxj",
  "ipfs.html.value": "QmVaAtQbi3EtsfpKoLzALm6vXphdi2KjMgxEDKeGg6wHuK",
  "ipfs.redirect_domain.value": "www.unstoppabledomains.com"
}
```

## Further reading

[Resolution Libraries - Unstoppable Domains](https://docs.unstoppabledomains.com/send-and-receive-crypto-payments/resolution-libraries)

[Resolving .zil domain with direct call](https://docs.unstoppabledomains.com/send-and-receive-crypto-payments/direct-blockchain-calls/resolve-.zil-without-libraries)

[How to resolve .zil domain names - Unstoppable Domains](https://medium.com/unstoppabledomains/how-to-resolve-zil-domain-names-f43da8fe37a9)

[Unstoppable Domains Registry Contract](https://viewblock.io/zilliqa/address/zil1jcgu2wlx6xejqk9jw3aaankw6lsjzeunx2j0jz?tab=state)

[Unstoppable Domains ProxyBestower Contract](https://viewblock.io/zilliqa/address/zil15yw7wej02h6mm7z5f2dvwytfr5qn0z6vlwqug6?tab=state)