<template>
  <div
    class="account-balance d-flex align-items-center mr-4"
    :class="{'not-loading': !balanceLoading}"
    @click="getBalance"
  >
    <svg
      width="24px"
      height="24px"
      class="mr-1 zilliqa-logo"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 149.33 208.66"
    >
      <g id="Layer_2" data-name="Layer 2">
        <g id="Layer_1-2" data-name="Layer 1">
          <g id="Layer_2-2" data-name="Layer 2">
            <g id="Layer_1-2-2" data-name="Layer 1-2">
              <path
                class="cls-1"
                d="M148.33,88.09V56.45L32.85,1.1,1,14.92v32L77.72,83.62,1,121v31.59l116.15,55,31.18-13.84v-95l-33.1,14.69V173.5L44,139.26l74-37.72Zm-29.18,8.69V72.16l25.32-11.5V85.58Zm22.66-39.2L117.19,68.76,7.68,16.3,32.81,5.36Zm-22.66,58.35,25.32-11.24v86.5l-25.32,11.24Zm-3.89,61.84v24.54L5,150.05V123.34L86.7,83.64,5,44.44V19.33L115.31,72.16V98.65L35.23,139.37Z"
              />
            </g>
          </g>
        </g>
      </g>
    </svg>
    <i class="fas fa-sync mr-2"></i>
    <span class="font-weight-medium" v-if="!balanceLoading">{{ balance }}</span>
    <i class="fas fa-spinner fa-spin" v-else></i>
    <span class="ml-1">ZIL</span>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
const { Zilliqa } = require("@zilliqa-js/zilliqa");
const { units, BN } = require("@zilliqa-js/util");

import numbro from "numbro";

export default {
  data() {
    return {
      internalBalance: 0,
      balanceLoading: true
    };
  },
  computed: {
    ...mapGetters("accounts", { account: "selected" }),
    ...mapGetters("networks", { network: "selected" }),
    balance() {
      return numbro(this.internalBalance).format({ thousandSeparated: true });
    }
  },
  methods: {
    async getBalance() {
      this.balanceLoading = true;
      if (this.account !== undefined && this.account.address) {
        const zilliqa = new Zilliqa(this.network.url);

        const balance = await zilliqa.blockchain.getBalance(
          this.account.address
        );

        if (!balance.error && balance.result.balance) {
          this.internalBalance = units.fromQa(
            new BN(balance.result.balance),
            units.Units.Zil
          );
          this.balanceLoading = false;
          return this.balance;
        }
        this.balanceLoading = false;
        this.internalBalance = 0;

        return 0;
      } else {
        this.balanceLoading = false;
        this.internalBalance = 0;
        return 0;
      }
    }
  },
  mounted() {
    this.getBalance();

    window.EventBus.$on("refresh-balance", async () => {
      await this.getBalance();
    });
  }
};
</script>

<style lang="scss" scoped>
.account-balance {
  padding: 0 0.5rem;
  transition: all 0.2s ease;
  color: #000;

  .fa-sync {
    display: none;
  }

  &.not-loading:hover {
    cursor: pointer;
    background-color: lighten($primary, 10);

    .zilliqa-logo {
      display: none;
    }

    .fa-sync {
      display: block;
    }
  }
}

.cls-1 {
  stroke: #000;
  stroke-miterlimit: 10;
  stroke-width: 2px;
}
</style>