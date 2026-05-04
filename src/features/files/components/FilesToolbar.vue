<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faChevronRight,
  faFolderPlus,
  faMagnifyingGlass,
  faUpload,
} from "@fortawesome/free-solid-svg-icons";
import {invoke} from "@tauri-apps/api/core";

defineProps<{
  search: string;
}>();

const emit = defineEmits<{
  "update:search": [value: string];
  createFolder: [];
  upload: [];
}>();

function trigger() {
  invoke<void>("get_change_logs");
}

</script>

<template>
  <header class="border-b border-border bg-surface px-4 py-3">
    <div class="flex flex-col gap-3 lg:flex-row lg:items-center">
      <nav class="flex items-center gap-1 text-[13px]" aria-label="Breadcrumb">
        <span class="rounded px-1.5 py-0.5 font-medium text-foreground">My Files</span>
        <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faChevronRight" />
        <span class="rounded px-1.5 py-0.5 text-muted-foreground">Root</span>
      </nav>

      <div class="flex min-w-0 flex-1 items-center gap-2 rounded-md border border-input bg-surface-2 px-2.5 text-[12px] lg:ml-4">
        <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faMagnifyingGlass" />
        <input
          :value="search"
          class="h-8 min-w-0 flex-1 bg-transparent outline-none placeholder:text-muted-foreground"
          placeholder="Search root folders..."
          type="search"
          @input="emit('update:search', ($event.target as HTMLInputElement).value)"
        />
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <button
          class="inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-surface px-2.5 text-[12px] font-medium hover:bg-surface-hover"
          type="button"
          @click="emit('createFolder')"
        >
          <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faFolderPlus" />
          New folder
        </button>
        <button
          class="inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-[12px] font-medium text-primary-foreground shadow-sm hover:bg-primary/90"
          type="button"
          @click="trigger"
        >
          <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faUpload" />
          Upload
        </button>
      </div>
    </div>
  </header>
</template>
