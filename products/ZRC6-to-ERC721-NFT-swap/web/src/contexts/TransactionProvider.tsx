import { useState, useEffect, useCallback } from "react"
import { useWriteContract, useWaitForTransactionReceipt, useGasPrice, useConfig } from "wagmi"
import { Address, WriteContractParameters, formatUnits, Abi } from "viem"
import { createContainer } from "../utils/context"
import { useWallet, ConnectedWalletType } from "../context/WalletContext"

const useTransactionProvider = () => {
  const { connectedWalletType, walletAddress } = useWallet()
  const wagmiConfig = useConfig()
  const [txHash, setTxHash] = useState<Address | undefined>(undefined)
  const [isTxInPreparation, setIsTxInPreparation] = useState(false)

  const {
    isLoading: isTxProcessedByChain,
    error: txContractError,
    status: txReceiptStatus,
  } = useWaitForTransactionReceipt({ hash: txHash })

  const {
    writeContract,
    status: txSubmissionStatus,
    error: txSubmissionError,
    data: currentTxData,
  } = useWriteContract()

  const { data: reportedGasPrice } = useGasPrice()

  // Apply 25% buffer to reported gas price
  const adjustedGasPrice = ((reportedGasPrice || BigInt(0)) * BigInt(125)) / BigInt(100)

  const getGasCostInZil = useCallback((estimatedGas: bigint) => {
    return Math.ceil(parseFloat(formatUnits(estimatedGas * adjustedGasPrice, 18)))
  }, [adjustedGasPrice])

  const callContract = useCallback((
    txCallParams: WriteContractParameters,
    successMessage = "Transaction successful",
    errorMessage = "Transaction failed",
    onSuccess?: () => void
  ) => {
    setIsTxInPreparation(true)
    setTxHash(undefined)

    if (connectedWalletType === ConnectedWalletType.MockWallet) {
      // Mock transaction simulation
      setTimeout(() => {
        const mockTxHash = `0x${Math.random().toString(16).slice(2).padStart(64, '0')}` as Address
        setTxHash(mockTxHash)
        setIsTxInPreparation(false)
        console.log(successMessage, { txHash: mockTxHash })
        if (onSuccess) {
          setTimeout(onSuccess, 1000) // Simulate block confirmation delay
        }
      }, 2000)
    } else {
      try {
        writeContract(txCallParams)
      } catch (error) {
        console.error(errorMessage, error)
        setIsTxInPreparation(false)
      }
    }
  }, [connectedWalletType, writeContract])

  // Handle real transaction status updates
  useEffect(() => {
    if (txReceiptStatus === "success") {
      console.log("Transaction confirmed on chain")
    }
  }, [txReceiptStatus])

  useEffect(() => {
    if (txSubmissionStatus === "success") {
      setTxHash(currentTxData)
      setIsTxInPreparation(false)
    }
  }, [txSubmissionStatus, currentTxData])

  useEffect(() => {
    if (txSubmissionStatus === "error") {
      console.error("Transaction submission failed:", txSubmissionError)
      setIsTxInPreparation(false)
    }
  }, [txSubmissionStatus, txSubmissionError])

  // Generic contract interaction helpers
  const transferTokens = useCallback((
    tokenAddress: string,
    to: string,
    amount: bigint,
    abi: Abi
  ) => {    
    if (!walletAddress) {
      throw new Error("Wallet not connected")
    }

    callContract({
      address: tokenAddress as Address,
      abi,
      functionName: "transfer",
      args: [to as Address, amount],
      account: walletAddress as Address,
      chain: wagmiConfig.chains[0],
    }, "Token transfer successful", "Token transfer failed")
  }, [callContract, walletAddress, wagmiConfig])

  const callContractFunction = useCallback((
    contractAddress: string,
    abi: Abi,
    functionName: string,
    args: readonly unknown[] = [],
    value?: bigint,
    onSuccess?: () => void
  ) => {
    if (!walletAddress) {
      throw new Error("Wallet not connected")
    }

    callContract({
      address: contractAddress as Address,
      abi,
      functionName,
      args,
      value,
      account: walletAddress as Address,
      chain: wagmiConfig.chains[0],
    }, `${functionName} successful`, `${functionName} failed`, onSuccess)
  }, [callContract, walletAddress, wagmiConfig])

  return {
    // Transaction state
    isTxInPreparation: isTxInPreparation || txSubmissionStatus === "pending",
    isTxProcessedByChain: isTxProcessedByChain && connectedWalletType !== ConnectedWalletType.MockWallet,
    txHash,
    txContractError,
    txSubmissionError,
    
    // Gas calculation
    getGasCostInZil,
    adjustedGasPrice,
    
    // Transaction methods
    callContract,
    callContractFunction,
    transferTokens,
  }
}

export const TransactionProvider = createContainer(useTransactionProvider)

export function useTransactions() {
  return TransactionProvider.useContainer()
}
