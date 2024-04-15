<template>
    <el-dialog
    :model-value="show"
    title="Units Convertor"
    width="500"
  >
    <el-form class="demo-form-inline" label-position="right" label-width="auto">
      <el-form-item label="Zil">
        <el-input v-model="zil" @change="handleChangeZil"/>
      </el-form-item>
      <el-form-item label="Li">
        <el-input v-model="li" @change="handleChangeLi"/>
      </el-form-item>
      <el-form-item label="Qa">
        <el-input v-model="qa" @change="handleChangeQa"/>
      </el-form-item>
      <el-form-item>
        <el-button type="primary">Convert</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>  
</template>

<script setup>
import { ref, defineProps } from "vue";
import { BN, units } from "@zilliqa-js/util";

const zil = ref(0);
const li = ref(0);
const qa = ref(0);
defineProps(['show'])

const handleChangeZil = () => {
  const qa_internal = units.toQa(zil.value, units.Units.Zil);
  li.value = units.fromQa(new BN(qa_internal), units.Units.Li);
  qa.value = qa_internal;
}

const handleChangeLi = () => {
  const qa_internal = units.toQa(li.value, units.Units.Li);
  zil.value = units.fromQa(new BN(qa_internal), units.Units.Zil);
  qa.value = qa_internal;
}

const handleChangeQa = () => {
  li.value = units.fromQa(new BN(qa.value), units.Units.Li);
  zil.value = units.fromQa(new BN(qa.value), units.Units.Zil);
}
</script>

<style lang="scss" scoped>
.panel {
  margin-bottom: 2rem;
}
</style>
