<template>
  <div class="editor">
    <div class="actions-bar">
      <div
        class="d-flex justify-content-between align-items-center"
        v-if="file && !file.contractId"
      >
        <div class="buttons d-flex">
          <button class="btn btn-check mr-2 ml-2" @click="handleCheck">
            <img src="@/assets/survey.svg" /> CHECK
          </button>
          <button class="btn btn-link mr-2" @click="handleDeploy">
            <img src="@/assets/send.svg" /> DEPLOY
          </button>

          <button class="btn btn-link" v-if="changed" @click="handleSave">
            <img src="@/assets/save.svg" /> SAVE
          </button>
        </div>
        <div class="message d-flex align-items-center" v-if="changed">Remember to save changes</div>
      </div>
      <div class="d-flex p-2 align-items-center" v-else>Deployed contracts are readonly.</div>
    </div>
    <div class="editor-inner d-flex">
      <ace-editor
        v-if="file && !file.contractId"
        v-model="file.code"
        :fontSize="editor.fontSize"
        :showPrintMargin="true"
        :showGutter="true"
        :highlightActiveLine="true"
        ref="aceEditor"
        mode="scilla"
        lang="scilla"
        theme="tomorrow"
        :annotations="annotations"
        width="100%"
        :maxLines="20000"
        height="calc(100% - 60px)"
        :onChange="handleInput"
        name="editor"
        :editorProps="{$blockScrolling: true}"
      />
      <pre class="p-5" style="max-width: 700px; overflow:scroll;" v-else>
        {{ file.code }}
      </pre>
    </div>
  </div>
</template>

<script>
/*eslint-disable */
import brace from "brace"; // eslint-disable-line no-use-before-define
/*eslint-enable */
import { Ace as AceEditor } from "vue2-brace-editor";

import "./scilla_mode";
import "brace/ext/searchbox";
import "brace/ext/keybinding_menu";
import "brace/keybinding/emacs";
import "brace/keybinding/vim";
import "brace/mode/javascript";
import "brace/theme/tomorrow";

import axios from "axios";
import { mapGetters } from "vuex";

export default {
  props: ["file"],
  data() {
    return {
      code: null,
      changed: false,
      annotations: [],
      readonly: false,
      SCILLA_CHECKER_URL: process.env.VUE_APP_SCILLA_CHECKER_URL,
    };
  },
  computed: {
    ...mapGetters("networks", { network: "selected" }),
    ...mapGetters("general", { editor: "editor" }),
  },
  methods: {
    handleInput(payload) {
      this.changed = true;
      this.file.code = payload;
    },
    async handleSave() {
      await this.$store.dispatch("files/UpdateCode", {
        id: this.file.id,
        code: this.file.code,
      });
      this.changed = false;
    },
    handleDeploy() {
      window.EventBus.$emit("open-deploy-contract", this.file);
    },
    async handleCheck() {
      this.annotations = [];

      window.EventBus.$emit("console-log", {
        message: `Running checker on ${this.file} contract.`,
      });

      axios
        .post(this.SCILLA_CHECKER_URL, {
          code: this.file.code,
        })
        .then((response) => {
          if (response.data.result === "success") {
            const message = JSON.parse(response.data.message);

            if (message.warnings !== []) {
              const markers = message.warnings.map((err) => {
                const row = parseInt(err.start_location.line, 10);
                const column = parseInt(err.start_location.column, 10);

                return {
                  row: row === 0 ? 0 : row - 1,
                  column,
                  type: "warning",
                  text: err.warning_message,
                };
              });

              window.EventBus.$emit("checker-events", { warnings: markers });

              this.annotations = markers;
            }
            // this.checked = true;
            window.EventBus.$emit("console-log", {
              message: `Contract check successfully passed.`,
            });
            this.$notify({
              group: "scilla",
              type: "success",
              position: "bottom right",
              title: "Scilla Checker",
              text: "Contract has been successfully checked",
            });
          }
        })
        .catch((error) => {
          window.EventBus.$emit("console-log", {
            message: `There are errors in your contract.`,
            type: "error",
          });
          this.$notify({
            group: "scilla",
            type: "error",
            position: "bottom right",
            title: "Scilla Checker",
            text: "There are errors in your contract. Check the editor.",
          });
          const markers = error.response.data.message.map((err) => {
            const row = parseInt(err.line, 10);
            const col = parseInt(err.column, 10);

            return {
              row: row === 0 ? 0 : row - 1,
              column: col,
              type: "error",
              text: err.msg,
            };
          });

          window.EventBus.$emit("checker-events", { errors: markers });

          this.annotations = markers;
        });
    },
  },
  components: {
    AceEditor,
  },
  mounted() {
    this.changed = false;

    window.EventBus.$on("console-run-checker", () => {
      this.handleCheck();
    });

    window.EventBus.$on("change-editor-fontSize", (payload) => {
      this.fontSize = payload;
    });

    window.EventBus.$on("open-editor-contract", ({ contractId }) => {
      /* this.changeEditorCode({
        type: "deployed-contract",
        contractId: contractId
      }); */
      this.$store.dispatch("contracts/SelectContract", { contractId });
    });
  },
};
</script>

<style lang="scss" scoped>
.editor {
  height: 100%;

  .ace_scrollbar.ace_scrollbar-h {
    display: none !important;
  }

  .actions-bar {
    padding: 0.5rem;
    padding-left: 0;
    font-size: 0.85rem;
    height: 50px;
  }

  .editor-inner {
    height: calc(100% - 50px);
    overflow-y: scroll;
  }

  .btn {
    font-family: "Montserrat", sans-serif;

    color: #000;
    font-weight: 500;
    font-size: 0.85rem;

    background-color: lighten($primary, 40);

    &:hover {
      background-color: rgba(0, 0, 0, 0.1);
      text-decoration: none;
    }

    img {
      height: 1rem;
    }
  }
}
</style>