<template>
  <div class="home">
    <Editor v-if="selectedFile || selectedContract" :file="selectedContract || selectedFile" />
    <div class="welcome p-5" v-else>
      <h2>Welcome to Neo Savant IDE!</h2>
      <p>Learn more about what you can do by visiting <a href="https://scilla.readthedocs.io/en/latest/scilla-trial.html" target="_blank">Scilla Docs</a> or by exploring an Example Contract from the Files section.</p>
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
import Editor from "@/components/Editor";
import { mapGetters } from "vuex";

export default {
  name: "home",
  components: {
    Editor
  },
  computed: {
    ...mapGetters("files", {selectedFile: "selected"}),
    ...mapGetters("contracts", {selectedContract: "selected"})
  }
};
</script>

<style lang="scss" scoped>
.home {
  height: 100%;
}
</style>
