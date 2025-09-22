import React from "react"
import { ConnectButton } from "@rainbow-me/rainbowkit"

interface CustomWalletConnectProps {
  children: React.ReactNode
  notConnectedClassName?: string
}

const CustomWalletConnect: React.FC<CustomWalletConnectProps> = ({
  children,
  notConnectedClassName = "",
}) => {
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
