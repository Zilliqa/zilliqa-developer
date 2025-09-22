import { NextApiRequest, NextApiResponse } from "next"

export interface AppConfig {
  chainId: number
  walletConnectProjectId: string
  appUrl: string
  appName: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<AppConfig>
) {
  const config: AppConfig = {
    chainId: parseInt(process.env.NEXT_PUBLIC_CHAIN_ID || "33101"),
    walletConnectProjectId: process.env.NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID || "",
    appUrl: process.env.NEXT_PUBLIC_APP_URL || "http://localhost:3000",
    appName: process.env.NEXT_PUBLIC_APP_NAME || "ZRC6 to ERC721 NFT Swap",
  }

  res.status(200).json(config)
}
