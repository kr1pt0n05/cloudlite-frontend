<script setup lang="ts">
import { computed, ref } from "vue";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faCheckCircle,
  faChevronRight,
  faDownload,
  faEllipsis,
  faFilter,
  faFolder,
  faFolderPlus,
  faMagnifyingGlass,
  faPen,
  faRotate,
  faShareNodes,
  faStar,
  faTrash,
  faUpload,
  faXmark,
} from "@fortawesome/free-solid-svg-icons";

type SyncState = "synced" | "syncing" | "pending";

type RootFolder = {
  id: string;
  name: string;
  owner: string;
  folders: number;
  files: number;
  size: string;
  modified: string;
  sync: SyncState;
  shared?: boolean;
  starred?: boolean;
};

const folders = ref<RootFolder[]>([
  { id: "1", name: "Design assets", owner: "Jamie", folders: 12, files: 84, size: "18.4 GB", modified: "Today, 09:14", sync: "synced", shared: true },
  { id: "2", name: "Engineering", owner: "Pat", folders: 18, files: 211, size: "42.7 GB", modified: "Yesterday", sync: "synced" },
  { id: "3", name: "Product strategy", owner: "Jamie", folders: 6, files: 36, size: "4.2 GB", modified: "2 days ago", sync: "syncing", starred: true },
  { id: "4", name: "Client contracts", owner: "Morgan", folders: 4, files: 128, size: "9.8 GB", modified: "Apr 22", sync: "pending", shared: true },
  { id: "5", name: "Invoices", owner: "Finance", folders: 9, files: 302, size: "6.1 GB", modified: "Apr 18", sync: "synced" },
  { id: "6", name: "Archive", owner: "Admin", folders: 31, files: 470, size: "96.3 GB", modified: "Apr 10", sync: "synced" },
]);

const selected = ref<Set<string>>(new Set());
const actionFolderId = ref<string | null>(null);
const renamingId = ref<string | null>(null);
const renameValue = ref("");
const search = ref("");

const selectedFolders = computed(() => folders.value.filter((folder) => selected.value.has(folder.id)));
const filteredFolders = computed(() => {
  const query = search.value.trim().toLowerCase();
  if (!query) {
    return folders.value;
  }

  return folders.value.filter((folder) => folder.name.toLowerCase().includes(query) || folder.owner.toLowerCase().includes(query));
});


function replaceSelected(next: Set<string>) {
  selected.value = next;
}

function toggleFolder(id: string) {
  const next = new Set(selected.value);
  next.has(id) ? next.delete(id) : next.add(id);
  replaceSelected(next);
}

function toggleAll() {
  if (selected.value.size === filteredFolders.value.length) {
    replaceSelected(new Set());
    return;
  }

  replaceSelected(new Set(filteredFolders.value.map((folder) => folder.id)));
}

function beginRename(folder: RootFolder) {
  actionFolderId.value = null;
  renamingId.value = folder.id;
  renameValue.value = folder.name;
}

function saveRename() {
  const name = renameValue.value.trim();
  if (!renamingId.value || !name) {
    renamingId.value = null;
    return;
  }

  const folder = folders.value.find((item) => item.id === renamingId.value);
  if (folder) {
    folder.name = name;
    folder.modified = "Just now";
  }

  renamingId.value = null;
  renameValue.value = "";
}

function deleteFolder(id: string) {
  folders.value = folders.value.filter((folder) => folder.id !== id);
  const next = new Set(selected.value);
  next.delete(id);
  replaceSelected(next);
  actionFolderId.value = null;
}

function deleteSelected() {
  const ids = selected.value;
  folders.value = folders.value.filter((folder) => !ids.has(folder.id));
  replaceSelected(new Set());
}

function createFolder() {
  const nextNumber = folders.value.length + 1;
  folders.value.unshift({
    id: String(Date.now()),
    name: `Untitled folder ${nextNumber}`,
    owner: "Jamie",
    folders: 0,
    files: 0,
    size: "0 KB",
    modified: "Just now",
    sync: "pending",
  });
}
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
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
            v-model="search"
            class="h-8 min-w-0 flex-1 bg-transparent outline-none placeholder:text-muted-foreground"
            placeholder="Search root folders..."
            type="search"
          />
        </div>

        <div class="flex shrink-0 items-center gap-2">
          <button
            class="inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-surface px-2.5 text-[12px] font-medium hover:bg-surface-hover"
            type="button"
            @click="createFolder"
          >
            <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faFolderPlus" />
            New folder
          </button>
          <button class="inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-[12px] font-medium text-primary-foreground shadow-sm hover:bg-primary/90" type="button">
            <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faUpload" />
            Upload
          </button>
        </div>
      </div>
    </header>

    <div
      v-if="selected.size > 0"
      class="flex items-center gap-3 border-b border-border bg-primary-soft px-4 py-2 text-[12px] text-primary"
    >
      <span class="font-medium">{{ selected.size }} selected</span>
      <button class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10" type="button">
        <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faDownload" />
        Download
      </button>
      <button class="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10" type="button" @click="deleteSelected">
        <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faTrash" />
        Delete
      </button>
      <button class="ml-auto inline-flex items-center gap-1 text-[11px] hover:underline" type="button" @click="replaceSelected(new Set())">
        <FontAwesomeIcon class="h-3 w-3" :icon="faXmark" />
        Clear
      </button>
    </div>

    <div class="relative min-h-0 flex-1 overflow-auto">
      <table class="w-full border-separate border-spacing-0 text-[13px]">
        <thead class="sticky top-0 z-10 bg-surface-2 text-[11px] uppercase tracking-wider text-muted-foreground">
          <tr>
            <th class="w-8 border-b border-border px-3 py-2 text-left">
              <input
                class="h-3.5 w-3.5 rounded border-border accent-primary"
                type="checkbox"
                :checked="filteredFolders.length > 0 && selected.size === filteredFolders.length"
                aria-label="Select all folders"
                @change="toggleAll"
              />
            </th>
            <th class="border-b border-border px-3 py-2 text-left font-semibold">Name</th>
            <th class="hidden w-[150px] border-b border-border px-3 py-2 text-left font-semibold md:table-cell">Owner</th>
            <th class="w-[120px] border-b border-border px-3 py-2 text-right font-semibold">Items</th>
            <th class="hidden w-[110px] border-b border-border px-3 py-2 text-right font-semibold lg:table-cell">Size</th>
            <th class="hidden w-[150px] border-b border-border px-3 py-2 text-left font-semibold lg:table-cell">Modified</th>
            <th class="w-[110px] border-b border-border px-3 py-2 text-left font-semibold">Sync</th>
            <th class="w-12 border-b border-border" />
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="folder in filteredFolders"
            :key="folder.id"
            class="group cursor-default"
            :class="selected.has(folder.id) ? 'bg-primary-soft/60' : 'hover:bg-surface-hover'"
            @click="toggleFolder(folder.id)"
          >
            <td class="border-b border-border/60 px-3 py-2">
              <input
                class="h-3.5 w-3.5 rounded border-border accent-primary"
                type="checkbox"
                :checked="selected.has(folder.id)"
                :aria-label="`Select ${folder.name}`"
                @click.stop
                @change="toggleFolder(folder.id)"
              />
            </td>
            <td class="border-b border-border/60 px-3 py-2">
              <div class="flex min-w-0 items-center gap-2.5">
                <FontAwesomeIcon class="h-4 w-4 shrink-0 text-primary" :icon="faFolder" />
                <input
                  v-if="renamingId === folder.id"
                  v-model="renameValue"
                  class="h-7 min-w-0 max-w-[360px] flex-1 rounded-md border border-input bg-surface px-2 text-[13px] outline-none focus:border-ring focus:ring-2 focus:ring-ring/20"
                  @click.stop
                  @keydown.enter.prevent="saveRename"
                  @keydown.esc.prevent="renamingId = null"
                  @blur="saveRename"
                />
                <span v-else class="truncate font-medium text-foreground">{{ folder.name }}</span>
                <FontAwesomeIcon v-if="folder.starred" class="h-3.5 w-3.5 text-warning" :icon="faStar" />
                <FontAwesomeIcon v-if="folder.shared" class="h-3.5 w-3.5 text-muted-foreground" :icon="faShareNodes" />
              </div>
            </td>
            <td class="hidden border-b border-border/60 px-3 py-2 text-muted-foreground md:table-cell">{{ folder.owner }}</td>
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
                @click.stop="actionFolderId = actionFolderId === folder.id ? null : folder.id"
              >
                <FontAwesomeIcon class="h-4 w-4" :icon="faEllipsis" />
              </button>
              <div
                v-if="actionFolderId === folder.id"
                class="absolute right-3 top-9 z-20 w-44 overflow-hidden rounded-lg border border-border bg-popover py-1 text-left shadow-pop"
                @click.stop
              >
                <button class="flex w-full items-center gap-2.5 px-3 py-1.5 text-[12px] hover:bg-surface-hover" type="button" @click="beginRename(folder)">
                  <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faPen" />
                  Rename
                </button>
                <button class="flex w-full items-center gap-2.5 px-3 py-1.5 text-[12px] hover:bg-surface-hover" type="button">
                  <FontAwesomeIcon class="h-3.5 w-3.5 text-muted-foreground" :icon="faDownload" />
                  Download
                </button>
                <div class="my-1 h-px bg-border" />
                <button class="flex w-full items-center gap-2.5 px-3 py-1.5 text-[12px] text-destructive hover:bg-destructive-soft" type="button" @click="deleteFolder(folder.id)">
                  <FontAwesomeIcon class="h-3.5 w-3.5" :icon="faTrash" />
                  Delete
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="filteredFolders.length === 0" class="flex h-48 items-center justify-center text-[13px] text-muted-foreground">
        No folders match your search.
      </div>
    </div>

    <footer class="flex items-center justify-between border-t border-border bg-surface px-4 py-1.5 text-[11px] text-muted-foreground">
      <span>{{ folders.length }} folders · {{ selectedFolders.length }} selected</span>
      <span class="flex items-center gap-1">
        <FontAwesomeIcon class="h-3 w-3 text-success" :icon="faCheckCircle" />
        Last sync 2 min ago
      </span>
    </footer>
  </div>
</template>
