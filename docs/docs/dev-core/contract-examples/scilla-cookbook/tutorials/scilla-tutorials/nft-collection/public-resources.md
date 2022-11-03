---
sidebar_position: 3
---

# Making resources public

For our example, lets say we have some .mp4 files we want to distribute.

Let's consider some options of where we could host these files.

## Surface Web

AWS, Google, and any Linux web server have the advantage of being easily accessible by the general public and the speeds of fetching an image is relatively quick and easy depending on the network speed of you and the image service provider.

These public environments have the potential to potentially go offline for maintenance or user error. Public servers also have the ability to change what image is shown is also possible by replacing the image on the server providing the content.

Example URL : ```https://mybucket.s3.amazonaws.com/myimage.jpg```

## IPFS

A popular alternative is IPFS. When you add a file to IPFS, your file is split into smaller chunks, cryptographically hashed, and given a unique fingerprint called a content identifier (CID). This CID acts as an permanent record of your file as it exists at that point in time.

When other nodes look up your file, they ask their peer nodes who's storing the content referenced by the file's CID. When they view or download your file, they cache a copy — and become another provider of your content until their cache is cleared.

A node can pin content in order to keep (and provide) it forever, or discard content it hasn't used in a while to save space. This means each node in the network stores only content it is interested in, plus some indexing information that helps figure out which node is storing what.

If you add a new version of your file to IPFS, its cryptographic hash is different, and so it gets a new CID. This means files stored on IPFS are resistant to tampering and censorship — any changes to a file don't overwrite the original, and common chunks across files can be reused in order to minimize storage costs.

An IPFS URL can be fetched from a specific provider using a gateway URL and an IPFS hash. In the below case, the gateway is ```https://permaweb.eu.org/ipfs/``` and the IPFS CID is ```QmSjJGa7zXUbixvYJpgkRkaSCYEBtSwgVtfzkoD3QkSsty```. This pattern of coupling a gateway to a CID is restrictive as the CID can be fetched from any valid gateway.

:::danger
Gateway defined URL : ```https://permaweb.eu.org/ipfs/QmSjJGa7zXUbixvYJpgkRkaSCYEBtSwgVtfzkoD3QkSsty```
:::

If a gateway is used then this can lead to performance issues when rendering your resources on ecosystem dapps, as the gateway has a limit to how much data you can request in a given time.

We can tell ecosystem partners we have a file on IPFS with the syntax ```ipfs://```. Ecosystem providers will use any provider they wish this way, including their own IPFS server. This reduces the coupling between a gateway and a resource.

:::tip
Any gateway can be used URL :```ipfs://QmSjJGa7zXUbixvYJpgkRkaSCYEBtSwgVtfzkoD3QkSsty```
:::

## Arweave

Lastly, Arweave is a worldwide community-owned permaweb. Anyone can contribute to the permaweb to maintain it and receive payment for doing so. On the surface, the permaweb appears to be just like the normal web.

All content the permaweb holds is permanent, decentralized, and easily retrievable. This includes images, text, applications, and just about anything else that lives on the web.

:::danger
Gateway defined URL : ```https://gacwaegdw6vfvhx2uhgvntjuvigda3bg7iky3jgxtpet66abfvxa.arweave.net/MAVgEMO3qlqe-qHNVs00qgwwbCb6FY2k15vJP3gBLW4```
:::

We can tell ecosystem partners we have a file on Arweave with the syntax ```ar://```. Ecosystem providers will use any provider they wish this way, including their own Arweave server.

:::tip
Any gateway can be used URL :```ar://MAVgEMO3qlqe-qHNVs00qgwwbCb6FY2k15vJP3gBLW4```
:::

## Points to consider

If you use a non decentralised type of image storage and if payment was to be suspended, the provider would stop hosting your images. Typically this leads to deadlinks  and this isn't appealing to buy an asset which has a resource potentially returning a 404 in the future. Decentralised hosting ensures that as long as a node somewhere is pinning that asset then it will exist on the network to be served.

## NFTCollection

After you've chosen a storage location, uploaded your resource content and have available a link to access that resource publicly over the internet. In the next step we can begin to create the metadata which contains a link to our public .mp4 asset.

## Further reading

[IPFS](https://ipfs.io/)

[Arweave](https://www.arweave.org/)
