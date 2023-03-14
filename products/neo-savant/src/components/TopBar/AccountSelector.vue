<template>
  <div class="account-selector d-flex" v-if="selected && selected.type === 'zilpay'">
    <v-popover trigger="click" class="d-flex align-items-center">
      <!-- This will be the popover target (for the events and position) -->
      <div class="selected-account">
        <div class="selected-item d-flex align-items-center" v-if="selected !== undefined">
          <img src="@/assets/user.svg" height="24px" class="account-icon mr-2" />
          <copy-to-clipboard class="copy-to-clipboard mr-2" :text="selected.address"></copy-to-clipboard>
          <address-display :address="selected.address"></address-display>
        </div>
        <div class="selected-item d-flex mr-5" @click="handleSelect" v-else>
          <img src="@/assets/user.svg" height="24px" class="mr-5" /> Select an account
        </div>
      </div>
      <template slot="popover" class="text-center">
        Accounts and Networks are managed by ZilPay Extension.
        <br />
        <br />
        <button class="btn btn-success" @click="handleAccountManagerSwitch">Switch back to IDE Account Manager</button>
      </template>
    </v-popover>
  </div>
  <div class="account-selector d-flex align-items-center mr-4" v-else>
    <div class="selected-account">
      <div class="selected-item d-flex align-items-center" v-if="selected !== undefined">
        <img src="@/assets/user.svg" height="24px" class="account-icon mr-2" />
        <copy-to-clipboard class="copy-to-clipboard mr-2" :text="selected.address"></copy-to-clipboard>
        <address-display :address="selected.address"></address-display>
      </div>
      <div class="selected-item d-flex mr-5" v-else>
        <img src="@/assets/user.svg" height="24px" class="mr-5" /> Select an account
      </div>

      <div class="accounts-list flex-column">
        <div class="item item-action" @click="handleImport">
          <div class="btn-action">
            <i class="fas fa-file-import mr-2"></i>
            Import account
          </div>
        </div>

        <div class="item-separator" v-if="filteredList.length"></div>
        <div
          class="item d-flex align-items-center"
          v-for="account in filteredList"
          :key="account.address"
        >
          <copy-to-clipboard :text="account.address"></copy-to-clipboard>
          <address-display
            class="select-account"
            :address="account.address"
            @click.native="handleSelect(account)"
            title="Click to select account"
          ></address-display>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";

import CopyToClipboard from "../UI/CopyToClipboard";
import AddressDisplay from "../UI/AddressDisplay";

export default {
  computed: {
    ...mapGetters("accounts", ["selected", "list"]),
    filteredList() {
      if (this.selected !== undefined) {
        return this.list.filter(item => item.address !== this.selected.address);
      }

      return this.list;
    }
  },
  components: { CopyToClipboard, AddressDisplay },
  methods: {
    async handleAccountManagerSwitch() {
      await this.$store.dispatch("accounts/SelectAccount", undefined);
      window.EventBus.$emit("refresh-balance");
    },
    async handleSelect({ address }) {
      await this.$store.dispatch("accounts/SelectAccount", { address });
      window.EventBus.$emit("refresh-balance");
    },
    handleImport() {
      window.EventBus.$emit("open-account-import");
    }
  }
};
</script>

<style lang="scss" scoped>
.account-selector {
  padding: 0 0.5rem;
  transition: all 0.2s ease;
  color: #000;
  position: relative;

  .selected-item {
    .copy-to-clipboard {
      display: none;
    }
  }

  &:hover {
    cursor: default;
    background-color: lighten($primary, 10);

    .selected-item {
      .account-icon {
        display: none;
      }
      .copy-to-clipboard {
        display: block;
      }
    }
  }

  .accounts-list {
    display: none;
    min-width: 100%;

    .item {
      padding: 0.5rem;

      .select-account {
        cursor: pointer;
      }

      &:hover {
        background-color: $primary;
      }
    }

    .item-action {
      cursor: pointer;
      padding-left: calc(0.5rem + 5px);
      font-weight: bold;
    }

    .item-separator {
      border-bottom: 1px solid darken($primary, 10);
    }
  }

  &:hover {
    .accounts-list {
      position: absolute;
      left: 0;
      top: 48px;
      z-index: 9999;
      display: flex;
      background-color: lighten($primary, 10);
    }
  }
}
</style>