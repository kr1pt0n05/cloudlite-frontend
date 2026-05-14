<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import FilesFooter from "../components/FilesFooter.vue";
import FilesSelectionBar from "../components/FilesSelectionBar.vue";
import FilesToolbar from "../components/FilesToolbar.vue";
import FoldersTable from "../components/FoldersTable.vue";
import {
  createFilesystemEntry,
  fetchRootFolders,
  getParentDirectory,
  normalizeEntryPath,
  upsertEntry,
  type DirectoryPath,
  type RootFolder,
} from "../services/filesService";

const folders = ref<RootFolder[]>([]);
const currentDirectory = ref<DirectoryPath>(null);
const selected = ref<Set<string>>(new Set());
const actionFolderId = ref<string | null>(null);
const renamingId = ref<string | null>(null);
const renameValue = ref("");
const search = ref("");
const syncing = ref(false);
const dropZone = ref<HTMLElement | null>(null);
const draggingOverDropZone = ref(false);
const directoryCache = new Map<string, RootFolder[]>();
let unlistenDragDrop: UnlistenFn | null = null;
let unlistenEntryCreated: UnlistenFn | null = null;

type FilesystemEntryCreated = {
  path: string;
  isDirectory: boolean;
};

const selectedFolders = computed(() => folders.value.filter((folder) => selected.value.has(folder.id)));
const filteredFolders = computed(() => {
  const query = search.value.trim().toLowerCase();
  if (!query) {
    return folders.value;
  }

  return folders.value.filter((folder) => folder.name.toLowerCase().includes(query));
});

onMounted(async () => {
  folders.value = await fetchRootFolders();
  directoryCache.set(cacheKey(currentDirectory.value), folders.value);
  unlistenEntryCreated = await listen<FilesystemEntryCreated>("filesystem-entry-created", ({ payload }) => {
    handleFilesystemEntryCreated(payload);
  });

  unlistenDragDrop = await getCurrentWebview().onDragDropEvent(({ payload }) => {
    if (payload.type === "leave") {
      draggingOverDropZone.value = false;
      return;
    }

    if (payload.type === "enter" || payload.type === "over") {
      draggingOverDropZone.value = isInsideDropZone(payload.position.x, payload.position.y);
      return;
    }

    if (payload.type === "drop") {
      const shouldHandleDrop = draggingOverDropZone.value || isInsideDropZone(payload.position.x, payload.position.y);
      draggingOverDropZone.value = false;

      if (shouldHandleDrop) {
        handleDroppedPaths(payload.paths);
      }
    }
  });
});

onBeforeUnmount(() => {
  unlistenDragDrop?.();
  unlistenEntryCreated?.();
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

function cacheKey(directory: DirectoryPath) {
  return directory ?? "";
}

function setCurrentDirectory(directory: DirectoryPath) {
  currentDirectory.value = directory;
  folders.value = directoryCache.get(cacheKey(directory)) ?? [];
  replaceSelected(new Set());
  actionFolderId.value = null;
  renamingId.value = null;
  renameValue.value = "";
}

function beginRename(folder: RootFolder) {
  actionFolderId.value = null;
  renamingId.value = folder.id;
  renameValue.value = folder.name;
}

function saveRename() {
  const id = renamingId.value;
  const name = renameValue.value.trim();
  if (!id || !name) {
    renamingId.value = null;
    return;
  }

  folders.value = folders.value.map((folder) => (folder.id === id ? { ...folder, name, modified: "Just now" } : folder));
  directoryCache.set(cacheKey(currentDirectory.value), folders.value);
  renamingId.value = null;
  renameValue.value = "";
}

function deleteFolder(id: string) {
  folders.value = folders.value.filter((folder) => folder.id !== id);
  directoryCache.set(cacheKey(currentDirectory.value), folders.value);

  const next = new Set(selected.value);
  next.delete(id);
  replaceSelected(next);
  actionFolderId.value = null;
}

function deleteSelected() {
  const ids = new Set(selected.value);
  folders.value = folders.value.filter((folder) => !ids.has(folder.id));
  directoryCache.set(cacheKey(currentDirectory.value), folders.value);
  replaceSelected(new Set());
}

function openFolder(folder: RootFolder) {
  if (folder.isDirectory) {
    setCurrentDirectory(folder.path);
  }
}

function navigateRoot() {
  setCurrentDirectory(null);
}

function navigateParent() {
  if (currentDirectory.value === null) {
    return;
  }

  setCurrentDirectory(getParentDirectory(currentDirectory.value));
}

async function syncFiles() {
  if (syncing.value) {
    return;
  }

  syncing.value = true;
  try {
    await invoke<void>("run_sync");
  } catch (error) {
    console.error("Failed to sync files", error);
  } finally {
    syncing.value = false;
  }
}

function isInsideDropZone(x: number, y: number) {
  const zone = dropZone.value;
  if (!zone) {
    return false;
  }

  const rect = zone.getBoundingClientRect();
  const scale = window.devicePixelRatio || 1;
  const cssX = x / scale;
  const cssY = y / scale;

  return cssX >= rect.left && cssX <= rect.right && cssY >= rect.top && cssY <= rect.bottom;
}

async function handleDroppedPaths(paths: string[]) {
  if (paths.length === 0) {
    return;
  }

  try {
    await invoke<void>("receive_dropped_paths", { paths, destinationPath: currentDirectory.value });
  } catch (error) {
    console.error("Failed to process dropped files", error);
  }
}

function handleFilesystemEntryCreated(payload: FilesystemEntryCreated) {
  const path = normalizeEntryPath(payload.path);
  if (!path || getParentDirectory(path) !== currentDirectory.value) {
    return;
  }

  folders.value = upsertEntry(folders.value, createFilesystemEntry(path, payload.isDirectory));
  directoryCache.set(cacheKey(currentDirectory.value), folders.value);
}
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
    <FilesToolbar
      v-model:search="search"
      :current-directory="currentDirectory"
      :syncing="syncing"
      @navigate-parent="navigateParent"
      @navigate-root="navigateRoot"
      @sync="syncFiles"
    />

    <FilesSelectionBar
      :selected-count="selected.size"
      @clear="replaceSelected(new Set())"
      @delete-selected="deleteSelected"
    />

    <div
      ref="dropZone"
      class="relative flex min-h-0 flex-1 flex-col overflow-hidden border-2 border-transparent transition-colors"
      :class="draggingOverDropZone ? 'border-primary bg-primary-soft/50' : ''"
    >
      <FoldersTable
        v-model:action-folder-id="actionFolderId"
        v-model:renaming-id="renamingId"
        v-model:rename-value="renameValue"
        :folders="filteredFolders"
        :selected="selected"
        @begin-rename="beginRename"
        @delete-folder="deleteFolder"
        @open-folder="openFolder"
        @save-rename="saveRename"
        @toggle-all="toggleAll"
        @toggle-folder="toggleFolder"
      />
    </div>

    <FilesFooter :folders-count="folders.length" :selected-count="selectedFolders.length" />
  </div>
</template>
