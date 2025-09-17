# **ZRC6 to ERC712 NFT Swap** web application

This is a NextJS powered web application

## Swapping steps

Swapping NFTs flow is as follows:
1. User connects ZilPay
2. Application checks what Scilla NFTs user has
3. Application displays a list of user owned Scilla collection NFTs
4. User selects the Scilla collection NFTs to be swapped for EVM collection NFTs
4. User connect EVM wallet
5. User approves the swap contract as a spender of Scilla NFT using ZilPay
6. User calls `burnAndReceive` using EVM wallet

## Development

To run the development server

```bash
npm run dev
```

