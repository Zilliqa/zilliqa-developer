<template>
  <div
    class="transactions-list"
    v-if="zilliqa"
  >
    <div class="heading">
      <h2 class="subtitle">Transactions</h2>
      <div class="filters text-white">
        Filter by:
        <span>
          {{ selectedToken ? selectedToken.symbol : 'All Transactions' }}
        </span>
      </div>
    </div>
    <h2
      v-if="list.length === 0"
      class="subtitle text-white"
    >
      Have no transactions yet.
    </h2>
    <div
      v-for="(tx, index) of list"
      :key="index"
      class="content transactions-list"
    >
      <Transaction
        :transaction="tx"
        :wallet="address"
        :owners="owners"
        :signature_counts="signature_counts"
        :signatures="signatures"
        :signatures_need="signatures_need"
        :network="network"
        :zilliqa="zilliqa"
      />
    </div>
  </div>

  <div class="nothing text-white" v-else>No transactions found.</div>
</template>

<script>
import { Zilliqa } from "@zilliqa-js/zilliqa";
import { validation } from "@zilliqa-js/util";
import { fromBech32Address } from "@zilliqa-js/crypto";

import Transaction from "@/components/Transaction.vue";
import ZIlpayMixin from '@/mixins/zilpay';

export default {
  name: "TransactionsList",
  mixins: [ZIlpayMixin],
  props: [
    "address",
    "signatures_need",
    "network",
    "tokens",
    "selectedToken"
  ],
  components: {
    Transaction
  },
  data() {
    return {
      transactions: [],
      owners: [],
      signature_counts: {},
      signatures: [],
      zilliqa: null,
      contractState: {}
    };
  },
  computed: {
    list() {
      return this.transactions.filter((t) => {
        if (!this.selectedToken && t.token) {
          return true;
        }

        return t.token && t.token.symbol === this.selectedToken.symbol;
      });
    }
  },
  methods: {
    async load() {
      let address = this.address;

      if (validation.isBech32(address)) {
        address = fromBech32Address(address);
      }
      const { result } = await this.zilliqa.blockchain.getSmartContractState(address);

      if (result && result.transactions && result.signatures && result.signature_counts) {
        const transactions = Object.keys(result.transactions).map((key) =>{
          const token  = this.tokens.find(
            (t) => {
              return String(t.address).toLowerCase() === String(result.transactions[key].arguments[0]).toLowerCase()
            }
          );
          let subType;
          if(typeof result.transactions[key].arguments[1] ==='object' && ('constructor' in result.transactions[key].arguments[1])) {
            subType = result.transactions[key].arguments[1].constructor.split('.')[1];
          }
          return {
            key: key,
            token: result.transactions && result.transactions[key] && !token
              ? this.tokens[0] : token,
            type: result.transactions[key].constructor.split('.')[1],
            subType: subType,
            destination: result.transactions[key].arguments[0],
            amount: result.transactions[key].arguments[1],
            third: result.transactions[key].arguments[2],
            signatures: result.signatures[key],
            signatures_count: result.signature_counts[key]
          };
        })
        
        this.transactions = transactions;
        this.owners = Object.keys(result.owners);
        this.signature_counts = result.signature_counts;
        this.signatures = Object.values(result.signatures);
      }
    }
  },
  async mounted() {
    if (this.network.name === "ZilPay") {
      this.zilliqa = await this.__getZilPay();
    } else {
      this.zilliqa = new Zilliqa(this.network.url);
    }

    await this.load();
  }
};
</script>

<style scoped lang="scss">
</style>
