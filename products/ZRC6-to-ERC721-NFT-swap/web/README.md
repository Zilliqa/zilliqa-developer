# ZRC6 to ERC721 NFT Swap

A Next.js application for swapping ZRC6 tokens to ERC721 NFTs across Zilliqa networks, built following the Zilliqa Wallet Integration Guide.

## Features

- 🔄 **Cross-Network Swapping**: Convert ZRC6 tokens to ERC721 NFTs
- 💳 **Multiple Wallet Support**: 
  - ZilPay for Zilliqa non-EVM network 
  - EVM wallets (MetaMask, WalletConnect, Coinbase, etc.) for Zilliqa EVM network
- 🛠️ **Developer Tools**: Mock wallets for testing and development
- 🔒 **Type-Safe**: Full TypeScript implementation with proper error handling
- 🎨 **Modern UI**: Responsive design with Tailwind CSS
- ⚡ **Performance Optimized**: Built with Next.js 15 and React 18

## Architecture

This application follows the **EVM-first approach** recommended in the Zilliqa Wallet Integration Guide:

1. **Primary**: EVM wallets (MetaMask, WalletConnect, etc.) for broad compatibility
2. **Enhanced**: ZilPay for Zilliqa-specific features (optional)
3. **Development**: Mock wallets for testing without real funds

## Swapping Flow

1. User connects ZilPay wallet (Zilliqa non-EVM network)
2. User connects EVM wallet (Zilliqa EVM network)
3. Application checks ZRC6 token balance
4. User selects ZRC6 tokens to swap for ERC721 NFTs
5. User approves the swap contract as spender using ZilPay
6. User calls swap function using EVM wallet
7. ERC721 NFT is minted to user's EVM address

## Getting Started

### 1. Install Dependencies

```bash
npm install
```

### 2. Environment Setup

Copy the environment template:

```bash
cp .env.example .env.local
```

Update `.env.local` with your configuration:

```env
NEXT_PUBLIC_CHAIN_ID=33469
NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID=your_project_id_here
NEXT_PUBLIC_APP_URL=http://localhost:3000
NEXT_PUBLIC_APP_NAME=ZRC6_to_ERC721_NFT_Swap
```

### 3. Run Development Server

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

## Key Features

### Wallet Integration
- **EVM-First Approach**: Prioritizes widely-adopted wallets (MetaMask, WalletConnect)
- **ZilPay Enhancement**: Optional for Zilliqa-specific features
- **Mock Wallet System**: For development and testing

### Developer Tools
- Mock wallet selector with different balance scenarios
- Debug state logging
- Transaction simulation
- Type-safe contract interactions

### Production Ready
- Comprehensive error handling
- Loading states and user feedback
- Responsive design
- Security best practices

## Technologies Used

- **Next.js 15**: React framework
- **TypeScript**: Type safety
- **RainbowKit**: Wallet connection UI
- **Wagmi**: React hooks for Ethereum
- **Viem**: TypeScript Ethereum library
- **Tailwind CSS**: Styling

## License

Part of the Zilliqa Developer Portal

---

Built with ❤️ for the Zilliqa ecosystem

