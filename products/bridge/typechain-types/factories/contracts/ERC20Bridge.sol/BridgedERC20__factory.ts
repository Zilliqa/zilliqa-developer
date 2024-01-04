/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import {
  Contract,
  ContractFactory,
  ContractTransactionResponse,
  Interface,
} from "ethers";
import type {
  Signer,
  AddressLike,
  ContractDeployTransaction,
  ContractRunner,
} from "ethers";
import type { NonPayableOverrides } from "../../../common";
import type {
  BridgedERC20,
  BridgedERC20Interface,
} from "../../../contracts/ERC20Bridge.sol/BridgedERC20";

const _abi = [
  {
    inputs: [
      {
        internalType: "string",
        name: "name_",
        type: "string",
      },
      {
        internalType: "string",
        name: "symbol_",
        type: "string",
      },
      {
        internalType: "address",
        name: "bridge_",
        type: "address",
      },
    ],
    stateMutability: "nonpayable",
    type: "constructor",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "spender",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "allowance",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "needed",
        type: "uint256",
      },
    ],
    name: "ERC20InsufficientAllowance",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "sender",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "balance",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "needed",
        type: "uint256",
      },
    ],
    name: "ERC20InsufficientBalance",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "approver",
        type: "address",
      },
    ],
    name: "ERC20InvalidApprover",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "receiver",
        type: "address",
      },
    ],
    name: "ERC20InvalidReceiver",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "sender",
        type: "address",
      },
    ],
    name: "ERC20InvalidSender",
    type: "error",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "spender",
        type: "address",
      },
    ],
    name: "ERC20InvalidSpender",
    type: "error",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "owner",
        type: "address",
      },
      {
        indexed: true,
        internalType: "address",
        name: "spender",
        type: "address",
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "Approval",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: "address",
        name: "from",
        type: "address",
      },
      {
        indexed: true,
        internalType: "address",
        name: "to",
        type: "address",
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "Transfer",
    type: "event",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "owner",
        type: "address",
      },
      {
        internalType: "address",
        name: "spender",
        type: "address",
      },
    ],
    name: "allowance",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "spender",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "approve",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "account",
        type: "address",
      },
    ],
    name: "balanceOf",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "burn",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "from",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "amount",
        type: "uint256",
      },
    ],
    name: "burn",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "account",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "burnFrom",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "decimals",
    outputs: [
      {
        internalType: "uint8",
        name: "",
        type: "uint8",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "to",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "amount",
        type: "uint256",
      },
    ],
    name: "mint",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "name",
    outputs: [
      {
        internalType: "string",
        name: "",
        type: "string",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "symbol",
    outputs: [
      {
        internalType: "string",
        name: "",
        type: "string",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "totalSupply",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "to",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "transfer",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "from",
        type: "address",
      },
      {
        internalType: "address",
        name: "to",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "value",
        type: "uint256",
      },
    ],
    name: "transferFrom",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "nonpayable",
    type: "function",
  },
] as const;

const _bytecode =
  "0x60806040523480156200001157600080fd5b5060405162000de138038062000de18339810160408190526200003491620002c2565b82826003620000448382620003de565b506004620000538282620003de565b5050600580546001600160a01b0319166001600160a01b038416179055506200007f336103e862000088565b505050620004d2565b6001600160a01b038216620000b85760405163ec442f0560e01b8152600060048201526024015b60405180910390fd5b620000c660008383620000ca565b5050565b6001600160a01b038316620000f9578060026000828254620000ed9190620004aa565b909155506200016d9050565b6001600160a01b038316600090815260208190526040902054818110156200014e5760405163391434e360e21b81526001600160a01b03851660048201526024810182905260448101839052606401620000af565b6001600160a01b03841660009081526020819052604090209082900390555b6001600160a01b0382166200018b57600280548290039055620001aa565b6001600160a01b03821660009081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef83604051620001f091815260200190565b60405180910390a3505050565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126200022557600080fd5b81516001600160401b0380821115620002425762000242620001fd565b604051601f8301601f19908116603f011681019082821181831017156200026d576200026d620001fd565b816040528381526020925086838588010111156200028a57600080fd5b600091505b83821015620002ae57858201830151818301840152908201906200028f565b600093810190920192909252949350505050565b600080600060608486031215620002d857600080fd5b83516001600160401b0380821115620002f057600080fd5b620002fe8783880162000213565b945060208601519150808211156200031557600080fd5b50620003248682870162000213565b604086015190935090506001600160a01b03811681146200034457600080fd5b809150509250925092565b600181811c908216806200036457607f821691505b6020821081036200038557634e487b7160e01b600052602260045260246000fd5b50919050565b601f821115620003d957600081815260208120601f850160051c81016020861015620003b45750805b601f850160051c820191505b81811015620003d557828155600101620003c0565b5050505b505050565b81516001600160401b03811115620003fa57620003fa620001fd565b62000412816200040b84546200034f565b846200038b565b602080601f8311600181146200044a5760008415620004315750858301515b600019600386901b1c1916600185901b178555620003d5565b600085815260208120601f198616915b828110156200047b578886015182559484019460019091019084016200045a565b50858210156200049a5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b80820180821115620004cc57634e487b7160e01b600052601160045260246000fd5b92915050565b6108ff80620004e26000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c806342966c681161008c57806395d89b411161006657806395d89b41146101ad5780639dc29fac146101b5578063a9059cbb146101c8578063dd62ed3e146101db57600080fd5b806342966c681461015e57806370a082311461017157806379cc67901461019a57600080fd5b806306fdde03146100d4578063095ea7b3146100f257806318160ddd1461011557806323b872dd14610127578063313ce5671461013a57806340c10f1914610149575b600080fd5b6100dc610214565b6040516100e99190610730565b60405180910390f35b61010561010036600461079a565b6102a6565b60405190151581526020016100e9565b6002545b6040519081526020016100e9565b6101056101353660046107c4565b6102c0565b604051601281526020016100e9565b61015c61015736600461079a565b6102e4565b005b61015c61016c366004610800565b610342565b61011961017f366004610819565b6001600160a01b031660009081526020819052604090205490565b61015c6101a836600461079a565b61034f565b6100dc610364565b61015c6101c336600461079a565b610373565b6101056101d636600461079a565b6103c8565b6101196101e936600461083b565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6060600380546102239061086e565b80601f016020809104026020016040519081016040528092919081815260200182805461024f9061086e565b801561029c5780601f106102715761010080835404028352916020019161029c565b820191906000526020600020905b81548152906001019060200180831161027f57829003601f168201915b5050505050905090565b6000336102b48185856103d6565b60019150505b92915050565b6000336102ce8582856103e8565b6102d9858585610466565b506001949350505050565b6005546001600160a01b031633146103345760405162461bcd60e51b815260206004820152600e60248201526d4e6f74207468652062726964676560901b60448201526064015b60405180910390fd5b61033e82826104c5565b5050565b61034c33826104fb565b50565b61035a8233836103e8565b61033e82826104fb565b6060600480546102239061086e565b6005546001600160a01b031633146103be5760405162461bcd60e51b815260206004820152600e60248201526d4e6f74207468652062726964676560901b604482015260640161032b565b61033e828261034f565b6000336102b4818585610466565b6103e38383836001610531565b505050565b6001600160a01b038381166000908152600160209081526040808320938616835292905220546000198114610460578181101561045157604051637dc7a0d960e11b81526001600160a01b0384166004820152602481018290526044810183905260640161032b565b61046084848484036000610531565b50505050565b6001600160a01b03831661049057604051634b637e8f60e11b81526000600482015260240161032b565b6001600160a01b0382166104ba5760405163ec442f0560e01b81526000600482015260240161032b565b6103e3838383610606565b6001600160a01b0382166104ef5760405163ec442f0560e01b81526000600482015260240161032b565b61033e60008383610606565b6001600160a01b03821661052557604051634b637e8f60e11b81526000600482015260240161032b565b61033e82600083610606565b6001600160a01b03841661055b5760405163e602df0560e01b81526000600482015260240161032b565b6001600160a01b03831661058557604051634a1406b160e11b81526000600482015260240161032b565b6001600160a01b038085166000908152600160209081526040808320938716835292905220829055801561046057826001600160a01b0316846001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516105f891815260200190565b60405180910390a350505050565b6001600160a01b03831661063157806002600082825461062691906108a8565b909155506106a39050565b6001600160a01b038316600090815260208190526040902054818110156106845760405163391434e360e21b81526001600160a01b0385166004820152602481018290526044810183905260640161032b565b6001600160a01b03841660009081526020819052604090209082900390555b6001600160a01b0382166106bf576002805482900390556106de565b6001600160a01b03821660009081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8360405161072391815260200190565b60405180910390a3505050565b600060208083528351808285015260005b8181101561075d57858101830151858201604001528201610741565b506000604082860101526040601f19601f8301168501019250505092915050565b80356001600160a01b038116811461079557600080fd5b919050565b600080604083850312156107ad57600080fd5b6107b68361077e565b946020939093013593505050565b6000806000606084860312156107d957600080fd5b6107e28461077e565b92506107f06020850161077e565b9150604084013590509250925092565b60006020828403121561081257600080fd5b5035919050565b60006020828403121561082b57600080fd5b6108348261077e565b9392505050565b6000806040838503121561084e57600080fd5b6108578361077e565b91506108656020840161077e565b90509250929050565b600181811c9082168061088257607f821691505b6020821081036108a257634e487b7160e01b600052602260045260246000fd5b50919050565b808201808211156102ba57634e487b7160e01b600052601160045260246000fdfea26469706673582212200307b5a397ba870a7fb9851b8b1563c851af8457c239e4aa8b7011b5cf1f376264736f6c63430008140033";

type BridgedERC20ConstructorParams =
  | [signer?: Signer]
  | ConstructorParameters<typeof ContractFactory>;

const isSuperArgs = (
  xs: BridgedERC20ConstructorParams
): xs is ConstructorParameters<typeof ContractFactory> => xs.length > 1;

export class BridgedERC20__factory extends ContractFactory {
  constructor(...args: BridgedERC20ConstructorParams) {
    if (isSuperArgs(args)) {
      super(...args);
    } else {
      super(_abi, _bytecode, args[0]);
    }
  }

  override getDeployTransaction(
    name_: string,
    symbol_: string,
    bridge_: AddressLike,
    overrides?: NonPayableOverrides & { from?: string }
  ): Promise<ContractDeployTransaction> {
    return super.getDeployTransaction(name_, symbol_, bridge_, overrides || {});
  }
  override deploy(
    name_: string,
    symbol_: string,
    bridge_: AddressLike,
    overrides?: NonPayableOverrides & { from?: string }
  ) {
    return super.deploy(name_, symbol_, bridge_, overrides || {}) as Promise<
      BridgedERC20 & {
        deploymentTransaction(): ContractTransactionResponse;
      }
    >;
  }
  override connect(runner: ContractRunner | null): BridgedERC20__factory {
    return super.connect(runner) as BridgedERC20__factory;
  }

  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): BridgedERC20Interface {
    return new Interface(_abi) as BridgedERC20Interface;
  }
  static connect(
    address: string,
    runner?: ContractRunner | null
  ): BridgedERC20 {
    return new Contract(address, _abi, runner) as unknown as BridgedERC20;
  }
}
