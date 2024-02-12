<template>
  <div
    class="token-card btn"
    @click="$emit('add')"
  >
    <h1 class="plus">+</h1>
  </div>
</template>

<script>

export default {
  name: 'AddTokenCard'
}
</script>

<style lang="scss">
.token-card {
  display: flex;
  flex-direction: column;
  align-items: center;

  padding: 10px;
  margin: 10px;

  border-radius: 0.375rem;
  max-width: 150px;
  width: 100%;
  height: 150px;

  border-color: #354960;

  & > .plus {
    color: #49c1bf;
    font-size: 89px;
  }

  &:hover {
    background-color: #354960;
    border-color: #354960;
  }
}
</style>
