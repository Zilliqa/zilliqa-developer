import fetch from "node-fetch";
import pinataSDK from "@pinata/sdk";

export async function pinJson(body: object): Promise<string> {
  if (process.env.IPFS_API_URL) {
    return pinToLocalNode(body);
  }

  const pinata = pinataSDK(
    String(process.env.PINATA_API_KEY),
    String(process.env.PINATA_SECRET_API_KEY)
  );
  const result = await pinata.pinJSONToIPFS(body);
  return result.IpfsHash;
}

async function pinToLocalNode(body: object): Promise<string> {
  const json = JSON.stringify(body);
  const boundary = `----IpfsBoundary${Date.now().toString(16)}`;
  const crlf = "\r\n";
  const multipart = [
    `--${boundary}`,
    'Content-Disposition: form-data; name="file"; filename="data.json"',
    "Content-Type: application/json",
    "",
    json,
    `--${boundary}--`,
  ].join(crlf);

  const response = await fetch(
    `${process.env.IPFS_API_URL}/api/v0/add?pin=true`,
    {
      method: "POST",
      headers: {
        "Content-Type": `multipart/form-data; boundary=${boundary}`,
      },
      body: multipart,
    }
  );

  if (!response.ok) {
    throw new Error(
      `IPFS add failed: ${response.status} ${await response.text()}`
    );
  }

  const data = (await response.json()) as { Hash: string };
  console.log("IPFS pin success", data.Hash);
  return data.Hash;
}
