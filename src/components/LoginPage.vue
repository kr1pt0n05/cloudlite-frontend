<script setup lang="ts">
import { computed, ref } from "vue";
import logo from "../assets/cloudlite-logo.png";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
import {faChevronRight} from "@fortawesome/free-solid-svg-icons";

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
  <main class="flex min-h-screen w-full items-center justify-center bg-background p-6 text-foreground">
    <section
      class="grid w-full max-w-230 grid-cols-1 overflow-hidden rounded-2xl border border-border bg-surface shadow-lg md:grid-cols-[1.05fr_1fr]"
      aria-labelledby="login-title"
    >
      <div class="relative hidden flex-col justify-between bg-linear-to-br from-primary-soft via-surface to-surface p-8 md:flex">
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

      </div>

      <form class="flex flex-col gap-6 p-8" @submit.prevent="submitServer">
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
  </main>
</template>
