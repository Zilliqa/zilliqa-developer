import { ConnectButton } from "@rainbow-me/rainbowkit";
import zilliqa from "../assets/zilliqa.png";

export default function Navbar() {
  return (
    <div className="fixed top-0 navbar py-6 px-10">
      <div className="flex-1 hidden sm:block">
        <img src={zilliqa} className="h-16" alt="Zilliqa Logo" />
      </div>
      <div className="flex-none">
        <ConnectButton />
      </div>
    </div>
  );
}
