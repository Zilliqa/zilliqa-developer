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
