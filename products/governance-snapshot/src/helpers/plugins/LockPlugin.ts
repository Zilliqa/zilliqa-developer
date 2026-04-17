// @ts-ignore
import Vue from 'vue'; // v^2.6.11
import LockConnector from '@snapshot-labs/lock/src/connector';
import Lock from '@snapshot-labs/lock/src/lock';

const name = 'lock';

let instance;

export const getInstance = () => instance;

export const useLock = ({ ...options }) => {
  if (instance) return instance;

  instance = new Vue({
    data() {
      return {
        isAuthenticated: false,
        lockClient: null,
        provider: null,
        web3: null
      };
    },
    methods: {
      async login(connector) {
        // @ts-ignore
        const lockConnector = this.lockClient.getConnector(connector);
        const provider = await lockConnector.connect();
        if (provider) {
          localStorage.setItem(`_${name}.connector`, connector);
          this.isAuthenticated = true;
          this.provider = provider;
        }
        return provider;
      },
      async logout() {
        const connector = await this.getConnector();
        if (connector) {
          // @ts-ignore
          const lockConnector = this.lockClient.getConnector(connector);
          await lockConnector.logout();
          localStorage.removeItem(`_${name}.connector`);
          this.isAuthenticated = false;
          this.provider = null;
        }
      },
      async getConnector() {
        const connector: any = localStorage.getItem(`_${name}.connector`);

        if (connector) {
          // @ts-ignore
          const lockConnector = this.lockClient.getConnector(connector);
          const isLoggedIn = await lockConnector.isLoggedIn();
          return isLoggedIn ? connector : false;
        }

        return false;
      }
    },
    async created() {
      const lock = new Lock();
      options.connectors.forEach(connector => {
        lock.addConnector(connector);
      });
      // @ts-ignore
      this.lockClient = lock;
    }
  });

  return instance;
};

export class ZilPay extends LockConnector {
  async connect() {
    let provider = null;
    if (window['zilPay']) {
      try {
        await window['zilPay'].wallet.connect();
        provider = window['zilPay'];
      } catch (e) {
        console.error("error getting ZilPay provider", e);
      }
    } else if (window['zilliqa']) {
      provider = window['zilliqa'];
    }
    return provider;
  }

  isLoggedIn(): boolean {
    return Boolean(window['zilPay']);
  }
}

export class EVMConnector extends LockConnector {
  async connect() {
    if (!window['ethereum'])
      throw new Error('No EVM wallet found. Install MetaMask or another browser wallet.');
    const accounts: string[] = await window['ethereum'].request({ method: 'eth_requestAccounts' });
    if (!accounts.length) throw new Error('No accounts returned by wallet.');
    return { isEVM: true, address: accounts[0] };
  }

  isLoggedIn(): boolean {
    return Boolean(window['ethereum']);
  }

  async logout() {
    // window.ethereum has no disconnect method; app state is cleared by useLock.logout()
  }
}

export const LockPlugin = {
  install(Vue, options) {
    Vue.prototype.$auth = useLock(options);
  }
};
