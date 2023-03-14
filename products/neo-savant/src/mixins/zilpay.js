// Copyright (C) 2020 Zilliqa

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https:www.gnu.org/licenses/>.

import { mapGetters } from "vuex";

export default {
  data() {
    return {
      observableNetwork: null,
      observableAccount: null
    }
  },
  computed: {
    ...mapGetters("networks", { network: "selected" }),
    ...mapGetters("networks", ["list"]),
    ...mapGetters("accounts", { acountsList: "list" }),

    /**
     * to ZilPay network nameing.
     */
    net() {
      return {
        mainnet: this.list[2],
        private: this.list[0],
        testnet: this.list[1]
      }
    }
  },
  methods: {
    /**
     * When page has loaded.
     */
    _isLoadTab() {
      return new Promise((resolve) => {
        if (window.document.readyState === 'complete') {
          resolve(true)
        }
        window.onload = function () {
          setTimeout(() => resolve(true), 1000)
        }
      })
    },
    /**
     * Testing for ZilPay inject script.
     */
    async testZilPay() {
      await this._isLoadTab()

      if (typeof window.zilPay === 'undefined') {
        throw new Error('ZilPay is not installed!')
      }

      if (!window.zilPay.wallet.isConnect) {
        return window.zilPay.wallet.connect()
      }
    },
    /**
     * Start observable ZilPay network and accounts.
     */
    async runZilPayObservable() {
      await this.testZilPay()

      const { wallet } = window.zilPay

      if (this.observableNetwork && !this.observableNetwork.isStopped) {
        this.observableNetwork.unsubscribe()
      }

      if (this.observableAccount && !this.observableAccount.isStopped) {
        this.observableAccount.unsubscribe()
      }

      this.observableNetwork = wallet.observableNetwork().subscribe(async () => {
        await this.getZilPayNetwork()
        this.getZilPayAccount()
      });
      this.observableAccount = wallet.observableAccount().subscribe(() => {
        this.getZilPayAccount()
      });
    },
    /**
     * Get and update network.
     */
    async getZilPayNetwork() {
      await this.testZilPay()

      const { wallet } = window.zilPay

      await this.$store.dispatch("networks/SelectNetwork", this.net[wallet.net]);
      window.EventBus.$emit("refresh-balance");
    },
    /**
     * Get, create, update accounts.
     */
    async getZilPayAccount() {
      await this.testZilPay()

      const { wallet } = window.zilPay
      const account = wallet.defaultAccount

      if (!wallet.isConnect || !wallet.isEnable) {
        this.$notify({
          group: "scilla",
          type: "error",
          position: "bottom right",
          title: "Accounts",
          text: "ZilPay could not be accessed. Please log in."
        });
        throw new Error("ZilPay could not be accessed. Please log in.");
      }
      
      const hasAccount = this.acountsList.find((acc) => account.base16 === acc.address)

      if (hasAccount) {
        this.$store.dispatch("accounts/SelectAccount", { address: account.base16 });
      } else {
        await this.$store.dispatch("accounts/AddAccount", {
          address: wallet.defaultAccount.base16,
          type: "zilpay"
        });
        this.$notify({
          group: "scilla",
          type: "success",
          position: "bottom right",
          title: "Accounts",
          text: "Account successfully imported"
        });
      }

      window.EventBus.$emit("refresh-balance");
    },
    async signZilPayTx(tx) {
      await this.testZilPay();
      const { blockchain } = window.zilPay
      const result = await blockchain.createTransaction(tx);
      return result;
    }
  }
}
