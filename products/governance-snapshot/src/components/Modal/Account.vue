<template>
  <UiModal :open="open" @close="$emit('close')">
    <div v-if="!web3.account.bech32">
      <h3 class="m-4 mb-0 text-center">Connect wallet</h3>
      <div class="m-4 mb-5">
        <a
          v-for="(connector, id, i) in config.connectors"
          :key="i"
          @click="$emit('login', connector.id)"
          target="_blank"
          class="mb-2 d-block"
        >
          <UiButton class="button-outline width-full v-align-middle">
            <img
              v-if="connector.id === 'zlp'"
              src="@/assets/zlp.png"
              height="28"
              width="28"
              class="mr-1 v-align-middle"
            />
            <Icon
              v-else
              name="wallet"
              size="28"
              class="mr-1 v-align-middle"
            />
            {{ connector.name }}
          </UiButton>
        </a>
      </div>
    </div>
    <div v-else>
      <h3 class="m-4 mb-0 text-center">Account</h3>
      <div v-if="$auth.isAuthenticated" class="m-4">
        <a
          :href="accountExplorerUrl"
          target="_blank"
          class="mb-2 d-block"
        >
          <UiButton class="button-outline width-full">
            <Avatar
              :address="web3.account.base16"
              size="16"
              class="mr-2 ml-n1"
            />
            <span v-text="_shorten(accountDisplayAddress)" />
            <Icon name="external-link" class="ml-1" />
          </UiButton>
        </a>
        <UiButton
          @click="handleLogout"
          class="button-outline width-full text-red mb-2"
        >
          Disconnect
        </UiButton>
      </div>
    </div>
  </UiModal>
</template>

<script>
import { mapActions } from 'vuex';

export default {
  props: ['open'],
  computed: {
    accountDisplayAddress() {
      if (this.web3.isEVM) {
        return '0x' + this.web3.account.base16;
      }
      return this.web3.account.bech32;
    },
    accountExplorerUrl() {
      if (this.web3.isEVM) {
        return `https://zilliqa.blockscout.com/address/0x${this.web3.account.base16}`;
      }
      return this._explorer(this.web3.network.name, this.web3.account.bech32);
    }
  },
  methods: {
    ...mapActions(['logout']),
    async handleLogout() {
      await this.logout();
      this.$emit('close');
    }
  }
};
</script>
