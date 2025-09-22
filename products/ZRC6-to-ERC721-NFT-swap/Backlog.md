# Backlog

## 24 Modify the `swapZRC6NFTForErc721NFTByByrningZRC6` to rely on the ZilPay approval for EVM wallet address instead of signatures

The `function swapZRC6NFTForErc721NFTByByrningZRC6` currently takes in `signature`. We don't need it cause the safeguard mechanism here is ZilPay wallet approving the EVM wallet as operator before calling this method. Remove the `signature` from the code and any documentation.

## 25 Implement the _transferEvmNFTs

Transfer NFTs from _evmNFTAddress according to the `nftSwapMapping`


## 027 Add tooltips and descriptions in UI to make the application straightforward to use

## 028 Modify the web UI around minting new Scilla NFTs

so it is better described that minting the new Scilla NFT is for testing purpose only and normally, for other collections, users would need to acquire NFTs outside of the swaping app