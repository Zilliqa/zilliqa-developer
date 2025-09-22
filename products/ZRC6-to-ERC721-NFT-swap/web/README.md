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
3. User approves EVM wallet as operator for their ZRC6 NFTs using ZilPay
4. Application checks ZRC6 token balance
5. User selects ZRC6 tokens to swap for ERC721 NFTs
6. User calls `swapZRC6NFTForErc721NFTByByrningZRC6` function on `burnScillaAndMintEVMNFTSwap` using EVM wallet
7. The user is informed about the outcome

## Technical Implementation

### Approval Setup (Step 3)
The application requires users to approve their EVM wallet as an operator for their ZRC6 NFTs using ZilPay. This approval allows the EVM wallet to burn the NFTs on behalf of the ZilPay wallet during the swap process.

### Contract Interaction (Step 6)
The approval is verified by the smart contract during the burn operation:

```solidity
function swapZRC6NFTForErc721NFTByByrningZRC6(
    uint256[] memory scillaNftIdsToSwap
) external nonReentrant whenNotPaused
```

The contract verifies the caller's operator approval on the Scilla side before proceeding with the swap.

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
NEXT_PUBLIC_CHAIN_ID=33101
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

