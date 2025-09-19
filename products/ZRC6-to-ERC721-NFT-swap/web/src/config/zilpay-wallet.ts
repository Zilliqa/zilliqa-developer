import {
  Wallet,
} from "@rainbow-me/rainbowkit"
import { hasInjectedProvider } from "../utils/connector"

declare global {
  interface Window {
    zilPay?: {
      wallet: {
        isConnect: boolean;
        defaultAccount: {
          bech32: string;
          base16: string;
        } | null;
        connect: () => Promise<boolean>;
        signMessage?: (message: Uint8Array) => Promise<string>;
      };
    };
  }
}

export const zilPayWallet = (): Wallet => {
  const isZilPayInjected = hasInjectedProvider({
    flag: "isZilPay",
  })

  return {
    id: "zilpay",
    name: "ZilPay",
    rdns: "io.zilpay",
    iconUrl: async () => "https://zilpay.io/favicon.ico",
    iconBackground: "#ffffff",
    installed: isZilPayInjected,
    downloadUrls: {
      android: "https://play.google.com/store/apps/details?id=com.zilpaymobile",
      ios: "https://apps.apple.com/app/zilpay/id1547105860",
      mobile: "https://zilpay.io/",
      qrCode: "https://zilpay.io/",
    },
    createConnector: () => {
      // This would be a custom implementation for ZilPay
      // For now, we'll handle ZilPay connections separately in the context
      throw new Error("ZilPay should be handled through the wallet context")
    },
  }
}

/**
 * Signs a message using ZilPay wallet
 * @param message The message to sign
 * @returns Promise<string> The signature
 */
export async function signMessageWithZilPay(message: string): Promise<string> {
  if (typeof window === 'undefined' || !window.zilPay) {
    throw new Error('ZilPay is not available')
  }

  if (!window.zilPay.wallet.signMessage) {
    throw new Error('ZilPay signing is not supported. Please update your ZilPay extension.')
  }

  try {
    // Convert message to bytes for signing
    const messageBytes = new TextEncoder().encode(message)

    // Use ZilPay's signing functionality
    const signature = await window.zilPay.wallet.signMessage(messageBytes)

    return signature
  } catch (error) {
    console.error('Error signing message with ZilPay:', error)
    throw new Error('Failed to sign message with ZilPay')
  }
}

/**
 * Creates a signature for the EVM wallet address using ZilPay
 * @param evmAddress The EVM wallet address to sign
 * @returns Promise<string> The signature
 */
export async function createEvmAddressSignature(evmAddress: string): Promise<string> {
  // Create the message to sign - just the EVM address
  const message = evmAddress.toLowerCase()

  return await signMessageWithZilPay(message)
}
