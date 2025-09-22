import {
  Wallet,
} from "@rainbow-me/rainbowkit"
import { hasInjectedProvider } from "../utils/connector"
import { BN } from "bn.js"
import Long from "long"

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

/**
 * Mints a new ZRC6 NFT using ZilPay
 * @param contractAddress The ZRC6 contract address
 * @param toAddress The recipient address (bech32 format)
 * @param tokenUri The token URI (optional, can be empty string)
 * @param gasLimit Gas limit for the transaction
 * @returns Promise<{transactionId: string}> The transaction result
 */
export async function mintZRC6NFT(
  contractAddress: string,
  toAddress: string,
  tokenUri: string = "",
  gasLimit: number = 50000
): Promise<{ transactionId: string }> {
  if (typeof window === 'undefined' || !window.zilPay) {
    throw new Error('ZilPay is not available')
  }

  if (!window.zilPay.contracts || !window.zilPay.utils) {
    throw new Error('ZilPay contract calling is not supported. Please update your ZilPay extension.')
  }

  if (!window.zilPay.wallet.isConnect || !window.zilPay.wallet.defaultAccount) {
    throw new Error('ZilPay wallet is not connected')
  }

  try {
    // Get contract instance
    const contract = window.zilPay.contracts.at(contractAddress);

    const MinimalGasPrice = new BN("3000000000");
    const DefaultGasLimit = Long.fromString("25000");

    const params = [
        {
            vname: "to",
            type: "ByStr20",
            value: `${toAddress}`,
        },
        {
            vname: "token_uri",
            type: "String",
            value: tokenUri,
        },
      ];
    
    const tx = await contract.call(
      'Mint',
      params,
      {
        amount: new BN(0),
        gasPrice: MinimalGasPrice,
        gasLimit: DefaultGasLimit
      }
    );

    if (!tx?.ID) {
      console.error('Transaction ID missing', { tx, error: contract.error });
      throw new Error('Transaction ID missing');
    }

    console.log('Transaction submitted', { transactionId: tx.ID });

    return {
      transactionId: tx.ID,
    };
  } catch (error) {
    console.error('Error minting ZRC6 NFT with ZilPay:', error);
    throw new Error('Failed to mint ZRC6 NFT');
  }
}
