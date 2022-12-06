---
id: brand-collectible-asset
title: Brand Collectible NFTs
keywords:
  - marketplace
  - brand collectible
description: Assets in a brand collectible
---

---

Get list of all NFTs in a brand collectible.

### Parameters

| Name           | Type   | Required | Description               |
| -------------- | ------ | -------- | ------------------------- |
| `collectionId` | String | Required | Id of a brand collectible |

### Example Request

=== "Graphql"

    #### Query

    ```graphql
    query BrandCollectibleNFTs($input: BrandCollectiblesNFTInput) {
      brandCollectiblesNFTs(input: $input) {
        cursor
        brandCollectiblesNFTList {
          collectionId
          createdAt
          collectionContract
        }
      }
    }
    ```

    #### Query Variables

    ```graphql
    {
    	"input": {
        "collectionId": "11"
    	}
    }
    ```

    #### HTTP Headers

    ```graphql
    {
      "Authorization": "Bearer <insert token>"
    }
    ```

=== "cURL"

    ```curl

    ```

### Example Response

```json
{
  "data": {
    "brandCollectiblesNFTs": {
      "cursor": "100",
      "brandCollectiblesNFTList": [
        {
          "collectionId": "11",
          "createdAt": "2022-11-25T07:30:49.313217+00:00",
          "collectionContract": "0x2878928cadf313ef27b35f985ef3e57b2aac7f4d"
        }
      ]
    }
  }
}
```
