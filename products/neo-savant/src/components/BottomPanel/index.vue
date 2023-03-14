<template>
  <div class="bottom-panel">
    <div class="header" @click="handleToggle">
      <div class="tab d-flex align-items-center">
        <img height="13px" class="pr-1" src="@/assets/terminal.svg" /> Checker
      </div>
    </div>
    <div class="body" v-if="active">
      <checker-tab />
    </div>
  </div>
</template>

<script>
import CheckerTab from "./Checker";

export default {
  props: ["active"],
  components: { CheckerTab },
  methods: {
    handleToggle() {
      this.$emit("toggle");
    }
  }
};
</script>

<style lang="scss" scoped>
.bottom-panel {
  max-height: 150px;
  overflow: hidden;
  z-index: 9999;
  background-color: #fff;

  border-top: 1px solid transparentize($color: #ccc, $amount: 0.55);

  .header {
    width: 100%;
    font-size: 0.8rem;
    height: 1.25rem;
    display: flex;

    .tab {
      background-color: darken($color: #fff, $amount: 5);
      padding-left: 0.5rem;
      padding-right: 0.5rem;
    }

    &:hover {
      cursor: pointer;
      background-color: lighten($primary, $amount: 40);

      .tab {
        background-color: lighten($primary, $amount: 30);
      }
    }
  }

  .body {
    height: calc(150px - 1.25rem);
    background-color: darken($color: #fff, $amount: 5);
    overflow-y: scroll;
  }
}
</style>