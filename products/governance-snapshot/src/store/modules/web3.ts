import Vue from 'vue';
import { getInstance } from '@/helpers/plugins/LockPlugin';
import store from '@/store';
import config from '@/helpers/config';
import { toAccount, addressesMatch, Account } from '@/helpers/account';

let wsProvider;
let auth;

if (wsProvider) {
  wsProvider.on('block', blockNumber => {
    store.commit('GET_BLOCK_SUCCESS', blockNumber);
  });
}

const state = {
  account: {
    base16: '',
    bech32: ''
  },
  name: null,
  network: config.networks['mainnet'],
  isEVM: false
};

const mutations = {
  LOGOUT(_state) {
    Vue.set(_state, 'account', { base16: '', bech32: '' });
    Vue.set(_state, 'name', null);
    Vue.set(_state, 'isEVM', false);
    console.debug('LOGOUT');
  },
  LOAD_PROVIDER_REQUEST() {
    console.debug('LOAD_PROVIDER_REQUEST');
  },
  LOAD_PROVIDER_SUCCESS(_state, payload) {
    console.log('LOAD_PROVIDER_SUCCESS', payload);
    Vue.set(_state, 'account', payload.account);
    Vue.set(_state, 'name', payload.name);
    console.debug('LOAD_PROVIDER_SUCCESS');
  },
  LOAD_PROVIDER_FAILURE(_state, payload) {
    Vue.set(_state, 'account', { base16: '', bech32: '' });
    console.debug('LOAD_PROVIDER_FAILURE', payload);
  },
  HANDLE_CHAIN_CHANGED(_state, net) {
    if (net) {
      config.networks[net] = {
        ...config.networks[net],
        net,
        chainId: 0,
        name: net,
        network: net
      };
    }
    Vue.set(_state, 'network', config.networks[net]);
    console.debug('HANDLE_CHAIN_CHANGED', net);
  },
  HANDLE_ACCOUNTS_CHANGED(_state, payload) {
    Vue.set(_state, 'account', payload);
    console.debug('HANDLE_ACCOUNTS_CHANGED', payload);
  },
  SET_IS_EVM(_state, value: boolean) {
    Vue.set(_state, 'isEVM', value);
  }
};

const actions = {
  login: async ({ dispatch }, connector = 'injected') => {
    auth = getInstance();
    await auth.login(connector);

    if (auth.provider) {
      auth.web3 = auth.provider;
      await dispatch('loadProvider');
    }
  },
  logout: async ({ commit, dispatch }) => {
    try {
      await Vue.prototype.$auth.logout();
    } catch (e) {
      dispatch('notify', ['red', 'Logout failed. Please try again.']);
    } finally {
      commit('LOGOUT');
    }
  },
  // Adopt the account that actually signed as the session account. The wallet
  // signs with its ACTIVE account, which can differ from the connected one
  // (multiple accounts, or the account-change subscription never fired under
  // the wallet's compat layer). Sync the session to the real signer and warn
  // when it changed so the user sees who they are acting as.
  reconcileSigner: ({ commit, dispatch, state }, signer: Account) => {
    if (addressesMatch(signer.base16, state.account.base16)) return;
    commit('HANDLE_ACCOUNTS_CHANGED', signer);
    dispatch('notify', [
      'yellow',
      `Signing as ${signer.bech32.slice(0, 9)}…${signer.bech32.slice(
        -4
      )} — the account active in your wallet. Switch accounts in your wallet if this isn't who you meant to act as.`
    ]);
  },
  loadProvider: async ({ commit }) => {
    commit('LOAD_PROVIDER_REQUEST');
    try {
      if (auth.provider?.isEVM) {
        // EVM path — address is carried on the provider sentinel from EVMConnector.connect()
        const account = toAccount(auth.provider.address);
        commit('HANDLE_CHAIN_CHANGED', 'mainnet');
        commit('SET_IS_EVM', true);
        commit('LOAD_PROVIDER_SUCCESS', {
          account,
          name: '0x' + account.base16
        });

        // Subscribe to future account changes (e.g. user switches wallet in MetaMask)
        window['ethereum'].on('accountsChanged', (accounts: string[]) => {
          if (accounts.length) {
            commit('HANDLE_ACCOUNTS_CHANGED', toAccount(accounts[0]));
          } else {
            commit('LOGOUT');
          }
        });
        return;
      }

      // ZilPay path
      if (auth.provider) {
        auth.provider.wallet
          .observableAccount()
          .subscribe(async (account: any) => {
            commit('HANDLE_ACCOUNTS_CHANGED', account);
          });
        auth.provider.wallet.observableNetwork().subscribe((net: string) => {
          commit('HANDLE_CHAIN_CHANGED', net);
        });
      }
      const net = auth.provider.wallet.net;
      commit('HANDLE_CHAIN_CHANGED', net);
      const account = auth.provider.wallet.defaultAccount;
      const name = auth.provider.wallet.defaultAccount.bech32;
      commit('LOAD_PROVIDER_SUCCESS', {
        account,
        name
      });
    } catch (e) {
      commit('LOAD_PROVIDER_FAILURE', e);
      return Promise.reject();
    }
  }
};

export default {
  state,
  mutations,
  actions
};
