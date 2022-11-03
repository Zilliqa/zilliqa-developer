---
sidebar_position: 2
---

# Deploy ZRC-6

Firstly, we'll need to fund our account through the testnet [faucet](https://dev-wallet.zilliqa.com/faucet?network=testnet).

Copy the contract from the ZRC standard for [ZRC-6](https://github.com/Zilliqa/ZRC/blob/master/zrcs/zrc-6.md) - create a new blank Scilla file - paste in the copied contract and rename the IDE file to ZRC-6.

Press deploy and the deployment initialization window will appear. Change Gas price to ```2000000000``` and gas limit to ```30000```. The default amount is too small for the amount of bytes we are trying to deploy.

![Docusaurus](/img/tutorials/nft-collection/zrc6-params.png)

```initial_contract_owner``` This is an address you control that's considered the lead administrator of the contract. This address can later give other addresses permission to have ```Mint``` permissions or ```Operator``` permissions.

```initial_base_uri``` This is an optional way of providing metadata/images for your tokens by using an API.

An example is when ```initial_base_uri``` is set to ```www.api.example.com/``` and token_id ```1``` is queried, ecosystem partners will look at ```www.api.example.com/1```.

We'll come back to this later in detail. Set this as an empty string for now.

```name``` The name of your NFT contract. (MyNFTProject)

```symbol``` The ticker symbol of your NFT contract. (MNP)

With our immutable variables filled out, we can deploy our contract to the network.

Once the contract has been deployed, you'll see a success message and the contract will appear in the bottom right. If you click on the contract, you can see all the transitions you can call.

![Docusaurus](/img/tutorials/nft-collection/zrc6-transitions.png)

The contract needs to split responsibilities so that different actors can interact with the NFT contract. 

The owner has permissions to set new Minters and Operators as well as the permission to mint new tokens to this contract. They can set and configure the contract using the transitions below.

* Owners transitions
  * Pause / Unpause
  * SetRoyaltyRecipient / SetRoyaltyFeeBPS
  * SetBaseURI
  * AddMinter / RemoveMinter
  * AddOperator / RemoveOperator
  * SetContractOwnershipRecipient
  * Mint / BatchMint

```Minters``` are privileged addresses that are allowed to Mint NFTs unconditionally, the contract owner can add other minting addresses they would like to control. This functionality is useful for when we want to delegate a proxy address to be responsible for minting. Owners can also revoke the Mint permission.
  
* Minters transitions
  * Mint / BatchMint

```Operators``` are privileged addresses that have the ability to move tokens to and from addresses. Similar to ```ApproveAllowance```, a token owner has to explicitly set an address for this functionality to be operable. This functionality is useful when you want to move NFT's to/from in the future, in the case where the user moves an NFT away from their address the ```Transfer``` transition will fail.

* Operators transitions
  * BurnToken
  * SetSpender
  * TransferToken

If you own token_id 1, you have the ability to Transfer, Burn or give another contract the ability to spend this token on your behalf.

* Token owner transitions
  * TransferFrom
  * SetSpender / RemoveSpender
  * BurnToken
