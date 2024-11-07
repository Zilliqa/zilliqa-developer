<template>
  <div class="file-name" :class="{ 'selected': selected }" @click="$emit('select-file', file.id)"
    @contextmenu="onContextMenu">
    <span class="editable" @dblclick="edit = true" v-if="!edit">{{ file.name }}</span>
    <input v-if="edit" v-model="newFileName" @blur="edit = false; newFileName = file.name"
      @keyup.enter="handleRename" @keyup.esc = "edit = false; newFileName = file.name"/>
    <span class="extension">.scilla</span>
  </div>
</template>

<script setup>
import ContextMenu from '@imengyu/vue3-context-menu'
import { defineProps, ref, defineEmits } from 'vue';
import { useStore } from 'vuex';
import { notify } from "@kyvg/vue3-notification";

const store = useStore();
const props = defineProps(["file", "selected"])
defineEmits(['select-file'])
const edit = ref(false)
const newFileName = ref(props.file.name)

const handleRename = () => {
  console.log("here")
  edit.value = false;

  store
    .dispatch("files/RenameFile", {
      id: props.file.id,
      name: newFileName.value
    })
    .then(() => {
      notify({
        group: "scilla",
        type: "success",
        position: "bottom right",
        title: "Files",
        text: "File has been renamed"
      });
    });
}

const handleDelete = () => {
  const confirmed = confirm("Are you sure you want to delete this file?");

  if (confirmed) {
    store
      .dispatch("files/RemoveFile", {
        id: props.file.id
      })
      .then(() => {
        notify({
          group: "scilla",
          type: "success",
          position: "bottom right",
          title: "Files",
          text: "File has been deleted"
        });
      });
  }
}

const onContextMenu = (e) => {
  // Prevent the browser's default menu
  e.preventDefault();
  ContextMenu.showContextMenu({
    x: e.x,
    y: e.y,
    items: [
      {
        label: "Rename",
        onClick: () => {
          edit.value = true;
        }
      },
      {
        label: "Delete",
        onClick: () => {
          handleDelete()
        }
      },
    ]
  });
}
</script>

<style lang="scss" scoped>
.file-name {
  font-size: 0.85rem;
  color: #000;

  &.selected {
    color: $primary;
  }

  &:hover {
    cursor: pointer;
    color: $primary;
  }
}

.context-menu {
  li {
    a {
      font-size: 14px;
    }
  }
}
</style>
