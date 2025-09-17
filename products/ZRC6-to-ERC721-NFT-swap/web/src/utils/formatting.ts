import { formatUnits } from "viem"

export function formatAddress(address: string): string {
  if (!address) return ""
  return `${address.slice(0, 6)}...${address.slice(-4)}`
}

export function formatBalance(balance: bigint, decimals = 18, displayDecimals = 4): string {
  const formatted = formatUnits(balance, decimals)
  const num = parseFloat(formatted)
  return num.toFixed(displayDecimals)
}

export function formatZIL(balance: bigint): string {
  return formatBalance(balance, 18, 2)
}

export function validateAddress(address: string): boolean {
  return /^0x[a-fA-F0-9]{40}$/.test(address)
}

export function sanitizeAmount(amount: string): string {
  return amount.replace(/[^0-9.]/g, "")
}

export function debounce<T extends (...args: unknown[]) => unknown>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout
  
  return (...args: Parameters<T>) => {
    clearTimeout(timeout)
    timeout = setTimeout(() => func(...args), wait)
  }
}
