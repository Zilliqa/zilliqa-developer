import { parseUnits } from "ethers";

export const config = {
  zq: {
    tokenManager: "0x3Be6E686397f04901Be15e3e02EDC0c7565e4b13",
    chainGateway: "0xF74C8a0AF3B03d7135C7fFb816774f24d0053A3B",
    token: "0x95ebe761b40042F23b717e1e00ECF6b871f24173",
    remoteToken: "0x37595dC4dDe8c43A5c80541c3ceF7c6Cc9A89867",
    remoteTokenManager: "0xf42aa5b0D9B14f37c5de088178DA68DF841879E1",
    remoteChainId: 56,
    remoteRecipient: "0xAAF33a7e4756D097B2158551a25374Daf600c49D",
    amount: parseUnits("1", 8),
  },
  bsc: {
    tokenManager: "0xf42aa5b0D9B14f37c5de088178DA68DF841879E1",
    chainGateway: "0x3fa391E5a4c1b55D04A1b164fDC67ECEb312B93d",
    token: "0x37595dC4dDe8c43A5c80541c3ceF7c6Cc9A89867",
    remoteToken: "0x95ebe761b40042F23b717e1e00ECF6b871f24173",
    remoteTokenManager: "0x3Be6E686397f04901Be15e3e02EDC0c7565e4b13",
    remoteChainId: 32769,
    remoteRecipient: "0xAAF33a7e4756D097B2158551a25374Daf600c49D",
    amount: parseUnits("1", 8),
  },
};
