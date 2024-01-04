/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import {
  Contract,
  ContractFactory,
  ContractTransactionResponse,
  Interface,
} from "ethers";
import type { Signer, ContractDeployTransaction, ContractRunner } from "ethers";
import type { NonPayableOverrides } from "../../../common";
import type {
  Target,
  TargetInterface,
} from "../../../contracts/Test.sol/Target";

const _abi = [
  {
    inputs: [
      {
        internalType: "uint256",
        name: "num",
        type: "uint256",
      },
    ],
    name: "test",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "pure",
    type: "function",
  },
] as const;

const _bytecode =
  "0x608060405234801561001057600080fd5b506101f9806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c806329e99f0714610030575b600080fd5b61004361003e36600461013b565b610055565b60405190815260200160405180910390f35b600061007e6040518060400160405280600681526020016574657374282960d01b8152506100d0565b6103e882106100bf5760405162461bcd60e51b8152602060048201526009602482015268546f6f206c6172676560b81b604482015260640160405180910390fd5b6100ca826001610154565b92915050565b610113816040516024016100e49190610175565b60408051601f198184030181529190526020810180516001600160e01b031663104c13eb60e21b179052610116565b50565b6101138160006a636f6e736f6c652e6c6f679050600080835160208501845afa505050565b60006020828403121561014d57600080fd5b5035919050565b808201808211156100ca57634e487b7160e01b600052601160045260246000fd5b600060208083528351808285015260005b818110156101a257858101830151858201604001528201610186565b506000604082860101526040601f19601f830116850101925050509291505056fea26469706673582212203779ddfc0af1c517538b0f0ab55ff6c84c25f3912af64534d4944852dce89ed464736f6c63430008140033";

type TargetConstructorParams =
  | [signer?: Signer]
  | ConstructorParameters<typeof ContractFactory>;

const isSuperArgs = (
  xs: TargetConstructorParams
): xs is ConstructorParameters<typeof ContractFactory> => xs.length > 1;

export class Target__factory extends ContractFactory {
  constructor(...args: TargetConstructorParams) {
    if (isSuperArgs(args)) {
      super(...args);
    } else {
      super(_abi, _bytecode, args[0]);
    }
  }

  override getDeployTransaction(
    overrides?: NonPayableOverrides & { from?: string }
  ): Promise<ContractDeployTransaction> {
    return super.getDeployTransaction(overrides || {});
  }
  override deploy(overrides?: NonPayableOverrides & { from?: string }) {
    return super.deploy(overrides || {}) as Promise<
      Target & {
        deploymentTransaction(): ContractTransactionResponse;
      }
    >;
  }
  override connect(runner: ContractRunner | null): Target__factory {
    return super.connect(runner) as Target__factory;
  }

  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): TargetInterface {
    return new Interface(_abi) as TargetInterface;
  }
  static connect(address: string, runner?: ContractRunner | null): Target {
    return new Contract(address, _abi, runner) as unknown as Target;
  }
}
