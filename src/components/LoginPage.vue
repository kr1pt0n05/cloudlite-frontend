<script setup lang="ts">
import { computed, ref } from "vue";
import logo from "../assets/cloudlite-logo.png";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faChevronRight,
  faKey,
  faServer,
  faShieldHalved,
} from "@fortawesome/free-solid-svg-icons";

const serverUrl = ref("");
const submittedUrl = ref("");

const normalizedUrl = computed(() => {
  const value = serverUrl.value.trim();

  if (!value) {
    return "";
  }

  return /^https?:\/\//i.test(value) ? value : `https://${value}`;
});

function submitServer() {
  submittedUrl.value = normalizedUrl.value;
}
</script>

<template>
  <main class="flex h-screen w-screen flex-col overflow-hidden bg-background text-foreground">
    <section
      class="grid min-h-0 flex-1 grid-cols-1 overflow-hidden bg-surface md:grid-cols-[1.05fr_1fr]"
      aria-labelledby="login-title"
    >
      <div class="relative hidden flex-col justify-start gap-12 bg-linear-to-br from-primary-soft
      via-surface to-surface px-12 py-12 pb-16 md:flex xl:pl-48 xl:pt-32">
        <div>
          <div class="flex items-center gap-2.5">
            <div class="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/15 ring-1 ring-primary/25">
              <img :src="logo" alt="" class="h-6 w-6" />
            </div>
            <div>
              <div class="text-[15px] font-semibold tracking-tight">CloudLite</div>
              <div class="text-[11px] text-muted-foreground">Self-hosted · Lightweight</div>
            </div>
          </div>

          <h1 class="mt-10 text-[26px] font-semibold leading-tight tracking-tight">
            Your files,<br />
            on your server.
          </h1>
          <p class="mt-3 max-w-75 text-[13px] leading-relaxed text-muted-foreground">
            A faster, simpler alternative to heavy self-hosted suites. Built for power users who want control without the bloat.
          </p>
        </div>
        <ul class="space-y-2.5 text-[12px] text-foreground/80">
          <li class="flex items-center gap-2"><FontAwesomeIcon class="h-4 w-4 text-primary" :icon="faShieldHalved" /> End-to-end encrypted upload sessions</li>
          <li class="flex items-center gap-2"><FontAwesomeIcon class="h-4 w-4 text-primary" :icon="faServer" />Connect to any CloudLite server</li>
          <li class="flex items-center gap-2"><FontAwesomeIcon class="h-4 w-4 text-primary" :icon="faKey" /> OAuth, Keycloak & token auth</li>
        </ul>
      </div>

      <form class="flex min-h-0 flex-col justify-start gap-6 overflow-y-auto
      px-6 py-8 pb-14 sm:px-10 sm:py-12 sm:pb-16 lg:px-12 xl:pr-48 xl:pt-50 sm:pt-32"
            @submit.prevent="submitServer">
        <div class="flex items-center gap-3 md:hidden">
          <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/15 ring-1 ring-primary/25">
            <img :src="logo" alt="" class="h-7 w-7" />
          </div>
          <div>
            <div class="text-[15px] font-semibold tracking-tight">CloudLite</div>
            <div class="text-[11px] text-muted-foreground">Self-hosted lightweight files</div>
          </div>
        </div>

        <div>
          <h2 id="login-title" class="text-[19px] font-semibold tracking-tight">Connect to a server</h2>
          <p class="mt-1 text-[12px] text-muted-foreground">Enter your CloudLite URL to continue.</p>
        </div>

        <label class="block">
          <span class="mb-1.5 block text-[12px] font-medium">Server URL</span>
          <div
            class="flex h-10 items-center overflow-hidden rounded-md border border-input bg-surface focus-within:border-ring focus-within:ring-2 focus-within:ring-ring/20"
          >
            <span class="border-r border-border bg-surface-2 px-3 text-[12px] text-muted-foreground">https://</span>
            <input
              v-model="serverUrl"
              class="h-full min-w-0 flex-1 bg-transparent px-3 text-[13px] outline-none placeholder:text-muted-foreground"
              type="text"
              inputmode="url"
              autocomplete="url"
              placeholder="your-server.example.com"
              required
            />
          </div>
        </label>

        <button
          class="inline-flex h-10 w-full items-center justify-center rounded-md bg-primary px-4 text-[13px] font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-ring/25"
          type="submit"
        >
          Continue
          <FontAwesomeIcon class="pl-2" :icon="faChevronRight"/>
        </button>

        <p v-if="submittedUrl" class="rounded-md border border-border bg-surface-2 px-3 py-2 text-[12px] text-muted-foreground">
          Connecting to <span class="font-medium text-foreground">{{ submittedUrl }}</span>
        </p>
      </form>
    </section>

    <footer class="flex shrink-0 flex-col gap-3 border-t border-border bg-surface px-6 py-4 sm:px-10 lg:flex-row lg:items-center lg:justify-end lg:px-12 xl:px-16">
      <span class="text-right text-[11px] font-medium text-muted-foreground">tauri-build v0.1.0</span>
    </footer>
  </main>
</template>
