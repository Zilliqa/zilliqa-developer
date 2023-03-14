<template>
  <div class="tools-panel" v-if="toolsPanel">
    <div class="header p-2">
      <div class="title">{{ tool.name }}</div>
      <img src="@/assets/close-color.svg" @click="handleClose" class="close-button" />
    </div>
    <div class="content p-4">
      <units-converter v-if="toolsPanel === 'units-converter'" />
      <address-converter v-if="toolsPanel === 'address-converter'" />
    </div>
  </div>
</template>

<script>
import UnitsConverter from "./UnitsConverter";
import AddressConverter from "./AddressConverter";

export default {
  name: "Tools",
  data() {
    return {
      toolsPanel: false,
      tools: [
        {
          slug: "units-converter",
          name: "Units Converter"
        },
        {
          slug: "address-converter",
          name: "Address Converter"
        }
      ]
    };
  },
  computed: {
    tool() {
      return this.tools.find(item => item.slug === this.toolsPanel);
    }
  },
  components: {
    UnitsConverter,
    AddressConverter
  },
  methods: {
    handleClose() {
      this.toolsPanel = false;
    }
  },
  mounted() {
    window.EventBus.$on("open-tools", toolName => {
      this.toolsPanel = toolName;
    });
  }
};
</script>

<style lang="scss" scoped>
.tools-panel {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 30%;
  height: 40%;
  min-width: 300px;
  min-height: 350px;
  z-index: 999;
  border: 2px solid $primary;
  background-color: #fff;
  border-top-right-radius: 10px;

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background-color: $primary;
    border-top-right-radius: 7px;

    .title {
      font-size: 1rem;
      color: #fff;
    }

    .close-button {
      cursor: pointer;
      height: 16px;
    }
  }
}
</style>