import { Contract } from '@ethersproject/contracts';
import { getAddress } from '@ethersproject/address';
import resolveENSContentHash from '@/helpers/resolveENSContentHash';
import { decodeContenthash } from '@/helpers/content';
import abi from '@/helpers/abi';
import { zilliqa } from '@/helpers/zilliqa';
import { fromBech32Address } from '@zilliqa-js/zilliqa';
import { validation } from '@zilliqa-js/util';
import { accountFromPublicKey, toAccount, Account } from '@/helpers/account';

export async function resolveContent(provider, name) {
  const contentHash = await resolveENSContentHash(name, provider);
  return decodeContenthash(contentHash);
}

export async function signMessage(
  web3: any,
  msg: string
): Promise<{ sig: any; signer: Account }> {
  if (web3?.isEVM) {
    // Sign with the wallet's currently-selected account. eth_requestAccounts
    // returns the active account (and re-prompts if the wallet is locked),
    // so the address we pass to personal_sign matches what the wallet signs
    // with — otherwise the wallet rejects with "Address mismatch".
    const accounts: string[] = await window['ethereum'].request({
      method: 'eth_requestAccounts'
    });
    if (!accounts || !accounts.length) {
      throw new Error(
        'No active account in your wallet. Unlock it and select an account, then try again.'
      );
    }
    const signerAddress = accounts[0];
    const signature: string = await window['ethereum'].request({
      method: 'personal_sign',
      params: [msg, signerAddress]
    });
    return {
      sig: { message: msg, signature },
      signer: toAccount(signerAddress)
    };
  }
  // ZilPay/Bearby Schnorr: the wallet signs with its active account; trust the
  // returned public key as the source of truth for who actually signed.
  const sig: any = await web3.wallet.sign(msg);
  if (!sig || !sig.publicKey) {
    throw new Error('Wallet did not return a public key for the signature.');
  }
  return { sig, signer: accountFromPublicKey(sig.publicKey) };
}

export async function getBlockNumber(): Promise<number> {
  try {
    const chainInfo: any = await zilliqa.blockchain.getBlockChainInfo();

    return parseInt(chainInfo.result.NumTxBlocks);
  } catch (e) {
    console.error('Error calling getBlockNumber ', e);
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
