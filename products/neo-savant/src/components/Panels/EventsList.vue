<template>
  <div class="events-list panel-content">
    <div class="header">
      <div class="title">Events</div>
      <img
        src="@/assets/close-color.svg"
        @click="handleClose"
        class="close-button-new"
      />
    </div>
    <div class="body p-4">
      <div
        class="text-danger text-right mb-4 clear-events"
        @click="handleClearEvents"
        v-if="events.length"
      >
        clear events
      </div>
      <div
        class="mb-4 event-item"
        v-for="(event, index) in events"
        :key="index"
      >
        <div class="remove-button" @click="handleRemove(index)">
          <img src="@/assets/rubbish.svg" />
        </div>
        <p class="item-header pb-2">
          <address-display :address="event.address"></address-display>
          _eventname:
          <span class="font-weight-bold">{{ event._eventname }}</span>
        </p>
        <vue-json-pretty :data="event.params" />
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import VueJsonPretty from "vue-json-pretty";
import AddressDisplay from "../UI/AddressDisplay";

export default {
  components: { VueJsonPretty, AddressDisplay },
  data() {
    return {};
  },
  computed: {
    ...mapGetters("events", { events: "list" }),
  },
  methods: {
    handleClearEvents() {
      this.$store.dispatch("events/ClearEvents").then(() => {
        this.$notify({
          group: "scilla",
          type: "success",
          position: "bottom right",
          title: "Events",
          text: "Events cleared",
        });
      });
    },
    handleClose() {
      window.EventBus.$emit("close-right-panel");
    },
    handleRemove(index) {
      this.$store.dispatch("events/RemoveEvent", { index }).then(() => {
        this.$notify({
          group: "scilla",
          type: "success",
          position: "bottom right",
          title: "Events",
          text: "Event removed",
        });
      });
    },
  },
};
</script>

<style lang="scss" scoped>
.clear-events {
  cursor: pointer;
}
.events-list {
  position: relative;

  .event-item {
    padding: 0.5rem;
    border: 1px dashed #ccc;
    position: relative;

    .item-header {
      border-bottom: 1px dashed #ccc;
    }

    .remove-button {
      position: absolute;
      top: 0;
      right: 0;
      padding: 5px;
      opacity: 0.5;
      display: none;
      width: 30px;
      background-color: $primary;

      &:hover {
        cursor: pointer;
        opacity: 1;
      }
    }

    &:hover {
      border: 1px solid $primary;
      .remove-button {
        display: block;
      }
    }
  }
}
</style>