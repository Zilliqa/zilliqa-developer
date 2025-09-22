import React from "react"

interface TransactionStatusProps {
  isPreparation: boolean
  isProcessing: boolean
  txHash?: string
  error?: Error | { message: string }
  className?: string
}

const TransactionStatus: React.FC<TransactionStatusProps> = ({
  isPreparation,
  isProcessing,
  txHash,
  error,
  className = ""
}) => {
  if (isPreparation) {
    return (
      <div className={`flex items-center gap-2 text-yellow-600 ${className}`}>
        <div className="w-4 h-4 border-2 border-yellow-600 border-t-transparent rounded-full animate-spin"></div>
        <span>⏳ Preparing transaction...</span>
      </div>
    )
  }

  if (isProcessing) {
    return (
      <div className={`flex items-center gap-2 text-blue-600 ${className}`}>
        <div className="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
        <span>🔄 Processing transaction...</span>
      </div>
    )
  }

  if (error) {
    return (
      <div className={`flex items-center gap-2 text-red-600 ${className}`}>
        <span>❌ Transaction failed: {error.message || 'Unknown error'}</span>
      </div>
    )
  }

  if (txHash) {
    return (
      <div className={`flex items-center gap-2 text-green-600 ${className}`}>
        <span>✅ Transaction successful!</span>
        <a 
          href={`https://otterscan.testnet.zilliqa.com/tx/${txHash}`}
          target="_blank"
          rel="noopener noreferrer"
          className="text-blue-600 hover:text-blue-800 underline text-sm"
        >
          View on Explorer
        </a>
      </div>
    )
  }

  return null
}

export default TransactionStatus
