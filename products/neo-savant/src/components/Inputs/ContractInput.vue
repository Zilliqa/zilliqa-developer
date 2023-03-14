<template>
  <div class="input-contract">
    <label>
      <div class="name">{{ vname }}</div>
      <div class="type">
        <input-popover :type="type"></input-popover>
      </div>
    </label>
    {{ inputType }} {{ type }}
    <input
      type="text"
      :value="pvalue"
      class="form-control"
      :class="{ 'has-errors': error }"
      ref="paramValue"
      @input="updateData"
      v-if="(inputType && inputType !== 'List') || inputType === type"
    />
    <ace-editor
      :value="pvalue"
      :onChange="updateData"
      :fontSize="14"
      :showPrintMargin="false"
      :showGutter="false"
      :highlightActiveLine="true"
      ref="paramValue"
      mode="text"
      lang="json"
      theme="dawn"
      width="100%"
      height="150px"
      :class="{ 'ace_editor ace-dawn ace-has-errors': error }"
      :name="`contractInput-${vname}`"
      :editorProps="{ $blockScrolling: true }"
      v-else
    />

    <small class="text-small font-weight-bold text-danger" v-if="error">{{
      error
    }}</small>
  </div>
</template>

<script>
import InputPopover from "./InputPopover";
/*eslint-disable */
import brace from "brace"; // eslint-disable-line no-use-before-define
/*eslint-enable */
import { Ace as AceEditor } from "vue2-brace-editor";

import "brace/mode/json";
import "brace/theme/dawn";

import { getParamType } from "@/utils/validation.js";

export default {
  name: "ContractInput",
  computed: {
    inputType() {
      return getParamType({ type: this.type });
    },
  },
  props: ["vname", "type", "pvalue", "error"],
  components: { AceEditor, InputPopover },
  methods: {
    updateData(payload) {
      let newval = this.pvalue;

      if (payload.target === undefined) {
        newval = payload;
      } else {
        newval = payload.target.value;
      }

      this.$emit("input", newval);
    },
  },
};
</script>

<style lang="scss" scoped>
.input-contract {
  label {
    display: flex;
    align-items: center;
    justify-content: space-between;

    .name {
      color: rgb(143, 142, 142);
      font-weight: bold;
    }

    .type {
      font-size: 12px;
    }
  }

  input {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;
    background-color: #eee;
    border: 0;
    border-radius: 0;
  }
}

.has-errors {
  border-color: #dc3545 !important;
  background-color: transparentize($color: #dc3545, $amount: 0.5) !important;
}
.ace-has-errors .ace_content {
  border-color: #dc3545 !important;
  background-color: transparentize($color: #dc3545, $amount: 0.5) !important;
}
</style>