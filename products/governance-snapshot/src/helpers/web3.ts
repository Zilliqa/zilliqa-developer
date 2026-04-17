import { Contract } from '@ethersproject/contracts';
import { getAddress } from '@ethersproject/address';
import resolveENSContentHash from '@/helpers/resolveENSContentHash';
import { decodeContenthash } from '@/helpers/content';
import abi from '@/helpers/abi';
import { zilliqa } from '@/helpers/zilliqa';
import { fromBech32Address } from '@zilliqa-js/zilliqa';
import { validation } from "@zilliqa-js/util";

export async function resolveContent(provider, name) {
  const contentHash = await resolveENSContentHash(name, provider);
  return decodeContenthash(contentHash);
}

export async function signMessage(web3: any, msg: string) {
  if (web3?.isEVM) {
    // personal_sign (EIP-191) via window.ethereum
    const accounts: string[] = await window['ethereum'].request({ method: 'eth_accounts' });
    const signature: string = await window['ethereum'].request({
      method: 'personal_sign',
      params: [msg, accounts[0]]
    });
    return { message: msg, signature };
  }
  // ZilPay Schnorr signing (existing)
  return await web3.wallet.sign(msg);
}

export async function getBlockNumber(): Promise<number> {
  try {
    const chainInfo: any = await zilliqa.blockchain.getBlockChainInfo();

    return parseInt(chainInfo.result.NumTxBlocks);
  } catch (e) {
    console.error("Error calling getBlockNumber ", e);
    return Promise.reject();
  }
}

export async function getTotalSupply(address: string): Promise<string> {
  const field = 'total_supply';
  const addr = validation.isBech32(address)
    ? fromBech32Address(address).toLowerCase()
    : address.toLowerCase();
  const res: any = await zilliqa.blockchain.getSmartContractSubState(
    addr,
    field
  );

  if (res && res['result'] && res['result']['total_supply']) {
    console.log('total_supply', res['result']['total_supply']);
    return res['result']['total_supply'];
  }

  console.error('Error calling getTotalSupply ', res);
  throw new Error('cannot fetch total_supply');
}

export async function sendTransaction(
  web3,
  [contractType, contractAddress, action, params]
) {
  const signer = web3.getSigner();
  const contract = new Contract(
    getAddress(contractAddress),
    abi[contractType],
    web3
  );
  const contractWithSigner = contract.connect(signer);
  const overrides = {};
  // overrides.gasLimit = 12e6;
  const tx = await contractWithSigner[action](...params, overrides);
  await tx.wait();
  return tx;
}
