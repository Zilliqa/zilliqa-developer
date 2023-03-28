<template>
  <div class="copy-to-clipboard mr-2" @click="handleCopy" title="Click to copy address">
    <i class="fas fa-copy" v-if="!done"></i>
    <i class="fas fa-check" v-else></i>
  </div>
</template>

<script>
export default {
  name: "CopyToClipboard",
  data() {
    return {
      done: false
    };
  },
  props: ["text"],
  methods: {
    handleCopy(ev) {
      ev.preventDefault();
      if (navigator.clipboard) {
        navigator.clipboard.writeText(this.text).then(() => {
          this.done = true;
          setTimeout(() => {
            this.done = false;
          }, 1000);
        });
      } else {
        const input = document.createElement("input");
        document.body.appendChild(input);
        input.value = this.text;
        input.focus();
        input.select();
        const result = document.execCommand("copy");
        if (result !== "unsuccessful") {
          this.done = true;
          setTimeout(() => {
            this.done = false;
          }, 1000);
        }
      }
    }
  }
};
</script>

<style lang="scss" scoped>
.copy-to-clipboard {
  width: 24px;
  height: 24px;
  font-size: 14px;
  border-radius: 50%;
  position: relative;
  transition: all 0.2s ease-in-out;

  .fas {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    opacity: 0.5;
    transition: all 0.2s ease-in-out;
  }

  &:hover {
    cursor: pointer;
    background-color: darken($primary, 5);

    .fas {
      opacity: 1;
    }
  }
}
</style>