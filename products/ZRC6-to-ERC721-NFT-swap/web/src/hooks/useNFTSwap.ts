import { useState } from 'react'
import { useWriteContract, useWaitForTransactionReceipt, useAccount } from 'wagmi'
import { SWAP_CONTRACT_ABI, CONTRACT_ADDRESSES } from '../config/contracts'
import { createEvmAddressSignature } from '../config/zilpay-wallet'

export interface SwapState {
  isPreparing: boolean
  isSigning: boolean
  isSwapping: boolean
  isConfirming: boolean
  txHash?: string
  error?: Error
}

export function useNFTSwap() {
  const [swapState, setSwapState] = useState<SwapState>({
    isPreparing: false,
    isSigning: false,
    isSwapping: false,
    isConfirming: false,
  })

  const { address: evmAddress, chain } = useAccount()
  const { writeContract, data: hash, error: writeError } = useWriteContract()
  const { isLoading: isConfirming, error: confirmError } = useWaitForTransactionReceipt({
    hash,
  })

  const swapNFTs = async (
    zilPayAddress: string,
    nftIds: number[],
    onSignatureCreated?: (signature: string) => void
  ) => {
    if (!evmAddress) {
      throw new Error('EVM wallet not connected')
    }

    if (!chain?.id || !CONTRACT_ADDRESSES[chain.id as keyof typeof CONTRACT_ADDRESSES]) {
      throw new Error('Unsupported network')
    }

    const contractAddress = CONTRACT_ADDRESSES[chain.id as keyof typeof CONTRACT_ADDRESSES].SWAP

    setSwapState({
      isPreparing: true,
      isSigning: false,
      isSwapping: false,
      isConfirming: false,
    })

    try {
      // Step 1: Create signature using ZilPay
      setSwapState(prev => ({ ...prev, isPreparing: false, isSigning: true }))

      const signature = await createEvmAddressSignature(evmAddress)
      onSignatureCreated?.(signature)

      // Step 2: Call the swap contract
      setSwapState(prev => ({ ...prev, isSigning: false, isSwapping: true }))

      // Convert number[] to bigint[]
      const tokenIds = nftIds.map(id => BigInt(id))

      writeContract({
        address: contractAddress,
        abi: SWAP_CONTRACT_ABI,
        functionName: 'swapZRC6NFTForErc721NFTByByrningZRC6',
        args: [
          zilPayAddress,
          tokenIds,
          signature as `0x${string}`
        ],
      })

      setSwapState(prev => ({ ...prev, isSwapping: false, isConfirming: true, txHash: hash }))

    } catch (error) {
      console.error('Swap failed:', error)
      setSwapState(prev => ({
        ...prev,
        isPreparing: false,
        isSigning: false,
        isSwapping: false,
        isConfirming: false,
        error: error as Error
      }))
      throw error
    }
  }

  // Update state when transaction is confirmed
  if (isConfirming && swapState.isConfirming) {
    setSwapState(prev => ({ ...prev, isConfirming: true }))
  }

  if (hash && !swapState.txHash) {
    setSwapState(prev => ({ ...prev, txHash: hash, isConfirming: true }))
  }

  if (writeError || confirmError) {
    setSwapState(prev => ({
      ...prev,
      error: (writeError || confirmError) as Error,
      isConfirming: false
    }))
  }

  const reset = () => {
    setSwapState({
      isPreparing: false,
      isSigning: false,
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
