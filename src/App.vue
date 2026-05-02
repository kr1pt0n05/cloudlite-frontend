<script setup lang="ts">
import { computed } from "vue";
import { RouterView, useRoute } from "vue-router";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faMagnifyingGlass, faMinus, faSquare, faXmark } from "@fortawesome/free-solid-svg-icons";
import NavBar from "./components/NavBar.vue";

const route = useRoute();
const showShell = computed(() => route.name !== "login");
</script>

<template>
  <RouterView v-if="!showShell" />

  <div v-else class="app-window flex h-screen w-screen flex-col overflow-hidden bg-background text-foreground">
    <div class="app-titlebar flex items-center justify-between px-3">
      <div class="flex min-w-0 items-center gap-2 text-[12px] text-muted-foreground">
        <span class="font-medium text-foreground">CloudLite</span>
        <span class="text-border-strong">·</span>
        <span class="truncate">cloud.acme.dev</span>
      </div>

      <div class="pointer-events-none hidden flex-1 justify-center md:flex">
        <div
          class="pointer-events-auto flex h-6 w-[320px] items-center gap-2 rounded-md border border-border bg-surface-2 px-2 text-[12px] text-muted-foreground shadow-xs"
        >
          <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faMagnifyingGlass" />
          <span class="truncate">Search files, folders, shares...</span>
          <span class="ml-auto rounded bg-muted px-1.5 font-mono text-[10px]">Ctrl K</span>
        </div>
      </div>

      <div class="flex items-center gap-1 text-muted-foreground">
        <button class="flex h-6 w-6 items-center justify-center rounded hover:bg-surface-hover" type="button" aria-label="Minimize">
          <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faMinus" />
        </button>
        <button class="flex h-6 w-6 items-center justify-center rounded hover:bg-surface-hover" type="button" aria-label="Maximize">
          <FontAwesomeIcon class="h-3 w-3" :icon="faSquare" />
        </button>
        <button
          class="flex h-6 w-6 items-center justify-center rounded hover:bg-destructive hover:text-destructive-foreground"
          type="button"
          aria-label="Close"
        >
          <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faXmark" />
        </button>
      </div>
    </div>

    <div class="flex min-h-0 flex-1 overflow-hidden">
      <NavBar />
      <main class="flex min-w-0 flex-1 flex-col overflow-hidden bg-background">
        <RouterView />
      </main>
    </div>
  </div>
</template>
