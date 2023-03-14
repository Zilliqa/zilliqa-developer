<template>
  <div class="checker-results">
    <div class="content">
      <ul>
        <li v-for="(item,index) in events" :key="index">
          <span
            class="badge"
            :class="{'badge-warning': item.type === 'warning', 'badge-danger': item.type === 'error'}"
          >{{item.row + 1}}, {{item.column}}</span>
          {{item.text}}
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      events: []
    };
  },
  mounted() {
    window.EventBus.$on("checker-events", ({ warnings, errors }) => {
      this.events = [];
      if (warnings !== undefined) {
        warnings.forEach(item => {
          this.events.push(item);
        });
      }

      if (errors !== undefined) {
        errors.forEach(item => {
          this.events.push(item);
        });
      }
    });
  }
};
</script>

<style lang="scss" scoped>
.checker-results {
  .content {
    padding: 0.5rem;
  }

  ul {
    padding: 0;
    list-style: none;
    li {
      font-size: 12px;
    }
  }
}
</style>