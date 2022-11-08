---
sidebar_position: 4
---

# Creating metadata for a token

The below is an example of what metadata for a token should be presented. Read the document for [ZRC-7 nonfungible token metadata standardisation](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-7.md) for the full standard specification.

Take note of the metadata 'resource' being the previously uploaded resource file location.

```json {3,4,5}
{
  "name": "Creature #101",
  "resources": [
    { "uri": "ipfs://QmZILGa7zXUbixvYJpgkRkaSCYEBtSwgVtfzkoD3YkNsE1" }
  ],
  "attributes": [
    {
      "trait_type": "Background",
      "value": "Black"
    },
    {
      "trait_type": "Eyes",
      "value": "Big"
    },
    {
      "trait_type": "Mouth",
      "value": "Grin"
    }
  ]
}
```

## Making metadata public

Previously we uploaded our image resource, which is attached on our metadata json object on lines 3-5. This metadata json also needs to be uploaded so we can use the metadata link on the token.

By having the image wrapped inside the metadata, it allows us to pass both the image and some richer details of the token, instead of passing just the image alone.

:::tip
we can give our metadata mutability using base_uri or immutability using token_uri
:::

## When to use base URI

Now with our image and metadata uploaded, we can decide if we want an API to serve the metadata, or if we want to directly embed it onto the token.

The advantage of having an API serve the metadata is that a developer can control what the API is exposing. If files on IPFS need to change, the API can handle showing the correct file or data whilst hiding the data for tokens yet to be minted. In the case of when files need to change whilst embedded them onto the token, this is not possible as when a token_uri is set, the value is then immutable.

The developer can set base_uri to an API they control. An example is when ```initial_base_uri``` is set to ```www.api.example.com/``` and token_id is ```1```, ecosystem partners will query ```www.api.example.com/1``` when looking up details for token_id 1.

Once the minting phase is complete and if the metadata won't change in the future. Developers may choose to remove the API and change base_uri to some folder of decentralised storage. An IPFS folder is the recommended solution. The base_uri would become ```ipfs://QmeUhYpVsJUD2ekWnbSsvjQ2vSBWDbrC2PmnjdCswYTjDF/``` and the same logic of having the token_id become part of the query string. ```ipfs://QmeUhYpVsJUD2ekWnbSsvjQ2vSBWDbrC2PmnjdCswYTjDF/1```

## When to use token URI

An advantage of embedding the file onto the token is that it's quick and easy with no API required. But then is immutable for further changes. It is recommended to use a solution like decentralised storage to ensure your content isn't taken offline at any stage if opting for a token_uri implementation.

It's possible to have a ```base_uri``` set and have set a ```token_uri```  for a token_id. In this case, the standard says that ```token_uri``` overrides ```base_uri``` if both are present.

:::tip
Since the metadata is immutable and cannot be changed, it's recommended to use base URI to have control over changing the value if it becomes unavailable in the future.
:::

## Creating project level metadata

An NFT contract can also expose metadata at the collection level. Project level metadata can be found in the location ```<base_uri>metadata.json``` where base_uri looks like ```www.api.example.com/``` and the file ```metadata.json``` can be found in the root of that directory structure.

Some ecosystem partners may choose to present data about a collection from this section, as it reduces the amount of friction needed for an ecosystem to display content in a generic way.

```json
{
  "name": "Unique and Diverse Creatures",
  "description": "10,000 unique and diverse creatures living on the blockchain.",
  "external_url": "https://example.com/creature",
  "animation_url": "https://animation.example.com/creature"
}
```

## Metadata API scaffolding

:::tip
For our example we will use token_uri, so have no need for this API.
:::

```base_uri``` can be useful to use when you want to guard or change metadata frequently. The API is where you would decide on what to show depending on the current mint count state.

The below is a very simple express.js API.

It currently reads a parameter passed with the GET request called ```token_uri```. It then queries this against an NFT contract checking how many tokens have currently been minted. If then does a further check to see if the token_id has been burnt. The API responds with different HTTP codes depending on which case the logic can match against.

This can be extended to show metadata for correct requests to the API for a project level implementation.

```js
const express = require('express')
const app = express()
const port = 3000;

const { Zilliqa } = require('@zilliqa-js/zilliqa');
const zilliqa = new Zilliqa('https://dev-api.zilliqa.com');
const nftContract = '18d1f737c1a1102cca966bf82dfe459e35fbd524';

app.get('/', async (req, res) => {
    if(Number.isInteger(parseInt(req.query.token_id))) {
        const currentTokenID = await getCurrentTokenID()
        if(req.query.token_id <= currentTokenID) {
            const isBurnt = await isTokenIDBurnt(req.query.token_id)
            if(!isBurnt) {
                // fetch and return your current implementation of metadata.json
                res.send(`token_id sent is ${req.query.token_id} and this token_id hasn't been burnt, current token_id onchain is ${currentTokenID}`) 
            } 
            else {
                res.status(404).send(`requested token_id ${req.query.token_id} has been burnt!`);
            }
        } 
        else {
            res.status(403).send(`requested token_id ${req.query.token_id} is greater than what was found onchain ${currentTokenID}`);
        }
    } 
    else {
        res.status(400).send(`input ${req.query.token_id} is NaN ${typeof req.query.token_id}`);
    }
});

async function getCurrentTokenID() {
    const stateTokenID = await zilliqa.blockchain.getSmartContractSubState(
        nftContract,
        'token_id_count',
        [],
    );
    //console.log(stateTokenID.result.token_id_count);
    return stateTokenID.result.token_id_count
}

async function isTokenIDBurnt(token_id) {
    const stateTokenOwner = await zilliqa.blockchain.getSmartContractSubState(
        nftContract,
        'token_owners',
        [token_id],
    );
    //console.log(stateTokenOwner.result.token_owners);
    if(stateTokenOwner.result === null ) return true
    else if (stateTokenOwner.result.token_owners !== null) return false
}

app.listen(port, () => console.log(`Listening on ${port}`))
```

### Contract state

```json
token_owners
{
  "1": "0x24ddedbf3a3df608f4c9fbf56153866947e1b159",
  "2": "0x24ddedbf3a3df608f4c9fbf56153866947e1b159",
  "3": "0x24ddedbf3a3df608f4c9fbf56153866947e1b159"
}
```

### Metadata exists for an existing token

In this case, the token exists in state and has not been burnt.

```bash
curl --location --request GET 'http://localhost:3000/?token_id=3'

200 token_id sent is 3 and this token_id hasn't been burnt, current token_id onchain is 4
```

### Metadata does not exist for a burnt token

In this case, the token has been burnt and metadata shouldn't be retrieved.

```bash
curl --location --request GET 'http://localhost:3000/?token_id=4'

404 requested token_id 4 has been burnt!
```

### Metadata not exposed for non minted tokens

In this case, the token requested does not exist on the chain and shouldn't be exposed until this token has been minted.

```bash
curl --location --request GET 'http://localhost:3000/?token_id=5'

403 requested token_id 5 is greater than what was found onchain 4
```

## Further reading

[ipfs.io - Best Practices For NFT Data](https://docs.ipfs.io/how-to/best-practices-for-nft-data)

[Zilliqa Github - What is Metadata and Token URI?](https://github.com/Zilliqa/ZRC/blob/main/zrcs/zrc-7.md#i-what-is-metadata-and-token-uri)
