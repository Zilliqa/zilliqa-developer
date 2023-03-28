<template>
  <a
    :href="link"
    target="_blank"
    class="explorer-link"
    :class="{'address-container': txid || address }"
  >
    <div v-if="txid || address" class="d-flex align-items-center">
      <i class="fas fa-link mr-2"></i>
      <address-display :address="txid || address" />
    </div>
    <span v-else>Network Explorer</span>
  </a>
</template>

<script>
import { mapGetters } from "vuex";
import AddressDisplay from "./AddressDisplay";

export default {
  name: "ExplorerLink",
  components: { AddressDisplay },
  props: ["txid", "address"],
  computed: {
    ...mapGetters("networks", { selectedNetwork: "selected" }),
    link() {
      let baseLink = process.env.VUE_APP_EXPLORER_URL;
      let networkLink = "?network=" + this.selectedNetwork.url;
      let txLink = "";

      if (this.txid) {
        txLink = `tx/${this.txid}`;
      }

      if (this.address) {
        txLink = `address/${this.address}`;
      }

      return baseLink + txLink + networkLink;
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