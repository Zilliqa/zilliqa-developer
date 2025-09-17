export interface MockWallet {
  id: string
  name: string
  address: string
  balance: bigint
  description: string
}

export const mockWallets: MockWallet[] = [
  {
    id: "wallet-1",
    name: "Developer Wallet",
    address: "0x1234567890123456789012345678901234567890",
    balance: BigInt("1000000000000000000000"), // 1000 ZIL
    description: "High balance for testing large transactions",
  },
  {
    id: "wallet-2",
    name: "User Wallet", 
    address: "0x0987654321098765432109876543210987654321",
    balance: BigInt("100000000000000000000"), // 100 ZIL
    description: "Medium balance for typical user scenarios",
  },
  {
    id: "wallet-3",
    name: "Low Balance Wallet",
    address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
    balance: BigInt("1000000000000000000"), // 1 ZIL
    description: "Low balance for testing insufficient funds",
  },
]
