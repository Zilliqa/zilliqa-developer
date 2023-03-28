<template>
  <div id="app">
    <notifications group="scilla" />
    <tools />
    <top-bar />
    <div class="ide">
      <div id="left-panel" class="left-panel" :class="{ open: leftPanel }">
        <div class="toggler" @click="handleToggleLeftPanel">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            x="0px"
            y="0px"
            viewBox="0 0 511.641 511.641"
            style="enable-background: new 0 0 511.641 511.641"
            xml:space="preserve"
            :class="{ 'panel-open': leftPanel }"
          >
            <path
              d="M148.32,255.76L386.08,18c4.053-4.267,3.947-10.987-0.213-15.04c-4.16-3.947-10.667-3.947-14.827,0L125.707,248.293
			c-4.16,4.16-4.16,10.88,0,15.04L371.04,508.667c4.267,4.053,10.987,3.947,15.04-0.213c3.947-4.16,3.947-10.667,0-14.827
			L148.32,255.76z"
            />
          </svg>
          <span v-if="leftPanel">TOGGLE</span>
          <span v-else>FILES / CONTRACTS</span>
        </div>
        <files-list class="files-container" />
        <contracts-list class="contracts-container" />
      </div>
      <div class="right-panel">
        <div class="main-panel" :class="{ 'has-bottom-panel': bottomPanel }">
          <router-view />
        </div>
        <bottom-panel
          :active="bottomPanel"
          v-on:toggle="handleToggleBottomPanel"
        />
      </div>

      <account-import v-if="rightPanel === 'accountImport'" />

      <events-list v-if="rightPanel === 'events'" />
      <settings v-if="rightPanel === 'settings'" />
      <add-custom-network v-if="rightPanel === 'addCustomNetwork'" />

      <!-- Contract panels -->
      <contract-import v-if="rightPanel === 'importContract'" />
      <deploy-contract
        v-if="rightPanel === 'deployContract'"
        :file="this.deployContract"
        :key="this.deployContract.id"
      />
      <call-contract
        v-if="rightPanel === 'callContract'"
        :contractId="this.callContract"
        :key="this.callContract"
      />

      <div class="right-sidebar">
        <div
          class="action events-badge"
          @click="handleToggleRightPanel('events')"
        >
          <img src="@/assets/notifications.svg" />

          <span class="badge badge-danger" v-if="events.length">{{
            events.length
          }}</span>
        </div>

        <div class="action" @click="handleToggleRightPanel('settings')">
          <img src="@/assets/industry.svg" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import FilesList from "@/components/Files/List";
import ContractsList from "@/components/Contracts/List";
import TopBar from "@/components/TopBar/index";

// Panels
import DeployContract from "@/components/Panels/DeployContract";
import CallContract from "@/components/Panels/CallContract";
import AccountImport from "@/components/Panels/AccountImport";
import ContractImport from "@/components/Panels/ContractImport";

import BottomPanel from "@/components/BottomPanel";

import EventsList from "@/components/Panels/EventsList";
import Settings from "@/components/Panels/Settings";
import AddCustomNetwork from "@/components/Panels/AddCustomNetwork";

import Tools from "@/components/Tools";

import { mapGetters } from "vuex";

import { generateMultipleZilliqaAccounts } from "./utils/zilliqa";
import { animateCSS } from "./utils/ui";

import { Zilliqa } from "@zilliqa-js/zilliqa";
import ZilPayMixin from "@/mixins/zilpay";
import { toChecksumAddress } from "@zilliqa-js/crypto";

export default {
  name: "App",
  data() {
    return {
      leftPanel: true,
      rightPanel: false,
      deployContract: false,
      callContract: false,
      bottomPanel: true,
    };
  },
  mixins: [ZilPayMixin],
  components: {
    FilesList,
    ContractsList,
    TopBar,
    AccountImport,
    DeployContract,
    CallContract,
    ContractImport,
    BottomPanel,
    EventsList,
    Settings,
    AddCustomNetwork,
    Tools,
  },
  computed: {
    ...mapGetters("events", { events: "list" }),
    ...mapGetters("accounts", {
      accounts: "list",
      selectedAccount: "selected",
    }),
    ...mapGetters("networks", { network: "selected", networksList: "list" }),
    ...mapGetters("contracts", { contracts: "list" }),
  },
  watch: {
    events: function () {
      animateCSS(".events-badge", "heartBeat");
    },
  },
  methods: {
    handleToggleRightPanel(type) {
      if (this.rightPanel === type) {
        this.rightPanel = false;
      } else {
        this.rightPanel = type;
      }
    },
    handleToggleBottomPanel() {
      this.bottomPanel = !this.bottomPanel;
    },
    handleToggleLeftPanel() {
      this.leftPanel = !this.leftPanel;
    },
  },
  async created() {
    // Initialize default network
    if (this.network === undefined) {
      this.$store.dispatch("networks/SelectNetwork", this.networksList[0]);
    }

    if (this.network.url === process.env.VUE_APP_ISOLATED_URL) {
      // Check if contracts are still on network
      const zilliqa = new Zilliqa(process.env.VUE_APP_ISOLATED_URL);

      this.contracts.forEach(async (contract) => {
        const deployed = zilliqa.contracts.at(
          toChecksumAddress(contract.contractId)
        );

        const state = await deployed.getState();

        if (state === undefined) {
          this.$store.dispatch("contracts/RemoveContract", {
            id: contract.contractId,
          });
        }
      });

      // Generate default accounts on Simulated ENV
      const ACCOUNTS_NUMBER = 5;

      if (this.accounts.length === 0) {
        const generatedAccounts = await generateMultipleZilliqaAccounts(
          ACCOUNTS_NUMBER
        );

        generatedAccounts.map((item) => {
          this.$store.dispatch("accounts/AddAccount", {
            address: item.address,
            keystore: item.privateKey,
            type: "privatekey",
          });
        });
      }
    }
  },
  mounted() {
    window.EventBus.$on("close-right-panel", () => {
      this.rightPanel = false;
    });

    window.EventBus.$on("open-account-import", () => {
      this.rightPanel = "accountImport";
    });

    window.EventBus.$on("open-deploy-contract", (file) => {
      window.EventBus.$emit("clear-components");
      this.deployContract = file;
      this.rightPanel = "deployContract";
    });

    window.EventBus.$on("close-deploy-contract", () => {
      this.deployContract = false;
      this.rightPanel = false;
    });

    window.EventBus.$on("open-call-contract", ({ contractId }) => {
      this.callContract = contractId;
      this.rightPanel = "callContract";
    });

    window.EventBus.$on("open-import-contract", () => {
      this.rightPanel = "importContract";
    });

    window.EventBus.$on("open-add-custom-network", () => {
      this.rightPanel = "addCustomNetwork";
    });

    if (
      this.selectedAccount !== undefined &&
      this.selectedAccount.type === "zilpay"
    ) {
      this.getZilPayNetwork()
        .then(() => this.getZilPayAccount())
        .then(() => this.runZilPayObservable());
    }
  },
};
</script>

<style lang="scss">
#app {
  font-family: "Montserrat", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  height: calc(100% - 48px);
}

.panel-content {
  position: absolute;
  top: 0;
  right: 30px; // 50px RightSidebar - 20px scroll sidebar
  height: 100%;
  width: 500px;
  min-width: 450px;
  min-height: 350px;
  z-index: 98;
  border-left: 1px solid saturate($primary, 10);
  background-color: #fff;

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: saturate($primary, 10);
    padding: 0.5rem calc(0.5rem + 20px) 0.5rem 0.5rem;
    border-top: 1px dashed #ccc;

    .title {
      font-size: 1rem;
      color: #fff;
    }

    .close-button-new {
      cursor: pointer;
      height: 16px;
    }
  }

  .body {
    height: calc(100% - 3rem);
    overflow-y: scroll;
    overflow-x: hidden;
  }
}

.btn {
  border-radius: 0px !important;
  font-size: 0.85rem;
  transition: all 0.2s ease-in-out;

  &.btn-block {
    i.fas {
      transition: all 0.2s ease-in-out;
      margin-left: 0.2rem;
    }

    &:hover {
      i.fas {
        transition: all 0.2s ease-in-out;
        margin-left: 0.5rem;
      }
    }
  }

  &.btn-primary {
    background-color: $primary;
    border-color: $primary;

    &:hover {
      background-color: lighten($color: $primary, $amount: 8);
      border-color: lighten($color: $primary, $amount: 8);
    }
  }

  &.btn-outline {
    background-color: transparent;
    &:hover {
      background-color: transparent;
      background-color: transparentize($color: $secondary, $amount: 0.9);
    }
  }
}

input.form-control {
  -webkit-appearance: none !important;
  -moz-appearance: none !important;
  appearance: none !important;
  background-color: #eee !important;
  border: 0 !important;
  border-radius: 0 !important;
}

.ide {
  position: relative;
  display: flex;
  height: 100%;

  .left-panel {
    position: relative;
    padding: 1.5rem 0;
    height: 100%;
    border-right: 1px solid #ccc;
    overflow: hidden;
    width: 12px;
    display: flex;
    flex-direction: column;

    .files-container {
      height: 100%;
      overflow-y: scroll;
    }

    .contracts-container {
      width: calc(100% + 24px);
      height: 100%;
      overflow-y: scroll;
    }

    .toggler {
      position: absolute;
      top: 0;
      right: 0;
      width: 12px;
      height: 100%;
      z-index: 100;
      padding-top: 0.5rem;
      display: flex;
      flex-direction: column;
      align-items: center;
      cursor: pointer;
      transition: all 0.2s ease-in-out;
      background-color: #fff;

      * {
        transition: all 0.2s ease-in-out;
      }

      span {
        writing-mode: vertical-lr;
        text-orientation: upright;
        font-size: 10px;
        opacity: 1;
      }

      svg {
        width: 10px;
        height: 10px;
        margin-bottom: 0.5rem;
        transform: rotate(180deg);
        opacity: 1;
      }

      &:hover {
        background-color: lighten($primary, $amount: 40);
      }
    }

    &.open {
      width: 300px;

      .toggler {
        background-color: transparent;
        svg {
          transform: rotate(0deg);
          opacity: 0.35;
        }
        span {
          opacity: 0;
        }

        &:hover {
          background-color: lighten($primary, $amount: 40);

          span,
          svg {
            opacity: 1;
          }
        }
      }
    }
  }

  .right-panel {
    height: 100%;
    flex-grow: 1;

    .main-panel {
      height: calc(
        100% - 1.25rem - 1px
      ); // full height - bottom panel header - border

      &.has-bottom-panel {
        height: calc(100% - 150px); // full height - bottom panel
      }
    }
  }
  .right-sidebar {
    position: absolute;
    right: 0;
    top: 0;
    width: 50px;
    height: 100%;
    border-left: 1px solid #ccc;
    padding: 0.5rem;
    padding-top: 1rem;
    z-index: 99;
    background-color: #fff;

    .action {
      margin-bottom: 1rem;
      display: flex;
      align-items: center;
      justify-content: center;
      position: relative;

      .badge {
        position: absolute;
        top: 0;
        right: 0;
      }

      img {
        width: 30px;
      }
      &:hover {
        cursor: pointer;
      }
    }
  }
}
</style>
