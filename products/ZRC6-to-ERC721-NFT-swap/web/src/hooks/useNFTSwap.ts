import { useState, useEffect } from 'react'
import { useWriteContract, useWaitForTransactionReceipt, useAccount } from 'wagmi'
import { SWAP_CONTRACT_ABI, CONTRACT_ADDRESSES } from '../config/contracts'

export interface SwapState {
  isSwapping: boolean
  isConfirming: boolean
  txHash?: string
  error?: Error
}

export function useNFTSwap() {
  const [swapState, setSwapState] = useState<SwapState>({
    isSwapping: false,
    isConfirming: false,
  })

  const { address: evmAddress, chain } = useAccount()
  const { writeContract, data: hash, error: writeError } = useWriteContract()
  const { isLoading: isConfirming, error: confirmError } = useWaitForTransactionReceipt({
    hash,
  })

  const swapNFTs = async (
    nftIds: number[]
  ) => {
    if (!evmAddress) {
      throw new Error('EVM wallet not connected')
    }

    if (!chain?.id || !CONTRACT_ADDRESSES[chain.id as keyof typeof CONTRACT_ADDRESSES]) {
      throw new Error('Unsupported network')
    }

    const contractAddress = CONTRACT_ADDRESSES[chain.id as keyof typeof CONTRACT_ADDRESSES].SWAP

    setSwapState({
      isSwapping: false,
      isConfirming: false,
    })

    try {
      // Step 1: Call the swap contract
      setSwapState(prev => ({ ...prev, isSwapping: true }))

      // Convert number[] to bigint[]
      const tokenIds = nftIds.map(id => BigInt(id))

      writeContract({
        address: contractAddress,
        abi: SWAP_CONTRACT_ABI,
        functionName: 'swapZRC6NFTForErc721NFTByByrningZRC6',
        args: [
          tokenIds
        ],
      })

      setSwapState(prev => ({ ...prev, isSwapping: false, isConfirming: true, txHash: hash }))

    } catch (error) {
      console.error('Swap failed:', error)
      setSwapState(prev => ({
        ...prev,
        isSwapping: false,
        isConfirming: false,
        error: error as Error
      }))
      throw error
    }
  }

  // Update state when transaction hash is available
  useEffect(() => {
    if (hash && !swapState.txHash) {
      setSwapState(prev => ({ ...prev, txHash: hash, isConfirming: true }))
    }
  }, [hash, swapState.txHash])

  // Update state when transaction is confirmed
  useEffect(() => {
    if (!isConfirming && swapState.isConfirming) {
      setSwapState(prev => ({ ...prev, isConfirming: false }))
    }
  }, [isConfirming, swapState.isConfirming])

  // Handle errors
  useEffect(() => {
    if (writeError || confirmError) {
      setSwapState(prev => ({
        ...prev,
        error: (writeError || confirmError) as Error,
        isConfirming: false
      }))
    }
  }, [writeError, confirmError])

  const reset = () => {
    setSwapState({
      isSwapping: false,
      isConfirming: false,
    })
  }

  return {
    swapNFTs,
    swapState,
    reset,
  }
}
