import Image from "next/image";
import { Geist, Geist_Mono } from "next/font/google";
import { ConnectButton } from '@rainbow-me/rainbowkit';
import { useWallet } from '@/context/WalletContext';

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
    zilPayAccount, 
    isZilPayConnected, 
    connectZilPay, 
    disconnectZilPay,
    evmAccount,
    isEvmConnected,
    disconnectEvm
  } = useWallet();

  return (
    <div
      className={`${geistSans.className} ${geistMono.className} font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20`}
    >
      <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start">
        <h1 className="text-4xl font-bold">ZRC6 to ERC721 NFT Swap</h1>
        <p className="text-lg">
          Swap your ZRC6 tokens for ERC721 NFTs seamlessly.
        </p>

        <div className="flex flex-col gap-4 w-full max-w-md">
          {/* ZilPay Wallet Connection */}
          <div className="border rounded-lg p-4">
            <h3 className="text-lg font-semibold mb-2">Zilliqa non-EVM Network</h3>
            {isZilPayConnected ? (
              <div className="space-y-2">
                <p className="text-sm text-green-600">✓ Connected</p>
                <p className="text-xs font-mono break-all">{zilPayAccount}</p>
                <button
                  className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition text-sm"
                  onClick={disconnectZilPay}
                >
                  Disconnect ZilPay
                </button>
              </div>
            ) : (
              <button
                className="px-6 py-3 bg-blue-600 text-white rounded hover:bg-blue-700 transition w-full"
                onClick={connectZilPay}
              >
                Connect ZilPay Wallet
              </button>
            )}
          </div>

          {/* EVM Wallet Connection */}
          <div className="border rounded-lg p-4">
            <h3 className="text-lg font-semibold mb-2">Zilliqa EVM Network</h3>
            {isEvmConnected ? (
              <div className="space-y-2">
                <p className="text-sm text-green-600">✓ Connected</p>
                <p className="text-xs font-mono break-all">{evmAccount}</p>
                <button
                  className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition text-sm"
                  onClick={disconnectEvm}
                >
                  Disconnect EVM Wallet
                </button>
              </div>
            ) : (
              <div className="w-full">
                <ConnectButton.Custom>
                  {({
                    account,
                    chain,
                    openAccountModal,
                    openChainModal,
                    openConnectModal,
                    authenticationStatus,
                    mounted,
                  }) => {
                    const ready = mounted && authenticationStatus !== 'loading';
                    const connected =
                      ready &&
                      account &&
                      chain &&
                      (!authenticationStatus ||
                        authenticationStatus === 'authenticated');

                    return (
                      <div
                        {...(!ready && {
                          'aria-hidden': true,
                          'style': {
                            opacity: 0,
                            pointerEvents: 'none',
                            userSelect: 'none',
                          },
                        })}
                      >
                        {(() => {
                          if (!connected) {
                            return (
                              <button
                                onClick={openConnectModal}
                                type="button"
                                className="px-6 py-3 bg-purple-600 text-white rounded hover:bg-purple-700 transition w-full"
                              >
                                Connect EVM Wallet
                              </button>
                            );
                          }

                          if (chain.unsupported) {
                            return (
                              <button
                                onClick={openChainModal}
                                type="button"
                                className="px-6 py-3 bg-red-600 text-white rounded hover:bg-red-700 transition w-full"
                              >
                                Wrong network
                              </button>
                            );
                          }

                          return (
                            <div style={{ display: 'flex', gap: 12 }}>
                              <button
                                onClick={openChainModal}
                                style={{ display: 'flex', alignItems: 'center' }}
                                type="button"
                                className="px-3 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition"
                              >
                                {chain.hasIcon && (
                                  <div
                                    style={{
                                      background: chain.iconBackground,
                                      width: 12,
                                      height: 12,
                                      borderRadius: 999,
                                      overflow: 'hidden',
                                      marginRight: 4,
                                    }}
                                  >
                                    {chain.iconUrl && (
                                      <img
                                        alt={chain.name ?? 'Chain icon'}
                                        src={chain.iconUrl}
                                        style={{ width: 12, height: 12 }}
                                      />
                                    )}
                                  </div>
                                )}
                                {chain.name}
                              </button>

                              <button
                                onClick={openAccountModal}
                                type="button"
                                className="px-3 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition"
                              >
                                {account.displayName}
                                {account.displayBalance
                                  ? ` (${account.displayBalance})`
                                  : ''}
                              </button>
                            </div>
                          );
                        })()}
                      </div>
                    );
                  }}
                </ConnectButton.Custom>
              </div>
            )}
          </div>

          {/* Swap Section */}
          {isZilPayConnected && isEvmConnected && (
            <div className="border-2 border-green-500 rounded-lg p-4 bg-green-50">
              <h3 className="text-lg font-semibold mb-2 text-green-800">Ready to Swap!</h3>
              <p className="text-sm text-green-700 mb-3">
                Both wallets connected. You can now proceed with the NFT swap.
              </p>
              <button
                className="px-6 py-3 bg-green-600 text-white rounded hover:bg-green-700 transition w-full"
                onClick={() => {
                  // TODO: Implement swap functionality
                  alert('Swap functionality will be implemented here');
                }}
              >
                Start NFT Swap
              </button>
            </div>
          )}
        </div>
      </main>
      <footer className="row-start-3 flex gap-[24px] flex-wrap items-center justify-center">
        <div>
          ZRC6 to ERC721 NFT Swap application template by{" "}
          <a
            className="underline"
            href="https://zilliqa.com"
          >
            Zilliqa
          </a>
        </div>
      </footer>
    </div>
  );
}
