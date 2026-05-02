<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
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
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
    <FilesToolbar v-model:search="search" @create-folder="createFolder" />

    <FilesSelectionBar
      :selected-count="selected.size"
      @clear="replaceSelected(new Set())"
      @delete-selected="deleteSelected"
    />

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

    <FilesFooter :folders-count="folders.length" :selected-count="selectedFolders.length" />
  </div>
</template>
