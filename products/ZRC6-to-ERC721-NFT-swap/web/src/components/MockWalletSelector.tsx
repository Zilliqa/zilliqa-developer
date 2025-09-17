import React from "react"
import { useWallet } from "../context/WalletContext"
import { formatZIL } from "../utils/formatting"

const MockWalletSelector: React.FC = () => {
  const {
    isMockWalletSelectorOpen,
    setIsMockWalletSelectorOpen,
    selectMockWallet,
    mockWallets,
  } = useWallet()

  if (!isMockWalletSelectorOpen) {
    return null
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h3 className="text-xl font-bold mb-4">Select Mock Wallet</h3>
        <div className="space-y-3">
          {mockWallets.map((wallet) => (
            <div
              key={wallet.id}
              className="p-4 border rounded-lg cursor-pointer hover:bg-gray-50 hover:border-blue-300 transition-colors"
              onClick={() => selectMockWallet(wallet)}
            >
              <div className="font-semibold text-lg">{wallet.name}</div>
              <div className="text-sm text-gray-600 font-mono break-all">
                {wallet.address}
              </div>
              <div className="text-sm font-medium text-green-600">
                {formatZIL(wallet.balance)} ZIL
              </div>
              <div className="text-xs text-gray-500 mt-1">
                {wallet.description}
              </div>
            </div>
          ))}
        </div>
        <div className="flex justify-end mt-6">
          <button
            className="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition"
            onClick={() => setIsMockWalletSelectorOpen(false)}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  )
}

export default MockWalletSelector
