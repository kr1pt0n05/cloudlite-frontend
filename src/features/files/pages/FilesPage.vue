<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import FilesFooter from "../components/FilesFooter.vue";
import FilesSelectionBar from "../components/FilesSelectionBar.vue";
import FilesToolbar from "../components/FilesToolbar.vue";
import FoldersTable from "../components/FoldersTable.vue";
import {
  createRootFolder,
  deleteRootFolder,
  deleteRootFolders,
  fetchRootFolders,
  renameRootFolder,
  type RootFolder,
} from "../services/filesService";

const folders = ref<RootFolder[]>([]);
const selected = ref<Set<string>>(new Set());
const actionFolderId = ref<string | null>(null);
const renamingId = ref<string | null>(null);
const renameValue = ref("");
const search = ref("");
const syncing = ref(false);
const dropZone = ref<HTMLElement | null>(null);
const draggingOverDropZone = ref(false);
let unlistenDragDrop: UnlistenFn | null = null;

type DroppedFsEntry = {
  path: string;
  relativePath: string;
  isDirectory: boolean;
  size?: number;
};

const selectedFolders = computed(() => folders.value.filter((folder) => selected.value.has(folder.id)));
const filteredFolders = computed(() => {
  const query = search.value.trim().toLowerCase();
  if (!query) {
    return folders.value;
  }

  return folders.value.filter((folder) => folder.name.toLowerCase().includes(query) || folder.owner.toLowerCase().includes(query));
});

onMounted(async () => {
  folders.value = await fetchRootFolders();
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

async function saveRename() {
  const id = renamingId.value;
  const name = renameValue.value.trim();
  if (!id || !name) {
    renamingId.value = null;
    return;
  }

  const renamedFolder = await renameRootFolder(id, name);
  if (renamedFolder) {
    folders.value = folders.value.map((folder) => (folder.id === id ? renamedFolder : folder));
  }

  renamingId.value = null;
  renameValue.value = "";
}

async function deleteFolder(id: string) {
  await deleteRootFolder(id);
  folders.value = folders.value.filter((folder) => folder.id !== id);

  const next = new Set(selected.value);
  next.delete(id);
  replaceSelected(next);
  actionFolderId.value = null;
}

async function deleteSelected() {
  const ids = new Set(selected.value);
  await deleteRootFolders(ids);
  folders.value = folders.value.filter((folder) => !ids.has(folder.id));
  replaceSelected(new Set());
}

async function createFolder() {
  const folder = await createRootFolder();
  folders.value = [folder, ...folders.value];
}

async function syncFiles() {
  if (syncing.value) {
    console.log("synching value", syncing.value);
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
  console.log(paths);

  try {
    const entries = await invoke<DroppedFsEntry[]>("receive_dropped_paths", { paths });
    console.log("Dropped filesystem entries", entries);
  } catch (error) {
    console.error("Failed to process dropped files", error);
  }
}
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
    <FilesToolbar v-model:search="search" :syncing="syncing" @create-folder="createFolder" @sync="syncFiles" />

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
        @save-rename="saveRename"
        @toggle-all="toggleAll"
        @toggle-folder="toggleFolder"
      />
    </div>

    <FilesFooter :folders-count="folders.length" :selected-count="selectedFolders.length" />
  </div>
</template>
