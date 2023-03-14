<template>
  <div
    class="file-name mb-2"
    :class="{'selected' : selected}"
    @click="$emit('select-contract', {contractId: contract.contractId})"
    @contextmenu.prevent="$refs.menu.open"
  >
    <div class="address">
      <address-display :address="contract.contractId"></address-display>
    </div>
    <div class="d-flex tags">
      <div
        v-for="(tag,index) in tags"
        :key="index"
        class="badge mr-1"
        :style="{'background-color': tag.color, 'color' : lightOrDark(tag.color) === 'light' ? '#000' : '#fff'}"
        @click="handleRemoveTag(index)"
      >
        <span class="remove">X</span>
        {{ tag.value }}
      </div>
    </div>

    <vue-context class="context-menu" ref="menu">
      <li>
        <a href="#" @click.prevent="handleOpenTagModal">Add Tag</a>
      </li>
      <li>
        <a href="#" @click.prevent="handleRemove">Remove</a>
      </li>
    </vue-context>

    <modal v-if="tagModal">
      <div slot="header">Add tag to contract</div>
      <div slot="body">
        <label>Tag value</label>
        <input
          type="text"
          class="form-control mb-4"
          placeholder="Enter tag value"
          v-model="tagValue"
        />
        <label>Tag color</label>
        <compact-picker v-model="tagColor" />
      </div>
      <div slot="footer" class="d-flex">
        <button class="btn btn-light text-danger text-small mr-2" @click="closeModal">Close</button>
        <button class="btn btn-primary btn-block" @click="handleAddTag">Add Tag</button>
      </div>
    </modal>
  </div>
</template>

<script>
import Swal from "sweetalert2";
import VueContext from "vue-context";
import AddressDisplay from "../UI/AddressDisplay";
import { Compact } from "vue-color";
import Modal from "../UI/Modal";
import {lightOrDark} from "../../utils/ui.js";

export default {
  data() {
    return {
      lightOrDark,
      tagModal: false,
      tagValue: "",
      tagColor: "#ff0000",
      tags: this.contract.tags
    };
  },
  props: ["contract", "selected"],
  components: { VueContext, AddressDisplay, Modal, "compact-picker": Compact },
  watch: {
    "contract.tags": function(val) {
      this.tags = val;
    }
  },
  methods: {
    closeModal() {
      this.tagValue = "";
      this.tagColor = "#ff0000";
      this.tagModal = false;
    },
    handleRemove() {
      const confirmed = confirm(
        "Are you sure you want to remove this contract?"
      );
      if (confirmed) {
        this.$store
          .dispatch("contracts/RemoveContract", {
            id: this.contract.contractId
          })
          .then(() => {
            this.$notify({
              group: "scilla",
              type: "success",
              position: "bottom right",
              title: "Contracts",
              text: "Contract has been removed"
            });
          });
      }
    },
    async handleRemoveTag(tagIndex) {
      Swal.fire({
        title: "Are you sure?",
        text: "Confirm that you want to remove the tag.",
        icon: "warning",
        showCancelButton: true,
        confirmButtonText: "Yes"
      }).then(result => {
        if (result.value) {
          this.$store.dispatch("contracts/RemoveTag", {
            contractId: this.contract.contractId,
            tagIndex
          });
        }
      });
    },
    async handleOpenTagModal() {
      this.tagModal = true;
    },
    async handleAddTag() {
      await this.$store.dispatch("contracts/AddTag", {
        id: this.contract.contractId,
        tag: {
          value: this.tagValue,
          color: this.tagColor.hex ?? "#ff0000"
        }
      });
      this.closeModal();
    }
  }
};
</script>

<style lang="scss" scoped>
.file-name {
  font-size: 0.75rem;
  color: #000;
  padding-top: 5px;
  padding-bottom: 5px;
  .address {
    width: calc(100% - 0.5rem);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  &.selected {
    color: $primary;
    font-weight: bold;
    .tags {
      opacity: 1;
    }
  }
  &:hover {
    cursor: pointer;
    color: $primary;
    .tags {
      opacity: 1;
    }
  }
  .tags {
    transition: all 0.2s;
    opacity: 0.7;
    .badge {
      display: flex;
      align-items: center;
      padding: 4px;
      .remove {
        color: #fff;
        transition: all 0.2s;
        margin-right: 0px;
        margin-right: -6px;
        opacity: 0;
      }
      &:hover {
        .remove {
          opacity: 1;
          margin-right: 5px;
        }
      }
    }
  }
}
.context-menu {
  li {
    a {
      font-size: 14px;
    }
  }
}
</style>