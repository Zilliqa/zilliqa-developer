<template>
  <div class="panel-content">
    <div class="header">
      <div class="title">Add Custom Network</div>
      <img
        src="@/assets/close-color.svg"
        @click="handleClose"
        class="close-button-new"
      />
    </div>
    <div class="body p-4">
      <div class="account-selector">
        <div class="deploy-form" v-if="!loading && !success">
          <div class="row mb-4">
            <div class="col-12 mb-4">
              <label>Network name</label>
              <input type="text" v-model="name" class="form-control" />
            </div>
            <div class="col-12 mb-4">
              <label>Network address</label>
              <input type="text" v-model="url" class="form-control" />
            </div>
            <div class="col-12 mb-4">
              <label>Chain ID</label>
              <input
                type="text"
                v-model.number="chainId"
                class="form-control"
              />
            </div>
            <div class="col-12 mb-4">
              <label>MSG VERSION</label>
              <input
                type="text"
                v-model.number="msgVersion"
                class="form-control"
              />
            </div>
          </div>

          <div class="row">
            <div class="col-12 mb-4" v-if="!loading">
              <button class="btn btn-secondary mr-2" @click="handleImport">
                Submit
              </button>
            </div>
          </div>
        </div>

        <div class="alert alert-info" v-if="loading">{{ loading }}</div>
        <div class="alert alert-danger" v-if="error">{{ error }}</div>

        <div class="alert alert-success" v-if="success">
          Network successfully added.
        </div>
      </div>
    </div>
  </div>
</template>

  <script>
import { mapGetters } from "vuex";

export default {
  data() {
    return {
      name: undefined,
      url: undefined,
      chainId: 1,
      msgVersion: 1,
      loading: false,
      error: false,
      success: false,
      contractCode: undefined,
    };
  },
  components: {},
  computed: {
    ...mapGetters("networks", { network: "selected" }),
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
      this.name = "";
    },
    handleImport() {
      if (!this.name || !this.url) {
        return;
      }

      this.$store
        .dispatch("networks/AddNetwork", {
          name: this.name,
          url: this.url,
          chainId: parseInt(this.chainId),
          type: "custom",
          msgVersion: parseInt(this.msgVersion),
        })
        .then(() => {
          window.EventBus.$emit("close-right-panel");
          this.$notify({
            group: "scilla",
            type: "success",
            position: "bottom right",
            title: "Accounts",
            text: "Network successfully imported.",
          });
          this.loading = false;
        })
        .catch((err) => {
          this.$notify({
            group: "scilla",
            type: "error",
            position: "bottom right",
            title: "Accounts",
            text: err.message,
          });
          this.loading = false;
        });
    },
  },
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