<script setup lang="ts">
import { ref } from "vue";
import logo from "../assets/cloudlite-logo.png";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faChevronRight,
  faKey,
  faServer,
  faShieldHalved,
  faSpinner,
} from "@fortawesome/free-solid-svg-icons";

const serverUrl = ref("");
const submittedUrl = ref("");
const submittedDomain = ref("");
const urlError = ref("");

function submitServer() {
  const value = serverUrl.value.trim();

  submittedUrl.value = "";
  submittedDomain.value = "";
  urlError.value = "";

  if (!/^https?:\/\//i.test(value)) {
    urlError.value = "URL must start with http:// or https://";
    return;
  }

  try {
    const url = new URL(value);

    submittedUrl.value = value;
    submittedDomain.value = url.host;
  } catch {
    urlError.value = "Enter a valid server URL.";
  }
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
          <div class="flex items-center gap-4">
            <div class="flex h-14 w-14 items-center justify-center rounded-xl bg-primary/15 ring-1 ring-primary/25">
              <img :src="logo" alt="" class="h-10 w-10" />
            </div>
            <div>
              <div class="text-[22px] font-semibold tracking-tight">CloudLite</div>
              <div class="text-[13px] text-muted-foreground">Self-hosted · Lightweight</div>
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
        <ul class="space-y-4 text-[14px] font-medium text-foreground/85">
          <li class="flex items-center gap-3"><FontAwesomeIcon class="h-5 w-5 text-primary" :icon="faShieldHalved" /> End-to-end encrypted upload sessions</li>
          <li class="flex items-center gap-3"><FontAwesomeIcon class="h-5 w-5 text-primary" :icon="faServer" />Connect to any CloudLite server</li>
          <li class="flex items-center gap-3"><FontAwesomeIcon class="h-5 w-5 text-primary" :icon="faKey" /> OAuth, Keycloak & token auth</li>
        </ul>
      </div>

      <form class="flex min-h-0 flex-col justify-start gap-6 overflow-y-auto
      px-6 py-8 pb-14 sm:px-10 sm:py-12 sm:pb-16 lg:px-12 xl:pr-48 xl:pt-50 sm:pt-32"
            @submit.prevent="submitServer">
        <div class="flex items-center gap-4 md:hidden">
          <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/15 ring-1 ring-primary/25">
            <img :src="logo" alt="" class="h-9 w-9" />
          </div>
          <div>
            <div class="text-[20px] font-semibold tracking-tight">CloudLite</div>
            <div class="text-[13px] text-muted-foreground">Self-hosted lightweight files</div>
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
            <input
              v-model="serverUrl"
              class="h-full min-w-0 flex-1 bg-transparent px-3 text-[13px] outline-none placeholder:text-muted-foreground"
              type="url"
              inputmode="url"
              autocomplete="url"
              pattern="https?://.+"
              placeholder="https://your-server.example.com"
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

        <p v-if="urlError" class="rounded-md border border-border bg-surface-2 px-3 py-2 text-[12px] text-muted-foreground">
          {{ urlError }}
        </p>

        <p v-if="submittedDomain" class="flex items-center gap-2 rounded-md border border-border bg-surface-2 px-3 py-2 text-[12px] text-muted-foreground">
          <FontAwesomeIcon class="h-4 w-4 animate-spin text-primary" :icon="faSpinner" />
          <span>Connecting to <span class="font-medium text-foreground">{{ submittedDomain }}</span></span>
        </p>
      </form>
    </section>

    <footer class="flex shrink-0 flex-col gap-3 border-t border-border bg-surface px-6 py-4 sm:px-10 lg:flex-row lg:items-center lg:justify-end lg:px-12 xl:px-16">
      <span class="text-right text-[11px] font-medium text-muted-foreground">tauri-build v0.1.0</span>
    </footer>
  </main>
</template>
