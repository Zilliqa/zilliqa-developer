import Image from "next/image";
import { Geist, Geist_Mono } from "next/font/google";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export default function Home() {
  const connectZilPay = async () => {
        if (typeof window.zilPay === 'undefined') {
            console.log('Please install the ZilPay wallet extension.');
            return;
        }
        try {
            console.log('Connecting to ZilPay...');
            const isConnect = await window.zilPay.wallet.connect();
            if (isConnect && window.zilPay.wallet.defaultAccount) {
                // Ensure ZilPay bech32 address is in lowercase
                const account = window.zilPay.wallet.defaultAccount.bech32.toLowerCase();
                console.log('Address from ZilPay set successfully!', { account });
            } else {
                console.log('Could not connect to ZilPay.');
            }
        } catch (error) {
            console.error("ZilPay connection error:", error);
            console.log('ZilPay connection was rejected.');
        }
  }

  return (
    <div
      className={`${geistSans.className} ${geistMono.className} font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20`}
    >
      <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start">
        <h1 className="text-4xl font-bold">ZRC6 to ERC721 NFT Swap</h1>
        <p className="text-lg">
          Swap your ZRC6 tokens for ERC721 NFTs seamlessly.
        </p>

        <button
          className="px-6 py-3 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
          onClick={connectZilPay}
        >
          Connect ZilPay Wallet
        </button>

      </main>
      <footer className="row-start-3 flex gap-[24px] flex-wrap items-center justify-center">
        <div>
          ZRC6 to ERC721 NFT Swap application template by{" "}
          <a
            className="underline"
            href="https://zilliqa.com"
          >
            Zilliqa
          </a>
        </div>
      </footer>
    </div>
  );
}
