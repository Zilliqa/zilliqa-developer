import { parseUnits } from "ethers";

export const config = {
  zq: {
    tokenManager: "0x6D61eFb60C17979816E4cE12CD5D29054E755948",
    chainGateway: "0xE76669e1cCc150194eB92581baE79Ef6fa0E248E",
    token: "0x241c677D9969419800402521ae87C411897A029f",
    remoteToken: "0x351dA1E7500aBA1d168b9435DCE73415718d212F",
    remoteTokenManager: "0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23",
    remoteChainId: 56,
    remoteRecipient: "0xb34b88220Fa1EAeDba5d50b271AF8c3eE14A24Dd",
    amount: parseUnits("33185000", 12),
  },
  bsc: {
    tokenManager: "0xF391A1Ee7b3ccad9a9451D2B7460Ac646F899f23",
    chainGateway: "0x2114e979b7CFDd8b358502e00f50Fd5f7787Fe63",
    token: "0x351dA1E7500aBA1d168b9435DCE73415718d212F",
    remoteToken: "0x241c677D9969419800402521ae87C411897A029f",
    remoteTokenManager: "0x6D61eFb60C17979816E4cE12CD5D29054E755948",
    remoteChainId: 32769,
    remoteRecipient: "0x5807b970DB344d9d2386BbF5c3ec4cDA5CCdF1C8",
    amount: parseUnits("1", 12),
  },
};
