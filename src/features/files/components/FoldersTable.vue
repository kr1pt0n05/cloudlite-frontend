<script setup lang="ts">
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faCheckCircle,
  faDownload,
  faEllipsis,
  faFile,
  faFolder,
  faPen,
  faRotate,
  faTrash,
} from "@fortawesome/free-solid-svg-icons";
import type { RootFolder } from "../services/filesService";

const props = defineProps<{
  folders: RootFolder[];
  selected: Set<string>;
  actionFolderId: string | null;
  renamingId: string | null;
  renameValue: string;
}>();

const emit = defineEmits<{
  toggleFolder: [id: string];
  toggleAll: [];
  beginRename: [folder: RootFolder];
  saveRename: [];
  deleteFolder: [id: string];
  openFolder: [folder: RootFolder];
  "update:actionFolderId": [id: string | null];
  "update:renamingId": [id: string | null];
  "update:renameValue": [value: string];
}>();

function toggleActions(id: string) {
  emit("update:actionFolderId", props.actionFolderId === id ? null : id);
}
</script>

<template>
  <div class="relative min-h-0 flex-1 overflow-auto">
    <table class="w-full border-separate border-spacing-0 text-[13px]">
      <thead class="sticky top-0 z-10 bg-surface-2 text-[11px] uppercase tracking-wider text-muted-foreground">
        <tr>
          <th class="w-8 border-b border-border px-3 py-2 text-left">
            <input
              class="h-3.5 w-3.5 rounded border-border accent-primary"
              type="checkbox"
              :checked="folders.length > 0 && selected.size === folders.length"
              aria-label="Select all folders"
              @change="emit('toggleAll')"
            />
          </th>
          <th class="border-b border-border px-3 py-2 text-left font-semibold">Name</th>
          <th class="w-[120px] border-b border-border px-3 py-2 text-right font-semibold">Items</th>
          <th class="hidden w-[110px] border-b border-border px-3 py-2 text-right font-semibold lg:table-cell">Size</th>
          <th class="hidden w-[150px] border-b border-border px-3 py-2 text-left font-semibold lg:table-cell">Modified</th>
          <th class="w-[110px] border-b border-border px-3 py-2 text-left font-semibold">Sync</th>
          <th class="w-12 border-b border-border" />
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="folder in folders"
          :key="folder.id"
          class="group"
          :class="selected.has(folder.id) ? 'bg-primary-soft/60' : 'hover:bg-surface-hover'"
          @click="emit('openFolder', folder)"
        >
          <td class="border-b border-border/60 px-3 py-2">
            <input
              class="h-3.5 w-3.5 rounded border-border accent-primary"
              type="checkbox"
              :checked="selected.has(folder.id)"
              :aria-label="`Select ${folder.name}`"
              @click.stop
              @change="emit('toggleFolder', folder.id)"
            />
          </td>
          <td class="border-b border-border/60 px-3 py-2">
            <div class="flex min-w-0 items-center gap-2.5">
              <FontAwesomeIcon class="h-4 w-4 shrink-0 text-primary" :icon="folder.isDirectory ? faFolder : faFile" />
              <input
                v-if="renamingId === folder.id"
                :value="renameValue"
                class="h-7 min-w-0 max-w-[360px] flex-1 rounded-md border border-input bg-surface px-2 text-[13px] outline-none focus:border-ring focus:ring-2 focus:ring-ring/20"
                @click.stop
                @input="emit('update:renameValue', ($event.target as HTMLInputElement).value)"
                @keydown.enter.prevent="emit('saveRename')"
                @keydown.esc.prevent="emit('update:renamingId', null)"
                @blur="emit('saveRename')"
              />
              <span v-else class="truncate font-medium text-foreground">{{ folder.name }}</span>
            </div>
          </td>
          <td class="border-b border-border/60 px-3 py-2 text-right tabular-nums text-muted-foreground">
            {{ folder.folders }} · {{ folder.files }}
          </td>
          <td class="hidden border-b border-border/60 px-3 py-2 text-right tabular-nums text-muted-foreground lg:table-cell">
            {{ folder.size }}
          </td>
          <td class="hidden border-b border-border/60 px-3 py-2 text-muted-foreground lg:table-cell">{{ folder.modified }}</td>
          <td class="border-b border-border/60 px-3 py-2">
            <span
              class="inline-flex h-5 items-center gap-1 rounded px-1.5 text-[11px] font-medium"
              :class="{
                'bg-success-soft text-success': folder.sync === 'synced',
                'bg-info-soft text-info': folder.sync === 'syncing',
                'bg-warning-soft text-warning-foreground': folder.sync === 'pending',
              }"
            >
              <FontAwesomeIcon
                class="h-3 w-3"
                :class="{ 'animate-spin': folder.sync === 'syncing' }"
                :icon="folder.sync === 'syncing' ? faRotate : faCheckCircle"
              />
              {{ folder.sync === "synced" ? "Synced" : folder.sync === "syncing" ? "Syncing" : "Pending" }}
            </span>
          </td>
          <td class="relative border-b border-border/60 px-3 py-2 text-right">
            <button
              class="rounded p-1 text-muted-foreground opacity-100 hover:bg-surface-hover hover:text-foreground sm:opacity-0 sm:group-hover:opacity-100"
              type="button"
              :aria-label="`Actions for ${folder.name}`"
              @click.stop="toggleActions(folder.id)"
            >
              <FontAwesomeIcon class="h-4 w-4" :icon="faEllipsis" />
            </button>
            <div
              v-if="actionFolderId === folder.id"
              class="absolute right-3 top-9 z-20 w-44 overflow-hidden rounded-lg border border-border bg-popover py-1 text-left shadow-pop"
              @click.stop
            >
              <button class="flex w-full items-center gap-2.5 px-3 py-1.5 text-[12px] hover:bg-surface-hover" type="button" @click="emit('beginRename', folder)">
                <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faPen" />
                Rename
              </button>
              <button class="flex w-full items-center gap-2.5 px-3 py-1.5 text-[12px] hover:bg-surface-hover" type="button">
                <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faDownload" />
                Download
              </button>
              <div class="my-1 h-px bg-border" />
              <button class="flex w-full items-center gap-2.5 px-3 py-1.5 text-[12px] text-destructive hover:bg-destructive-soft" type="button" @click="emit('deleteFolder', folder.id)">
                <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faTrash" />
                Delete
              </button>
            </div>
          </td>
        </tr>
      </tbody>
    </table>

    <div v-if="folders.length === 0" class="flex h-48 items-center justify-center text-[13px] text-muted-foreground">
      No folders match your search.
    </div>
  </div>
</template>
