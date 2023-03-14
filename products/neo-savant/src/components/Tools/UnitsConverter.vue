<template>
  <div class="panel">
    <div class="form-group mb-2">
      <label>Zil</label>
      <input type="text" class="form-control mb-2" v-model="zil" @change="handleChangeZil" />
    </div>

    <div class="form-group mb-2">
      <label>Li</label>
      <input type="text" class="form-control mb-2" v-model="li" @change="handleChangeLi" />
    </div>

    <div class="form-group">
      <label>Qa</label>
      <input type="text" class="form-control mb-2" v-model="qa" @change="handleChangeQa" />
    </div>
  </div>
</template>

<script>
import { BN, units } from "@zilliqa-js/util";

export default {
  data() {
    return {
      zil: 0,
      li: 0,
      qa: 0
    };
  },
  methods: {
    handleChangeZil() {
      const qa = units.toQa(this.zil, units.Units.Zil);
      this.li = units.fromQa(new BN(qa), units.Units.Li);
      this.qa = qa;
    },
    handleChangeLi() {
      const qa = units.toQa(this.li, units.Units.Li);
      this.zil = units.fromQa(new BN(qa), units.Units.Zil);
      this.qa = qa;
    },
    handleChangeQa() {
      this.li = units.fromQa(new BN(this.qa), units.Units.Li);
      this.zil = units.fromQa(new BN(this.qa), units.Units.Zil);
    }
  }
};
</script>

<style lang="scss" scoped>
.panel {
  margin-bottom: 2rem;
}
</style>