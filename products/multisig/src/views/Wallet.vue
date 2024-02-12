<template>
  <div class="wallet">
    <h1 class="title">My Wallet</h1>
    <h2 class="subtitle text-white">{{ bech32Address }}</h2>
    <div
      v-if="address"
      class="tokens"
    >
      <TokenCard
        v-for="(el, index) of tokenList"
        :key="index"
        :selected="selectedToken && selectedToken.symbol === el.symbol"
        :token="el"
        :network="network"
        :address="address"
        @select="onSelectedToken"
      />
      <AddTokenCard @add="onAddToken" v-if="wallet.version === '2.0'"/>
    </div>
    <div class="wallet-details">
      <div class="transactions-container" v-if="!addFunds && !newTransaction && !addToken">
        <transactions-list
          :address="$route.params.address"
          :signatures_need="wallet.signatures"
          :network="network"
          :tokens="tokens"
          :selectedToken="selectedToken"
        />
        <button
          class="btn btn-outline-primary btn-block new-tx-btn"
          @click="onNewTransaction"
        >
          New Transaction
        </button>
      </div>
      <add-funds
        :bech32="bech32Address"
        :address="address"
        :zilliqa="zilliqa"
        :selectedToken="selectedToken"
        v-on:cancel-add-funds="onCancelAddFunds"
        v-if="addFunds"
      />
      <add-token
        v-if="addToken"
        :wallet="address"
        v-on:cancel-add-token="onCancelAddToken"
      />
      <new-transaction
        :zilliqa="zilliqa"
        :address="address"
        :token="selectedToken"
        :walletVersion="wallet.version"
        v-on:cancel-new-transaction="onCancelNewTransaction"
        v-if="newTransaction && selectedToken"
      />
      <div class="sidebar">
        <contract-actions
          :balance="wallet.balance"
          :owners_list="wallet.owners_list"
          class="mb-4"
          v-on:add-funds="onAddFunds"
        />
        <contract-owners :owners="wallet.owners_list" :signatures="wallet.signatures"></contract-owners>
      </div>
    </div>
  </div>
</template>

<script>
import Swal from "sweetalert2";
import { mapGetters, mapState } from "vuex";

import { Zilliqa } from "@zilliqa-js/zilliqa";
import { toBech32Address, fromBech32Address } from "@zilliqa-js/crypto";

import TransactionsList from "@/components/TransactionsList";
import AddFunds from "@/components/AddFunds";
import AddToken from "@/components/AddToken";
import AddTokenCard from "@/components/AddTokenCard";
import TokenCard from "@/components/TokenCard";

import ContractActions from "@/components/Wallet/ContractActions";
import ContractOwners from "@/components/Wallet/ContractOwners";
import NewTransaction from "@/components/Wallet/NewTransaction";
import { units, BN, validation } from '@zilliqa-js/util';
import ZIlpayMixin from '@/mixins/zilpay';

export default {
  name: "Wallet",
  mixins: [ZIlpayMixin],
  components: {
    TransactionsList,
    ContractActions,
    ContractOwners,
    AddFunds,
    AddToken,
    AddTokenCard,
    TokenCard,
    NewTransaction
  },
  data() {
    return {
      zilliqa: null,
      selectedToken: null,
      init: null,
      address: null,
      bech32Address: null,
      wallet: {
        balance: 0,
        owners_list: [],
        signatures: null,
        version: '1.0',
      },
      addFunds: false,
      addToken: false,
      newTransaction: false
    };
  },
  computed: {
    ...mapState('general', [
      'tokens'
    ]),
    ...mapGetters("general", {
      network: "selectedNetwork",
      personalAddress: "personalAddress",
      localWallet: "wallet"
    }),
    ...mapGetters("wallets", {
      getWalletOwnersList: "getOwnersList"
    }),
    tokenList() {
      return this.tokens.filter((t) => {
        if (t.symbol === 'ZIL') {
          return true;
        }
        
        return t.wallet && t.wallet.includes(this.address);
      });
    }
  },
  methods: {
    onSelectedToken(token) {
      this.selectedToken = token;
      this.addFunds = false;
      this.addToken = false;
    },
    onNewTransaction() {
      this.newTransaction = true;

      if (!this.selectedToken) {
        const [zilliqa] = this.tokens;

        this.selectedToken = zilliqa;
      }
    },
    onAddFunds() {
      this.newTransaction = false;
      this.addFunds = true;
      this.addToken = false;

      if (!this.selectedToken) {
        const [zilliqa] = this.tokens;

        this.selectedToken = zilliqa;
      }
    },
    onAddToken() {
      this.newTransaction = false;
      this.addFunds = false;
      this.addToken = true;
      this.selectedToken = null;
    },
    onCancelAddFunds() {
      this.addFunds = false;
    },
    onCancelAddToken() {
      this.newTransaction = false;
      this.addFunds = false;
      this.addToken = false;
    },
    onCancelNewTransaction() {
      this.newTransaction = false;
    }
  },
  async mounted() {
    try {
      if (this.network.name === "ZilPay") {
        this.zilliqa = await this.__getZilPay();
      } else {
        this.zilliqa = new Zilliqa(this.network.url);
      }

      let address = this.$route.params.address;

      if (validation.isBech32(address)) {
        this.bech32Address = address;
        this.address = fromBech32Address(address);
      } else {
        this.bech32Address = toBech32Address(address);
        this.address = address;
      }

      const contract = await this.zilliqa.blockchain.getSmartContractInit(
        this.address
      );

      if (contract.error !== undefined) {
        throw contract.error;
      }

      if (contract.result !== undefined) {
        const required_signatures = contract.result.find(item => {
          return item.vname === "required_signatures";
        });

        const contract_version = contract.result.find(item => {
          return item.vname === "contract_version";
        });

        this.init = contract.result;

        if (contract_version !== undefined) {
          this.wallet.version = contract_version.value;
        }

        if (required_signatures === undefined) {
          throw "Required signatures value could not be found.";
        } else {
          this.wallet.signatures = required_signatures.value;
        }

        this.wallet.owners_list = this.getWalletOwnersList(toBech32Address(this.address));
      }

      const contractState = await this.zilliqa.blockchain.getSmartContractState(
        this.address
      );
      
      if (contractState.result !== undefined) {
        this.wallet.balance = units.fromQa(new BN(contractState.result._balance), units.Units.Zil);
      } else {
        throw contractState.error;
      }

      this.transactions = contractState.transactions;
    } catch (error) {
      console.warn(error);
      Swal.fire({
        type: "error",
        text: error.message
      });
      return this.$router.push({ name: "wallet-list" });
    }
  }
};
</script>


<style lang="scss">
.tokens {
  display: flex;
  flex-wrap: wrap;
  padding-bottom: 50px;
}

.wallet-details {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
}

.sidebar {
  max-width: 300px;
}
.new-tx-btn {
  max-width: 280px;
}
@media only screen and (max-width: 1300px) {
  .sidebar {
    margin-top: 2rem;
  }
}
</style>
