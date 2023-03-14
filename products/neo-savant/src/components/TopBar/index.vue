<template>
  <div class="top-bar d-flex">
    <div class="logo-container mr-5">
      <img src="@/assets/logo.png" />
    </div>
    <div class="menus flex-grow-1 d-flex justify-content-between">
      <div class="main d-flex align-items-center mr-5">
        <a href="https://scilla.readthedocs.io/en/latest/" target="_blank" class="mr-3">Scilla Docs</a>
        <a href="https://learnscilla.com" target="_blank">Tutorial</a>
        <li class="tools-menu">
          <a href="#">Tools</a>

          <ul class="submenu">
            <li>
              <a href="#" @click="handleOpenTools('units-converter')">Units Converter</a>
            </li>
            <li>
              <a href="#" @click="handleOpenTools('address-converter')">Address Converter</a>
            </li>
          </ul>
        </li>
        <explorer-link />
        <faucet-link v-if="isFaucetAvailable" class="ml-3"/>
      </div>
      <div class="details d-flex">
        <account-balance />
        <account-selector />
        <network-selector />
      </div>
    </div>
  </div>
</template>

<script>
import NetworkSelector from "./NetworkSelector";
import AccountSelector from "./AccountSelector";
import AccountBalance from "./AccountBalance";
import ExplorerLink from "../UI/ExplorerLink";
import FaucetLink from "../UI/FaucetLink";

import { mapGetters } from "vuex";

export default {
  name: "TopBar",
  components: {
    NetworkSelector,
    AccountSelector,
    AccountBalance,
    ExplorerLink,
    FaucetLink,
  },
  computed: {
    ...mapGetters("networks", { selectedNetwork: "selected" }),
    isFaucetAvailable() {
      const allowList = [222, 333];
      return allowList.includes(this.selectedNetwork.chainId);
    },
  },
  methods: {
    handleOpenTools(toolName) {
      window.EventBus.$emit("open-tools", toolName);
    }
  },
};
</script>

<style lang="scss" scoped>
.top-bar {
  background: $primary;
  position: relative;

  width: 100%;
  height: 48px;

  padding-left: 2rem;
  padding-right: 2rem;

  font-size: 14px;

  a {
    color: #fff;
  }

  .logo-container {
    background-color: #fff;
    width: 80px;
    align-items: center;
    justify-content: center;
    display: flex;

    img {
      max-height: 48px;
      object-fit: contain;
    }
  }

  .tools-menu {
    list-style: none;
    margin-left: 1rem;
    margin-right: 1rem;
    position: relative;

    .submenu {
      display: none;
      flex-direction: column;
      position: absolute;
      left: 0;
      width: 150px;
      list-style: none;
      padding: 0;
      background-color: saturate($primary, 80);
      z-index: 99;
      padding-top: 0.5rem;

      li {
        padding: 0.5rem;
      }
    }

    &:hover {
      .submenu {
        display: flex;
      }
    }
  }
}
</style>
