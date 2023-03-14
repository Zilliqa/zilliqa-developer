<template>
  <v-popover offset="16" trigger="click" placement="top">
    <!-- This will be the popover target (for the events and position) -->
    <a class="tooltip-target d-flex align-items-center">
      <span>{{ type }}</span>
      <img class="ml-1" src="@/assets/question.svg" />
    </a>

    <!-- This will be the content of the popover -->
    <template slot="popover" v-if="selected !== undefined">
      {{selected.description}} Read more about this
      <a :href="selected.link" target="_blank">here</a>
    </template>
    <template slot="popover" v-else>{{type}}</template>
  </v-popover>
</template>

<script>
import { getParamType } from "@/utils/validation";

export default {
  data() {
    return {
      texts: [
        {
          type: "Uint",
          description:
            "Scilla defines signed and unsigned integer types of 32, 64, 128, and 256 bits. These integer types can be specified with the keywords IntX and UintX where X can be 32, 64, 128, or 256. For example, the type of an unsigned integer of 32 bits is Uint32.",
          link:
            "https://scilla.readthedocs.io/en/latest/scilla-in-depth.html#integer-types"
        },
        {
          type: "String",
          description:
            "String literals in Scilla are expressed using a sequence of characters enclosed in double quotes. Variables can be declared by specifying using keyword String.",
          link:
            "https://scilla.readthedocs.io/en/latest/scilla-in-depth.html#strings"
        },
        {
          type: "ByStr20",
          description:
            "An address in Scilla is declared using the data type ByStr20. ByStr20 represents a hexadecimal byte string of 20 bytes (40 hexadecimal characters). A ByStr20 literal is prefixed with 0x",
          link:
            "https://scilla.readthedocs.io/en/latest/scilla-in-depth.html#addresses"
        },
        {
          type: "List",
          description:
            "Lists of values are specified using the type List t, where t is some type. All elements in a list must be of the same type t. In other words, two values of different types cannot be added to the same list.",
          link:
            "https://scilla.readthedocs.io/en/latest/scilla-in-depth.html#list"
        }
      ],
      selected: undefined
    };
  },
  props: ["type"],
  mounted() {
    const foundType = getParamType({ type: this.type });

    this.selected = this.texts.find(item => item.type === foundType);
  }
};
</script>

<style lang="scss" scoped>
.tooltip-target {
  cursor: pointer;
  img {
    height: 12px;
  }
}
</style>