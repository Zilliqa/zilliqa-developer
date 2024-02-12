import { Zilliqa } from "@zilliqa-js/zilliqa";

const devPrivateKey =
  "3375F915F3F9AE35E6B301B7670F53AD1A5BE15D8221EC7FD5E503F21D3450C8";
const privateKey = process.env.RELAYER_PK || devPrivateKey;
const zilliqa = new Zilliqa("");

zilliqa.wallet.addByPrivateKey(privateKey);

const relayer = zilliqa.wallet.defaultAccount;

if (!relayer) {
  throw new Error("Incorect RELAYER_PK");
}

export default relayer;
