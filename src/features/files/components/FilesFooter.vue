<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faCheckCircle } from "@fortawesome/free-solid-svg-icons";

defineProps<{
  foldersCount: number;
  selectedCount: number;
}>();
</script>

<template>
  <footer class="flex items-center justify-between border-t border-border bg-surface px-4 py-1.5 text-[11px] text-muted-foreground">
    <span>{{ foldersCount }} folders · {{ selectedCount }} selected</span>
    <span class="flex items-center gap-1">
      <FontAwesomeIcon class="h-3 w-3 text-success" :icon="faCheckCircle" />
      Last sync 2 min ago
    </span>
  </footer>
</template>
