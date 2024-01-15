import { parseUnits } from "ethers";

export const config = {
  zq: {
    tokenManager: "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C",
    chainGateway: "0x18BCE81F9De993cdB2ebd680a44A8068B62D7f26",
    token: "0x63B6ebD476C84bFDd5DcaCB3f974794FC6C2e721",
    remoteToken: "0x6d78c86D66DfE5Be5F55FBAA8B1d3FD28edfF396",
    remoteTokenManager: "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C",
    remoteChainId: 97,
    remoteRecipient: "0xb494D016F2CF329224e2dB445aA748Cf96C18C29",
    amount: parseUnits("100", 12),
  },
  bsc: {
    tokenManager: "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C",
    chainGateway: "0x5cE584e24f6703f3197Ca83d442807cB82474f8D",
    token: "`0x6d78c86D66DfE5Be5F55FBAA8B1d3FD28edfF396`",
    remoteToken: "0x63B6ebD476C84bFDd5DcaCB3f974794FC6C2e721",
    remoteTokenManager: "0xd10077bCE4A9D19068965dE519CED8a2fC1B096C",
    remoteChainId: 33101,
    remoteRecipient: "0xb494D016F2CF329224e2dB445aA748Cf96C18C29",
    amount: parseUnits("100", 12),
  },
};
