<template>
  <div
    class="network-selector d-flex"
    v-if="account && account.type === 'zilpay'"
  >
    <v-popover trigger="click" class="d-flex align-items-center">
      <!-- This will be the popover target (for the events and position) -->
      <div class="selected-network d-flex align-items-center">
        <img src="@/assets/server.svg" height="24px" class="mr-2" />
        {{ selected.name }}
      </div>
      <template slot="popover" class="text-center">
        Accounts and Networks are managed by ZilPay Extension.
        <br />
        <br />
        <button class="btn btn-success" @click="handleAccountManagerSwitch">
          Switch back to IDE Account Manager
        </button>
      </template>
    </v-popover>
  </div>
  <div class="network-selector not-zilpay d-flex" v-else>
    <div class="selected-network d-flex align-items-center">
      <img src="@/assets/server.svg" height="24px" class="mr-2" />
      {{ selected.name }}
    </div>

    <div class="networks-list flex-column">
      <div
        class="d-flex align-items-center"
        v-for="network in list"
        :key="network.name"
      >
        <div class="p-2" v-if="network.type === 'custom'">
          <i class="fas fa-trash" @click.prevent="handleRemove(network)"></i>
        </div>
        <div class="item flex-grow-1" @click="handleSelect(network)">
          {{ network.name }}
        </div>
      </div>

      <div class="item font-weight-bold" @click="handleAddNetwork">
        <i class="fas fa-plus-square"></i> Add network
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  computed: {
    ...mapGetters("networks", ["selected", "list"]),
    ...mapGetters("accounts", { account: "selected" }),
  },
  methods: {
    async handleAccountManagerSwitch() {
      await this.$store.dispatch("accounts/SelectAccount", undefined);
      window.EventBus.$emit("refresh-balance");
    },
    handleSelect(network) {
      this.$store.dispatch("networks/SelectNetwork", network);
    },
    handleAddNetwork() {
      window.EventBus.$emit("open-add-custom-network");
    },
    handleRemove(network) {
      return confirm(
        "Are you sure you want to delete this network?",
        this.$store.dispatch("networks/RemoveNetwork", network)
      );
    },
  },
};
</script>

<style lang="scss" scoped>
.network-selector {
  min-width: 130px;
  padding: 0 0.5rem;
  transition: all 0.2s ease;
  &:hover {
    cursor: pointer;
    background-color: lighten($primary, 10);
  }
  .trigger {
    display: flex !important;
  }

  color: #000;
  position: relative;
  .networks-list {
    display: none;

    .fa-trash {
      opacity: 0.6;

      &:hover {
        opacity: 1;
      }
    }
  }

  &.not-zilpay:hover {
    .networks-list {
      position: absolute;
      left: 0;
      top: 48px;
      z-index: 9999;
      display: flex;
      background-color: lighten($primary, 20);

      .item {
        padding: 0.5rem;

        &:hover {
          background-color: $primary;
        }
      }
    }
  }
}
</style>