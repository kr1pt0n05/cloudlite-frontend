<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faDownload, faTrash, faXmark } from "@fortawesome/free-solid-svg-icons";

defineProps<{
  selectedCount: number;
}>();

const emit = defineEmits<{
  clear: [];
  deleteSelected: [];
  downloadSelected: [];
}>();
</script>

<template>
  <div
    v-if="selectedCount > 0"
    class="flex items-center gap-3 border-b border-border bg-primary-soft px-4 py-2 text-[12px] text-primary"
  >
    <span class="font-medium">{{ selectedCount }} selected</span>
    <button class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10" type="button" @click="emit('downloadSelected')">
      <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faDownload" />
      Download
    </button>
    <button class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10" type="button" @click="emit('deleteSelected')">
      <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faTrash" />
      Delete
    </button>
    <button class="ml-auto inline-flex items-center gap-1 text-[11px] hover:underline" type="button" @click="emit('clear')">
      <FontAwesomeIcon class="h-3 w-3" :icon="faXmark" />
      Clear
    </button>
  </div>
</template>
