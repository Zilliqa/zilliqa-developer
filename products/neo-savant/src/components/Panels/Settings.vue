<template>
  <div class="settings panel-content">
    <div class="header">
      <div class="title">IDE Settings</div>
      <img src="@/assets/close-color.svg" @click="handleClose" class="close-button-new" />
    </div>
    <div class="body p-4">
      <div class="settings-group mb-4">
        <h5>IDE Settings</h5>

        <div class="form-input">
          <label>Font size</label>
          <input
            class="form-control"
            type="number"
            :value="editor.fontSize"
            @change="updateFontSize"
          />
        </div>
      </div>

      <div class="settings-group mt-5">
        <h5>Reset</h5>
        <button class="btn btn-danger" @click="handleHardReset">Reset IDE to default</button>
      </div>

      <div class="settings-group mt-5">
        <label>Version {{ appVersion }}</label>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import Swal from "sweetalert2";

export default {
  components: {},
  data() {
    return {
      network: {
      }
    };
  },
  computed: {
    ...mapGetters("general", { editor: "editor", appVersion: "appVersion" })
  },
  methods: {
    handleHardReset() {
      Swal.fire({
        title: "Are you sure?",
        text:
          "This action will reset all Savant IDE data to default. You will lose all accounts and contracts stored.",
        type: "warning",
        showCancelButton: true
      }).then(e => {
        if (e.value && e.value === true) {
          localStorage.clear("savant-ide");
          window.location.reload();
        }
      });
    },
    handleClose() {
      window.EventBus.$emit("close-right-panel");
    },
    async updateFontSize(ev) {
      //window.EventBus.$emit("change-editor-fontSize", parseInt(ev.target.value));
      this.$store.dispatch("general/ChangeFontSize", {
        fontSize: parseInt(ev.target.value)
      });
    }
  },
  mounted() {
    /* window.EventBus.$on("console-log", ({ message, type }) => {
      // console.log(message, type);
      //this.$refs.console.$_executeCommand('help');
    }); */
  }
};
</script>

<style lang="scss" >
</style>