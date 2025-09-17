import React from "react"
import { ConnectButton } from "@rainbow-me/rainbowkit"
import { useWallet, ConnectedWalletType } from "../context/WalletContext"
import { formatAddress, formatZIL } from "../utils/formatting"

interface CustomWalletConnectProps {
  children: React.ReactNode
  notConnectedClassName?: string
}

const CustomWalletConnect: React.FC<CustomWalletConnectProps> = ({
  children,
  notConnectedClassName = "",
}) => {
  const {
    connectedWalletType,
    isDummyWalletConnecting,
    isDummyWalletConnected,
    disconnectDummyWallet,
    walletAddress,
    zilAvailable,
  } = useWallet()

  // Mock wallet handling
  if (connectedWalletType === ConnectedWalletType.MockWallet || (isDummyWalletConnected && !isDummyWalletConnecting)) {
    return (
      <button
        className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition group relative"
        onClick={disconnectDummyWallet}
      >
        <div className="group-hover:hidden flex items-center">
          💼 {formatAddress(walletAddress || "")} | {formatZIL(zilAvailable || BigInt(0))} ZIL
        </div>
        <span className="hidden group-hover:block">
          Disconnect
        </span>
      </button>
    )
  }

  if (isDummyWalletConnecting) {
    return (
      <button
        className={`px-4 py-2 bg-gray-400 text-white rounded cursor-not-allowed ${notConnectedClassName}`}
        disabled
      >
        Connecting...
      </button>
    )
  }

  // Real wallet handling with ConnectButton
  return (
    <ConnectButton.Custom>
      {({ account, chain, openConnectModal, mounted }) => {
        if (!mounted) {
          return (
            <button className={`px-4 py-2 bg-gray-400 text-white rounded ${notConnectedClassName}`}>
              Loading...
            </button>
          )
        }

        if (!account || !chain) {
          return (
            <button
              onClick={openConnectModal}
              className={`px-6 py-3 bg-purple-600 text-white rounded hover:bg-purple-700 transition ${notConnectedClassName}`}
            >
              {children}
            </button>
          )
        }

        return (
          <div className="flex justify-end items-center">
            <ConnectButton />
          </div>
        )
      }}
    </ConnectButton.Custom>
  )
}

export default CustomWalletConnect
