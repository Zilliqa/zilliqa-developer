import fetch from "node-fetch";
import pinataSDK from "@pinata/sdk";

const pinata = pinataSDK(
  String(process.env.PINATA_API_KEY),
  String(process.env.PINATA_SECRET_API_KEY)
);

export async function pinJson(body: object) {
  let ipfsHash: string;

  const result = await pinata.pinJSONToIPFS(body);
  ipfsHash = result.IpfsHash;

  fetch(`https://gateway.pinata.cloud/ipfs/${ipfsHash}`)
    .then((res) => res.json())
    .then((json) => console.log("Arweave success", ipfsHash))
    .catch((e) => console.error("Arweave error", e));

  return ipfsHash;
}
