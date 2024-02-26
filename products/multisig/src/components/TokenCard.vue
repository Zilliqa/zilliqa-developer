<template>
  <div
    class="token-card btn"
    :class="{ selected }"
    @click="$emit('select', token)"
  >
    <img
      class="card-image"
      :src="url"
      height="60"
      width="60"
      @error="onErrorLoading"
    >
    <h4 class="card-title subtitle text-white">
      {{ token.name }}
    </h4>
    <h2 class="card-subtitle subtitle text-white">
      <div v-show="balance === undefined">
        <i class="fas fa-spinner fa-spin" />
      </div>
      <span v-show="balance !== undefined">
        {{ balance }} {{ token.symbol }}
      </span>
    </h2>
  </div>
</template>

<script>
import { Zilliqa } from "@zilliqa-js/zilliqa";
import plug from '@/assets/token.svg';

export default {
  name: 'TokenCard',
  props: ['token', 'network', 'address', 'selected'],
  data() {
    return {
      balance: undefined,
      zilliqa: null,
      loadingError: null
    };
  },
  computed: {
    url() {
      return `https://raw.githubusercontent.com/Switcheo/zilswap-token-list/master/logos/${this.token.symbol}.svg`;
    }
  },
  methods: {
    onErrorLoading(event) {
      if (!event.target.src.includes(plug)) {
        event.target.src = plug;
      }
    },
    async getBalance() {
      try {
        if (this.token.symbol === 'ZIL') {
          const { result } = await this.zilliqa.blockchain.getBalance(this.address);

          if (result && result.balance) {
            return result.balance;
          }

          throw new Error();
        } else {
          const field = 'balances';
          const wallet = String(this.address).toLowerCase();
          const { result } = await this.zilliqa.blockchain.getSmartContractSubState(
            this.token.address,
            field,
            [wallet]
          );

          if (result && result[field] && result[field][wallet]) {
            return result[field][wallet];
          }
        }
        

        return '0';
      } catch (err) {
        return '0';
      }
    },
    async balanceUpdate() {
      const balance = await this.getBalance();
      const decimals = Number(this.token.decimals);

      this.balance = Number(balance) / 10 ** decimals;
    }
  },
  async beforeMount() {
    if (this.network.name === "ZilPay") {
      this.zilliqa = window['zilPay'];
    } else {
      this.zilliqa = new Zilliqa(this.network.url);
    }

    await this.balanceUpdate();
  }
}
</script>

<style lang="scss">
.token-card {
  display: flex;
  flex-direction: column;
  align-items: center;

  padding: 10px;
  margin: 10px;

  border-radius: 0.375rem;
  max-width: 150px;
  width: 100%;
  height: 150px;

  border-color: #354960;

  &:hover {
    background-color: #354960;
    border-color: #354960;
  }
}

.selected {
  background-color: #354960;
  border-color: #354960;
}
.card-title {
  font-size: 13px;
}
.card-subtitle {
  font-size: 15px;
}
// #354960
</style>
