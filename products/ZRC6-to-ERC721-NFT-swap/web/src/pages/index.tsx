import { Geist, Geist_Mono } from "next/font/google";
import { ConnectButton } from '@rainbow-me/rainbowkit';
import { useWallet } from '@/context/WalletContext';
import CustomWalletConnect from '@/components/CustomWalletConnect';
import SwapComponent from '@/components/SwapComponent';
import MintNFTComponent from '@/components/MintNFTComponent';
import OwnedNFTs from '@/components/OwnedNFTs';
import { formatAddress } from '@/utils/formatting';

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export default function Home() {
  const { 
    // ZilPay wallet
    zilPayAccount, 
    isZilPayConnected, 
    isZilPayConnecting,
    connectZilPay, 
    disconnectZilPay,
    
    // EVM wallet
    evmAccount,
    isEvmConnected,
  } = useWallet();

  return (
    <>
      <div
        className={`${geistSans.className} ${geistMono.className} font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20`}
      >
        <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start">
          <div className="text-center sm:text-left">
            <h1 className="text-4xl font-bold mb-4">ZRC6 to ERC721 NFT Swap</h1>
            <p className="text-lg text-gray-600">
              Swap your ZRC6 tokens for ERC721 NFTs seamlessly across Zilliqa networks.
            </p>
          </div>

          <div className="flex flex-col gap-6 w-full max-w-2xl">
            {/* Zilliqa Non-EVM Network (ZRC6) */}
            <div className="border rounded-lg p-6 bg-white shadow-sm">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-xl font-semibold">Zilliqa Network (ZRC6)</h3>
                <div className="text-sm text-gray-500">Non-EVM</div>
              </div>
              
              {isZilPayConnected ? (
                <div className="space-y-3">
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                    <span className="text-sm font-medium text-green-700">Connected via ZilPay</span>
                  </div>
                  <div className="bg-gray-50 p-3 rounded text-sm">
                    <div className="font-mono text-xs break-all text-gray-700">
                      {zilPayAccount}
                    </div>
                  </div>
                  <button
                    className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition text-sm"
                    onClick={disconnectZilPay}
                  >
                    Disconnect ZilPay
                  </button>

                  {/* Mint NFT Component */}
                  <MintNFTComponent zilPayAccount={zilPayAccount || ''} />

                  {/* Owned NFTs Component */}
                  <OwnedNFTs zilPayAccount={zilPayAccount || ''} />
                </div>
              ) : (
                <div className="space-y-3">
                  <p className="text-sm text-gray-600">
                    Connect your ZilPay wallet to access ZRC6 tokens on the Zilliqa network.
                  </p>
                  <button
                    className="w-full px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg hover:from-blue-700 hover:to-purple-700 transition font-medium"
                    onClick={connectZilPay}
                    disabled={isZilPayConnecting}
                  >
                    {isZilPayConnecting ? 'Connecting...' : 'Connect ZilPay Wallet'}
                  </button>
                </div>
              )}
            </div>

            {/* Zilliqa EVM Network (ERC721) */}
            <div className="border rounded-lg p-6 bg-white shadow-sm">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-xl font-semibold">Zilliqa EVM Network (ERC721)</h3>
                <div className="text-sm text-gray-500">EVM Compatible</div>
              </div>
              
              {isEvmConnected ? (
                <div className="space-y-3">
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                    <span className="text-sm font-medium text-green-700">Connected via EVM Wallet</span>
                  </div>
                  <div className="bg-gray-50 p-3 rounded text-sm">
                    <div className="font-mono text-xs break-all text-gray-700">
                      {evmAccount}
                    </div>
                  </div>
                  <div className="flex justify-start">
                    <ConnectButton />
                  </div>
                </div>
              ) : (
                <div className="space-y-3">
                  <p className="text-sm text-gray-600">
                    Connect your EVM wallet (MetaMask, WalletConnect, etc.) to receive ERC721 NFTs.
                  </p>
                  <CustomWalletConnect notConnectedClassName="w-full">
                    Connect EVM Wallet
                  </CustomWalletConnect>
                </div>
              )}
            </div>

            {/* Swap Section */}
            {isZilPayConnected && isEvmConnected && (
              <div className="border-2 border-green-500 rounded-lg p-6 bg-green-50">
                <div className="flex items-center gap-3 mb-4">
                  <div className="w-4 h-4 bg-green-500 rounded-full"></div>
                  <h3 className="text-xl font-semibold text-green-800">Ready to Swap!</h3>
                </div>
                <p className="text-sm text-green-700 mb-4">
                  Both wallets are connected. You can now proceed with the NFT swap.
                </p>
                
                <div className="bg-white p-4 rounded border mb-4">
                  <div className="text-sm space-y-2">
                    <div className="flex justify-between">
                      <span className="text-gray-600">Zilliqa Wallet:</span>
                      <span className="font-mono text-xs">
                        {formatAddress(isZilPayConnected ? (zilPayAccount || '') : '')}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600">EVM Wallet:</span>
                      <span className="font-mono text-xs">
                        {formatAddress(evmAccount || '')}
                      </span>
                    </div>
                  </div>
                </div>

                <SwapComponent />
              </div>
            )}

            {/* Connection Status */}
            {!(isZilPayConnected && isEvmConnected) && (
              <div className="border border-yellow-400 rounded-lg p-4 bg-yellow-50">
                <h3 className="text-lg font-semibold text-yellow-800 mb-2">Connection Required</h3>
                <div className="text-sm text-yellow-700 space-y-1">
                  <div className="flex items-center gap-2">
                    {isZilPayConnected ? '✅' : '❌'}
                    <span>Zilliqa wallet (for ZRC6 tokens)</span>
                  </div>
                  <div className="flex items-center gap-2">
                    {isEvmConnected ? '✅' : '❌'}
                    <span>EVM wallet (for ERC721 NFTs)</span>
                  </div>
                </div>
                <p className="text-xs text-yellow-600 mt-2">
                  Connect both wallets to enable the swap functionality.
                </p>
              </div>
            )}
          </div>
        </main>

        <footer className="row-start-3 flex gap-[24px] flex-wrap items-center justify-center text-sm text-gray-500">
          <div>
            ZRC6 to ERC721 NFT Swap • Template by{" "}
            <a
              className="underline hover:text-gray-700 transition"
              href="https://zilliqa.com"
              target="_blank"
              rel="noopener noreferrer"
            >
              Zilliqa
            </a>
          </div>
        </footer>
      </div>
    </>
  );
}
