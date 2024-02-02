export const MintAndBurnTokenManagerAbi = [
  {
    type: "function",
    name: "accept",
    inputs: [
      {
        name: "metadata",
        type: "tuple",
        internalType: "struct CallMetadata",
        components: [
          { name: "sourceChainId", type: "uint256", internalType: "uint256" },
          { name: "sender", type: "address", internalType: "address" },
        ],
      },
      { name: "args", type: "bytes", internalType: "bytes" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "getGateway",
    inputs: [],
    outputs: [{ name: "", type: "address", internalType: "address" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "getRemoteTokens",
    inputs: [
      { name: "token", type: "address", internalType: "address" },
      { name: "remoteChainId", type: "uint256", internalType: "uint256" },
    ],
    outputs: [
      {
        name: "",
        type: "tuple",
        internalType: "struct ITokenManagerStructs.RemoteToken",
        components: [
          { name: "token", type: "address", internalType: "address" },
          { name: "tokenManager", type: "address", internalType: "address" },
          { name: "chainId", type: "uint256", internalType: "uint256" },
        ],
      },
    ],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "registerToken",
    inputs: [
      { name: "token", type: "address", internalType: "address" },
      {
        name: "remoteToken",
        type: "tuple",
        internalType: "struct ITokenManagerStructs.RemoteToken",
        components: [
          { name: "token", type: "address", internalType: "address" },
          { name: "tokenManager", type: "address", internalType: "address" },
          { name: "chainId", type: "uint256", internalType: "uint256" },
        ],
      },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "setGateway",
    inputs: [{ name: "_gateway", type: "address", internalType: "address" }],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "function",
    name: "transfer",
    inputs: [
      { name: "token", type: "address", internalType: "address" },
      { name: "remoteChainId", type: "uint256", internalType: "uint256" },
      { name: "remoteRecipient", type: "address", internalType: "address" },
      { name: "amount", type: "uint256", internalType: "uint256" },
    ],
    outputs: [],
    stateMutability: "nonpayable",
  },
  {
    type: "event",
    name: "Locked",
    inputs: [
      {
        name: "token",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      { name: "from", type: "address", indexed: true, internalType: "address" },
      {
        name: "amount",
        type: "uint256",
        indexed: false,
        internalType: "uint256",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "Released",
    inputs: [
      {
        name: "token",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      {
        name: "recipient",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      {
        name: "amount",
        type: "uint256",
        indexed: false,
        internalType: "uint256",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "TokenRegistered",
    inputs: [
      {
        name: "token",
        type: "address",
        indexed: true,
        internalType: "address",
      },
      {
        name: "remoteToken",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "remoteTokenManager",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "remoteChainId",
        type: "uint256",
        indexed: false,
        internalType: "uint256",
      },
    ],
    anonymous: false,
  },
  {
    type: "event",
    name: "TokenRemoved",
    inputs: [
      {
        name: "token",
        type: "address",
        indexed: false,
        internalType: "address",
      },
      {
        name: "remoteChainId",
        type: "uint256",
        indexed: false,
        internalType: "uint256",
      },
    ],
    anonymous: false,
  },
  { type: "error", name: "InvalidSourceChainId", inputs: [] },
  { type: "error", name: "InvalidTokenManager", inputs: [] },
  { type: "error", name: "NotGateway", inputs: [] },
] as const;
