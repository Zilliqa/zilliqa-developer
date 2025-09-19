import { useState } from 'react'
import { useWallet } from '../context/WalletContext'
import { useNFTSwap } from '../hooks/useNFTSwap'
import TransactionStatus from './TransactionStatus'
import { formatAddress } from '../utils/formatting'

interface SwapComponentProps {
  onSwapComplete?: () => void
}

export default function SwapComponent({ onSwapComplete }: SwapComponentProps) {
  const { zilPayAccount, evmAccount } = useWallet()
  const { swapNFTs, swapState, reset } = useNFTSwap()

  const [selectedTokenIds, setSelectedTokenIds] = useState<number[]>([])
  const [newTokenId, setNewTokenId] = useState<string>('')
  const [signature, setSignature] = useState<string>('')

  const handleAddTokenId = () => {
    const id = parseInt(newTokenId)
    if (!isNaN(id) && !selectedTokenIds.includes(id)) {
      setSelectedTokenIds([...selectedTokenIds, id])
      setNewTokenId('')
    }
  }

  const handleRemoveTokenId = (id: number) => {
    setSelectedTokenIds(selectedTokenIds.filter(tokenId => tokenId !== id))
  }

  const handleSwap = async () => {
    if (!zilPayAccount || !evmAccount || selectedTokenIds.length === 0) {
      return
    }

    try {
      await swapNFTs(
        zilPayAccount,
        selectedTokenIds,
        (createdSignature) => {
          setSignature(createdSignature)
        }
      )
      onSwapComplete?.()
    } catch (error) {
      console.error('Swap failed:', error)
    }
  }

  const handleReset = () => {
    reset()
    setSelectedTokenIds([])
    setSignature('')
  }

  const canSwap = zilPayAccount && evmAccount && selectedTokenIds.length > 0 && !swapState.isPreparing && !swapState.isSigning && !swapState.isSwapping && !swapState.isConfirming

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
          <h4 className="font-medium text-gray-700 mb-3">Select ZRC6 Token IDs to Swap</h4>
          <div className="flex gap-2 mb-3">
            <input
              type="number"
              value={newTokenId}
              onChange={(e) => setNewTokenId(e.target.value)}
              placeholder="Enter token ID"
              className="flex-1 px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
              onKeyPress={(e) => e.key === 'Enter' && handleAddTokenId()}
            />
            <button
              onClick={handleAddTokenId}
              disabled={!newTokenId || selectedTokenIds.includes(parseInt(newTokenId))}
              className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed"
            >
              Add
            </button>
          </div>

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

        {/* Signature Display */}
        {signature && (
          <div className="mb-6">
            <h4 className="font-medium text-gray-700 mb-2">Generated Signature</h4>
            <div className="bg-gray-50 p-3 rounded">
              <p className="text-xs font-mono break-all text-gray-600">{signature}</p>
            </div>
          </div>
        )}

        {/* Transaction Status */}
        <TransactionStatus
          isPreparation={swapState.isPreparing}
          isProcessing={swapState.isSigning || swapState.isSwapping}
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
            {swapState.isPreparing ? 'Preparing...' :
             swapState.isSigning ? 'Signing with ZilPay...' :
             swapState.isSwapping ? 'Swapping NFTs...' :
             swapState.isConfirming ? 'Confirming...' :
             'Swap NFTs'}
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
          <li>1. Select the ZRC6 token IDs you want to swap</li>
          <li>2. Click &quot;Swap NFTs&quot; to sign with ZilPay and execute the swap</li>
          <li>3. Your ZRC6 tokens will be burned and ERC721 tokens will be minted</li>
        </ol>
      </div>
    </div>
  )
}
