<template>
  <a
    :href="link"
    target="_blank"
    class="testnet-wallet-link"
  >
    <span>Faucet</span>
  </a>
</template>

<script>
import { mapGetters } from "vuex";
import { toBech32Address } from "@zilliqa-js/crypto";

export default {
  name: "FaucetLink",
  computed: {
    ...mapGetters("accounts", { account: "selected" }),
    ...mapGetters("networks", { network: "selected", networksList: "list" }),
    link() {
      // the possible chain IDs for the faucet service: 222, 333
      const network = this.network.chainId === 222 ? 'isolated_server' : 'testnet';
      const url = process.env.VUE_APP_FAUCET_URL
      if (this.account && this.account.address) {
        const bech32Address = toBech32Address(this.account.address)
        return url+ `?address=${bech32Address}&network=${network}`
      }
      return url+`?network=${network}`
    }
  }
};
</script>

<style lang="scss">
.address-container {
  width: 100%;
  color: #000;

  &:hover {
    text-decoration: none;
    color: $primary;
  }

  .address {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
}
</style>