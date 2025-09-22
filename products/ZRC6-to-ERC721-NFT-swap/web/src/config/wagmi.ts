import { createWagmiConfig } from './chains'

// This will be set up dynamically in _app.tsx with proper configuration
export const config = createWagmiConfig(
  33101, // Default to testnet
  process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || '',
  'ZRC6 to ERC721 NFT Swap',
  process.env.NEXT_PUBLIC_APP_URL || 'http://localhost:3000'
)
