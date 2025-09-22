import { useWallet } from '../context/WalletContext'
import { useNFTSwap } from '../hooks/useNFTSwap'
import TransactionStatus from './TransactionStatus'
import { formatAddress } from '../utils/formatting'
import { useState } from 'react'
import { approveEvmWalletAsOperator } from '../config/zilpay-wallet'
import { CONTRACT_ADDRESSES } from '../config/contracts'

interface SwapComponentProps {
  onSwapComplete?: () => void
  selectedTokenIds: string[]
  onRemoveSelected: (tokenId: string) => void
}

export default function SwapComponent({ onSwapComplete, selectedTokenIds, onRemoveSelected }: SwapComponentProps) {
  const { zilPayAccount, evmAccount } = useWallet()
  const { swapNFTs, swapState, reset } = useNFTSwap()
  const [isApproving, setIsApproving] = useState(false)

  const handleRemoveTokenId = (tokenId: string) => {
    onRemoveSelected(tokenId)
  }

  const handleSwap = async () => {
    if (!evmAccount || selectedTokenIds.length === 0) {
      return
    }

    try {
      await swapNFTs(
        selectedTokenIds.map(id => parseInt(id))
      )
      onSwapComplete?.()
    } catch (error) {
      console.error('Swap failed:', error)
    }
  }

  const handleReset = () => {
    reset()
    // selectedTokenIds are now managed by parent component
  }

  const handleApprove = async () => {
    if (!zilPayAccount || !evmAccount) return

    setIsApproving(true)
    try {
      // Use testnet contract address
      const contractAddress = CONTRACT_ADDRESSES[33101].ZRC6
      await approveEvmWalletAsOperator(contractAddress, evmAccount)
      // Optionally, set a state to indicate approval success
    } catch (error) {
      console.error('Approval failed:', error)
      // Handle error, perhaps show a message
    } finally {
      setIsApproving(false)
    }
  }

  const canSwap = evmAccount && selectedTokenIds.length > 0 && !swapState.isSwapping && !swapState.isConfirming
  const isApproved = false // Replace with actual approval state check

  return (
    <div className="space-y-6">
      <div className="bg-white p-6 rounded-lg border shadow-sm">
        <h3 className="text-xl font-semibold mb-4">NFT Swap Details</h3>

        {/* Wallet Information */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
          <div className="bg-gray-50 p-4 rounded">
            <h4 className="font-medium text-gray-700 mb-2">ZilPay Wallet (ZRC6)</h4>
            <p className="text-sm font-mono break-all text-gray-600">
              {zilPayAccount ? formatAddress(zilPayAccount) : 'Not connected'}
            </p>
          </div>
          <div className="bg-gray-50 p-4 rounded">
            <h4 className="font-medium text-gray-700 mb-2">EVM Wallet (ERC721)</h4>
            <p className="text-sm font-mono break-all text-gray-600">
              {evmAccount ? formatAddress(evmAccount) : 'Not connected'}
            </p>
          </div>
        </div>

        {/* Token ID Selection */}
        <div className="mb-6">
          <h4 className="font-medium text-gray-700 mb-3">Selected ZRC6 Token IDs to Swap</h4>
          <p className="text-sm text-gray-600 mb-3">Click on NFTs in your owned list above to select them for swapping.</p>

          {selectedTokenIds.length > 0 && (
            <div className="bg-gray-50 p-3 rounded">
              <p className="text-sm text-gray-600 mb-2">Selected tokens:</p>
              <div className="flex flex-wrap gap-2">
                {selectedTokenIds.map(id => (
                  <span
                    key={id}
                    className="inline-flex items-center gap-1 px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm"
                  >
                    #{id}
                    <button
                      onClick={() => handleRemoveTokenId(id)}
                      className="ml-1 text-blue-600 hover:text-blue-800"
                    >
                      ×
                    </button>
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Transaction Status */}
        <TransactionStatus
          isPreparation={false}
          isProcessing={swapState.isSwapping}
          txHash={swapState.txHash}
          error={swapState.error}
          className="mb-4"
        />

        {/* Action Buttons */}
        <div className="flex gap-3">
          <button
            onClick={handleSwap}
            disabled={!canSwap}
            className="flex-1 px-6 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:bg-gray-300 disabled:cursor-not-allowed font-medium"
          >
            {swapState.isSwapping ? 'Swapping NFTs...' :
             swapState.isConfirming ? 'Confirming...' :
             'Swap NFTs'}
          </button>

          <button
            onClick={handleApprove}
            disabled={isApproving || isApproved}
            className="flex-1 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed font-medium"
          >
            {isApproving ? 'Approving...' : 'Approve Wallet'}
          </button>

          {(swapState.txHash || swapState.error) && (
            <button
              onClick={handleReset}
              className="px-6 py-3 bg-gray-500 text-white rounded-lg hover:bg-gray-600 font-medium"
            >
              Reset
            </button>
          )}
        </div>
      </div>

      {/* Instructions */}
      <div className="bg-blue-50 p-4 rounded-lg border border-blue-200">
        <h4 className="font-medium text-blue-800 mb-2">How it works:</h4>
        <ol className="text-sm text-blue-700 space-y-1">
          <li>1. Approve your EVM wallet as an operator for your ZRC6 NFTs using ZilPay</li>
          <li>2. Select the ZRC6 token IDs you want to swap</li>
          <li>3. Click &quot;Swap NFTs&quot; to execute the swap</li>
          <li>4. Your ZRC6 tokens will be burned and ERC721 tokens will be minted</li>
        </ol>
      </div>

      {/* Operator Approval Section */}
      <div className="mb-6">
        <h4 className="font-medium text-gray-700 mb-3">Step 1: Approve EVM Wallet as Operator</h4>
        <p className="text-sm text-gray-600 mb-3">
          Your EVM wallet must be approved as an operator for your ZRC6 NFTs to enable burning them during the swap.
        </p>
        <button
          onClick={handleApprove}
          disabled={isApproving || !zilPayAccount || !evmAccount}
          className="px-6 py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:bg-gray-300 disabled:cursor-not-allowed font-medium"
        >
          {isApproving ? 'Approving...' : 'Approve EVM Wallet as Operator'}
        </button>
      </div>
    </div>
  )
}
