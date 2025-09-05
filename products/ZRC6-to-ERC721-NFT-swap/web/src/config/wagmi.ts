import { getDefaultConfig } from '@rainbow-me/rainbowkit';
import {
  zilliqaTestnet
} from 'wagmi/chains';

export const config = getDefaultConfig({
  appName: 'ZRC6 to ERC721 NFT Swap',
  projectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || 'YOUR_PROJECT_ID', // Get this from https://cloud.walletconnect.com
  chains: [
    zilliqaTestnet
  ],
  ssr: true, // If your dApp uses server side rendering (SSR)
});
