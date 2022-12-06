---
id: brand-collectible-list
title: Brand Collectible List
keywords:
  - marketplace
  - brand collectible
description: Brand collectible list
---

---

Get list of all collectibles owned by a brand.

### Parameters

| Name         | Type   | Required | Description                  |
| ------------ | ------ | -------- | ---------------------------- |
| `brandOwner` | String | Required | Address of the brand account |

### Example Request

=== "Graphql"

    #### Query

    ```graphql
    query BrandCollectibles($input: BrandCollectiblesInput) {
      brandCollectibles(input: $input) {
        cursor
        brandCollectiblesList {
          collectionId
          brandOwner
          createdAt
          collectionContract
          commissionFee
        }
      }
    }
    ```

    #### Query Variables

    ```graphql
    {
    	"input": {
        "brandOwner": "0x22b251cc155ac0a181a156aaec74e964a82011c1"
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
    "brandCollectibles": {
      "cursor": "100",
      "brandCollectiblesList": [
        {
          "collectionId": "11",
          "brandOwner": "0x22b251cc155ac0a181a156aaec74e964a82011c1",
          "createdAt": "2022-11-15T15:21:48.241312+00:00",
          "collectionContract": "0x2878928cadf313ef27b35f985ef3e57b2aac7f4d",
          "commissionFee": "250"
        }
      ]
    }
  }
}
```
