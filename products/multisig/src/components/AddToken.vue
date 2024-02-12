<template>
  <div class="add-funds-form">
    <h2 class="subtitle mb-5">Add Token</h2>
    <div class="input-group mb-3">
      <div class="input-group-prepend">
        <span class="input-group-text" id="basic-addon1">zil1</span>
      </div>
      <input
        type="text"
        class="form-control"
        placeholder="Token address"
        aria-label="Token address"
        v-model="address"
        @input="error = false"
      >
    </div>
    <h3 v-if="error" class="err-message text-danger">
      {{errorMsg}}
    </h3>
    <div class="buttons">
      <div v-if="isLoading" class="loading text-white">
        <i class="fas fa-spinner fa-spin"></i> Please wait a bit.
      </div>
      <div v-if="!isLoading">
        <button
          class="btn btn-primary mr-4"
          :disabled="!isEnable"
          @click="proceed"
        >
          Submit
        </button>
        <button class="btn btn-outline-secondary" @click="$emit('cancel-add-token')">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters, mapMutations } from 'vuex';
import { fromBech32Address } from "@zilliqa-js/crypto";
import { Zilliqa } from "@zilliqa-js/zilliqa";

export default {
  name: 'AddToken',
  props: ['wallet'],
  data() {
    return {
      isLoading: false,
      error: false,
      errorMsg: "Incorrect token",
      zilliqa: null,
      address: null
    };
  },
  components: {
  },
  computed: {
    ...mapGetters('general', {
      network: 'selectedNetwork',
      personalAddress: 'personalAddress'
    }),

    isEnable() {
      try {
        fromBech32Address(this.address);

        return true;
      } catch {
        return false;
      }
    }
  },
  methods: {
    ...mapMutations('general', [
      'addToken'
    ]),
    async getMethods(base16){
      const methods = {
        IncreaseAllowance: 'IncreaseAllowance',
        DecreaseAllowance: 'DecreaseAllowance',
        Transfer: 'Transfer',
        TransferFrom: 'TransferFrom',
        Burn: 'Burn',
        Mint: 'Mint'
      };
      const scCode = await this.zilliqa.blockchain.getSmartContractCode(base16);

      for(let i in methods){
        let v = methods[i];
        let isExist = scCode.result.code.includes(`${v}(`);
        if(!isExist){
          delete methods[i];
        }
      }

      return methods;
    },
    async getBalance(contract) {
      const field = 'balances';
      const wallet = String(this.wallet).toLowerCase();
      try {
        const { result } = await this.zilliqa.blockchain.getSmartContractSubState(
          contract,
          field,
          [wallet]
        );

        if (result && result[field] && result[field][wallet]) {
          return result[field][wallet];
        }

        return '0';
      } catch {
        return '0';
      }
    },
    async proceed() {
      this.isLoading = true;

      try {
        const base16 = fromBech32Address(this.address);
        const initRes = await this.zilliqa.blockchain.getSmartContractInit(base16);
        const init = initRes.result;
        const symbol = init.find((el) => el.vname === 'symbol').value;
        const name = init.find((el) => el.vname === 'name').value;
        const decimals = init.find((el) => el.vname === 'decimals').value;

        if (!symbol || !name || !decimals) {
          throw new Error();
        }

        const methods = await this.getMethods(base16);

        this.addToken({
          address: base16,
          name,
          symbol,
          decimals,  
          methods,
          wallet: this.wallet
        });

        this.$emit('cancel-add-token');
      } catch(err) {
        this.error = true;
        this.errorMsg = err.message ? err.message: this.errorMsg;
      }

      this.isLoading = false;
    }
  },
  mounted() {
    if (this.network.name === "ZilPay") {
      this.zilliqa = window['zilPay'];
    } else {
      this.zilliqa = new Zilliqa(this.network.url);
    }
  }
};
</script>

<style lang="scss" scoped>
.advanced-options {
  margin-bottom: 2rem;
}

.toggle-advanced-options {
  cursor: pointer;
}

.err-message {
  min-width: 200px;
}
</style>