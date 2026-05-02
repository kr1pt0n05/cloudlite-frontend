<script setup lang="ts">
import logo from "../assets/cloudlite-logo.png";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import type { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import {
  faFolderOpen,
  faGear,
  faHardDrive,
  faUpload,
} from "@fortawesome/free-solid-svg-icons";

type NavItem = {
  to: string;
  label: string;
  icon: IconDefinition;
  badge?: string;
};

const primary: NavItem[] = [
  { to: "/files", label: "My Files", icon: faFolderOpen },
];

const activity: NavItem[] = [
  { to: "/uploads", label: "Uploads", icon: faUpload, badge: "3" },
];

const footer: NavItem[] = [{ to: "/settings", label: "Settings", icon: faGear }];
</script>

<template>
  <aside class="hidden h-full w-[232px] shrink-0 flex-col border-r border-sidebar-border bg-sidebar sm:flex">
    <div class="flex items-center gap-2.5 px-3 py-3">
      <div class="flex h-7 w-7 items-center justify-center rounded-md bg-primary/10 ring-1 ring-primary/20">
        <img :src="logo" alt="CloudLite" class="h-5 w-5" />
      </div>
      <div class="min-w-0 flex-1">
        <div class="truncate text-[13px] font-semibold text-sidebar-foreground">CloudLite</div>
        <div class="flex items-center gap-1 truncate text-[11px] text-muted-foreground">
          <span class="h-1.5 w-1.5 rounded-full bg-success" />
          cloud.acme.dev
        </div>
      </div>
    </div>

    <nav class="flex-1 space-y-1 overflow-y-auto px-2 pb-2" aria-label="Primary navigation">
      <div class="space-y-0.5">
        <RouterLink
          v-for="item in primary"
          :key="item.to"
          :to="item.to"
          class="group flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-[13px] font-medium text-sidebar-foreground/80 transition-colors hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground"
          active-class="bg-sidebar-accent text-sidebar-accent-foreground"
        >
          <FontAwesomeIcon class="h-[15px] w-[15px] text-muted-foreground group-hover:text-foreground" :icon="item.icon" />
          <span class="flex-1 truncate">{{ item.label }}</span>
        </RouterLink>
      </div>

      <div class="space-y-0.5">
        <div class="px-2.5 pb-1 pt-3 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/70">
          Activity
        </div>
        <RouterLink
          v-for="item in activity"
          :key="item.to"
          :to="item.to"
          class="group flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-[13px] font-medium text-sidebar-foreground/80 transition-colors hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground"
          active-class="bg-sidebar-accent text-sidebar-accent-foreground"
        >
          <FontAwesomeIcon class="h-[15px] w-[15px] text-muted-foreground group-hover:text-foreground" :icon="item.icon" />
          <span class="flex-1 truncate">{{ item.label }}</span>
          <span v-if="item.badge" class="rounded bg-primary/10 px-1.5 text-[10px] font-semibold leading-4 text-primary">
            {{ item.badge }}
          </span>
        </RouterLink>
      </div>
    </nav>

    <div class="mx-2 mb-2 rounded-lg border border-sidebar-border bg-surface px-3 py-2.5 shadow-xs">
      <div class="mb-1.5 flex items-center justify-between text-[11px]">
        <span class="flex items-center gap-1.5 font-medium text-foreground">
          <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faHardDrive" />
          Storage
        </span>
        <span class="text-muted-foreground">62%</span>
      </div>
      <div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
        <div class="h-full rounded-full bg-primary" style="width: 62%" />
      </div>
      <div class="mt-1.5 text-[11px] text-muted-foreground">124.5 GB of 200 GB used</div>
    </div>

    <div class="border-t border-sidebar-border px-2 py-2">
      <RouterLink
        v-for="item in footer"
        :key="item.to"
        :to="item.to"
        class="group flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-[13px] font-medium text-sidebar-foreground/80 transition-colors hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground"
        active-class="bg-sidebar-accent text-sidebar-accent-foreground"
      >
        <FontAwesomeIcon class="h-[15px] w-[15px] text-muted-foreground group-hover:text-foreground" :icon="item.icon" />
        <span class="flex-1 truncate">{{ item.label }}</span>
      </RouterLink>
      <div class="mt-2 flex items-center gap-2 rounded-md px-2 py-1.5">
        <div class="flex h-7 w-7 items-center justify-center rounded-full bg-primary/15 text-[11px] font-semibold text-primary">
          JD
        </div>
        <div class="min-w-0 flex-1">
          <div class="truncate text-[12px] font-medium text-foreground">Jamie Dolan</div>
          <div class="truncate text-[11px] text-muted-foreground">jamie@acme.dev</div>
        </div>
      </div>
    </div>
  </aside>
</template>
