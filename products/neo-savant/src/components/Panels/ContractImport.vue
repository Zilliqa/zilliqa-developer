<template>
  <div class="panel-content">
    <div class="header">
      <div class="title">Import Contract</div>
      <img src="@/assets/close-color.svg" @click="handleClose" class="close-button-new" />
    </div>
    <div class="body p-4">
      <div class="account-selector" v-if="account !== undefined && account.address">
        <div class="deploy-form" v-if="!loading">
          <div class="row mb-4">
            <div class="col-12">
              <label>Contract Address</label>
              <input type="text" v-model="address" class="form-control" />
            </div>
          </div>

          <div class="row">
            <div class="col-12 mb-4" v-if="!loading">
              <button class="btn btn-secondary mr-2" @click="handleImport">Import Contract</button>
            </div>
          </div>
        </div>

        <div class="alert alert-info" v-if="loading">{{loading}}</div>
        <div class="alert alert-danger" v-if="error">{{error}}</div>

        <div class="alert alert-success" v-if="success">Contract successfully imported.</div>

        <button class="btn btn-danger" @click="reset" v-if="loading || error || success">Reset</button>
      </div>
      <div class="alert alert-info m-4" v-else>Please select an account first.</div>
    </div>
  </div>
</template>

<script>
// mport { BN, units, bytes, Long } from "@zilliqa-js/util";
// import Ledger from '@/utils/zil-ledger-interface';
// import TransportU2F from '@ledgerhq/hw-transport-u2f';
import { Zilliqa } from "@zilliqa-js/zilliqa";
import { mapGetters } from "vuex";

export default {
  data() {
    return {
      address: "",
      loading: false,
      error: false,
      success: false,
      contractCode: undefined
    };
  },
  components: {},
  computed: {
    ...mapGetters("accounts", { account: "selected" }),
    ...mapGetters("contracts", { contract: "selected" }),
    ...mapGetters("networks", { network: "selected" })
  },
  methods: {
    handleClose() {
      window.EventBus.$emit("close-right-panel");
    },
    reset() {
      this.loading = false;
      this.error = false;
      this.success = false;
      this.address = "";
      this.contractCode = undefined;
    },
    async handleImport() {
      this.success = false;
      this.error = false;
      this.loading = `Trying to import contract from ${this.address}`;
      try {
        const zilliqa = new Zilliqa(this.network.url);

        this.contractInit = (
          await zilliqa.blockchain.getSmartContractInit(this.address)
        ).result;

        this.contractState = (
          await zilliqa.blockchain.getSmartContractState(this.address)
        ).result;

        const contractCode = await zilliqa.blockchain.getSmartContractCode(
          this.address
        );

        if (contractCode.error) {
          throw new Error(contractCode.error.message);
        }

        this.contractCode = contractCode.result.code;

        const contract = {
          transId: this.address,
          txData: "",
          contractId: this.address,
          network: this.network.url,
          file_id: "imported",
          file_name: "imported",
          deployed_by: this.account.address,
          code: this.contractCode
        };

        await this.$store
          .dispatch("contracts/AddContract", contract)
          .then(() => {
            this.success = true;
          });
      } catch (error) {
        this.error = error;
      }
    }
  }
};
</script>

<style lang="scss" scoped>
.accounts-list {
  .item {
    border: 1px dashed #ccc;
    background-color: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
    transition: all 0.2s ease-in-out;

    &:hover {
      background-color: rgba(0, 0, 0, 0.1);
      cursor: pointer;
    }
  }
}

.btn {
  font-size: 0.85rem !important;

  &.faded {
    opacity: 0.5;
  }
}
</style>