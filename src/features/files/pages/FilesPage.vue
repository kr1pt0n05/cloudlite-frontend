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
  fetchDirectoryEntries,
  fetchRootFolders,
  getParentDirectory,
  normalizeEntryPath,
  renameFile,
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
const loadingDirectory = ref(false);
const dropZone = ref<HTMLElement | null>(null);
const draggingOverDropZone = ref(false);
const directoryCache = new Map<string, RootFolder[]>();
const directoryIdByPath = new Map<string, string>();
const navigationStack = ref<DirectoryLocation[]>([{ path: null, directoryId: null }]);
let unlistenDragDrop: UnlistenFn | null = null;
let unlistenEntryCreated: UnlistenFn | null = null;

type DirectoryLocation = {
  path: DirectoryPath;
  directoryId: string | null;
};

type FilesystemEntryCreated = {
  id: string;
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
  rememberDirectoryIds(folders.value);
  directoryCache.set(cacheKey(navigationStack.value[0]), folders.value);
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

function cacheKey(location: DirectoryLocation) {
  return location.directoryId ?? location.path ?? "";
}

function currentLocation(): DirectoryLocation {
  return navigationStack.value[navigationStack.value.length - 1] ?? { path: null, directoryId: null };
}

function setCurrentDirectory(location: DirectoryLocation) {
  currentDirectory.value = location.path;
  folders.value = directoryCache.get(cacheKey(location)) ?? [];
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

async function saveRename() {
  const id = renamingId.value;
  const name = renameValue.value.trim();
  if (!id || !name) {
    renamingId.value = null;
    return;
  }

  const entry = folders.value.find((folder) => folder.id === id);
  if (!entry || entry.name === name) {
    renamingId.value = null;
    renameValue.value = "";
    return;
  }

  try {
    const renamedName = entry.isDirectory ? name : await renameFile(id, name);
    updateRenamedEntry(id, renamedName);
  } catch (error) {
    console.error("Failed to rename file", error);
  } finally {
    renamingId.value = null;
    renameValue.value = "";
  }
}

function updateRenamedEntry(id: string, name: string) {
  folders.value = folders.value.map((folder) => renameEntry(folder, id, name));
  directoryCache.set(cacheKey(currentLocation()), folders.value);
}

function renameEntry(entry: RootFolder, id: string, name: string): RootFolder {
  if (entry.id !== id) {
    return entry;
  }

  const parentPath = getParentDirectory(entry.path);
  const path = normalizeEntryPath([parentPath, name].filter(Boolean).join("/"));

  return { ...entry, name, path, modified: "Just now" };
}

function deleteFolder(id: string) {
  folders.value = folders.value.filter((folder) => folder.id !== id);
  directoryCache.set(cacheKey(currentLocation()), folders.value);

  const next = new Set(selected.value);
  next.delete(id);
  replaceSelected(next);
  actionFolderId.value = null;
}

function deleteSelected() {
  const ids = new Set(selected.value);
  folders.value = folders.value.filter((folder) => !ids.has(folder.id));
  directoryCache.set(cacheKey(currentLocation()), folders.value);
  replaceSelected(new Set());
}

async function openFolder(folder: RootFolder) {
  if (!folder.isDirectory || loadingDirectory.value) {
    return;
  }

  loadingDirectory.value = true;
  const parentLocation = currentLocation();
  const location = {
    path: folder.path,
    directoryId: folder.directoryId ?? directoryIdByPath.get(folder.path) ?? null,
  };

  navigationStack.value = [...navigationStack.value, location];
  setCurrentDirectory(location);
  await loadDirectory(location, folder, parentLocation);
}

async function navigateRoot() {
  if (loadingDirectory.value) {
    return;
  }

  loadingDirectory.value = true;
  navigationStack.value = [{ path: null, directoryId: null }];
  setCurrentDirectory(currentLocation());
  await loadDirectory(currentLocation());
}

async function navigateParent() {
  if (navigationStack.value.length <= 1 || loadingDirectory.value) {
    return;
  }

  loadingDirectory.value = true;
  navigationStack.value = navigationStack.value.slice(0, -1);
  setCurrentDirectory(currentLocation());
  await loadDirectory(currentLocation());
}

async function navigatePath(path: string) {
  if (loadingDirectory.value) {
    return;
  }

  const normalizedPath = normalizeEntryPath(path);
  const stackIndex = navigationStack.value.findIndex((location) => location.path === normalizedPath);

  if (stackIndex !== -1) {
    loadingDirectory.value = true;
    navigationStack.value = navigationStack.value.slice(0, stackIndex + 1);
    navigationStack.value[stackIndex] = {
      path: normalizedPath,
      directoryId: currentLocation().directoryId ?? directoryIdByPath.get(normalizedPath) ?? null,
    };
    setCurrentDirectory(currentLocation());
    await loadDirectory(currentLocation());

    return;
  }

  const location = { path: normalizedPath, directoryId: directoryIdByPath.get(normalizedPath) ?? null };

  loadingDirectory.value = true;
  navigationStack.value = buildNavigationStack(normalizedPath, location.directoryId);
  setCurrentDirectory(location);
  await loadDirectory(location);
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

  folders.value = upsertEntry(folders.value, createFilesystemEntry(payload.id, path, payload.isDirectory));
  directoryCache.set(cacheKey(currentLocation()), folders.value);
  rememberDirectoryIds(folders.value);
}

async function loadDirectory(location: DirectoryLocation, sourceFolder?: RootFolder, parentLocation?: DirectoryLocation) {
  loadingDirectory.value = true;

  try {
    const directoryId =
      location.directoryId ??
      (sourceFolder ? await resolveDirectoryId(sourceFolder, parentLocation ?? currentLocation()) : location.path ? await resolveDirectoryIdByPath(location.path) : null);

    if (directoryId) {
      location.directoryId = directoryId;
      directoryIdByPath.set(location.path ?? "", directoryId);
      navigationStack.value = navigationStack.value.map((item, index) =>
        index === navigationStack.value.length - 1 ? { ...item, directoryId } : item,
      );
    }

    if (!directoryId && location.path !== null) {
      directoryCache.set(cacheKey(location), []);

      if (currentDirectory.value === location.path) {
        folders.value = [];
      }

      return;
    }

    const entries = await fetchDirectoryEntries(directoryId, location.path);
    rememberDirectoryIds(entries);
    directoryCache.set(cacheKey(location), entries);

    if (currentDirectory.value === location.path) {
      folders.value = entries;
    }
  } catch (error) {
    console.error("Failed to load directory entries", error);
  } finally {
    loadingDirectory.value = false;
  }
}

async function resolveDirectoryId(folder: RootFolder, parentLocation: DirectoryLocation) {
  const entries = await fetchDirectoryEntries(parentLocation.directoryId, parentLocation.path);
  rememberDirectoryIds(entries);
  directoryCache.set(cacheKey(parentLocation), entries);

  const hydratedFolder = entries.find((entry) => entry.isDirectory && entry.path === folder.path);

  if (hydratedFolder?.directoryId) {
    return hydratedFolder.directoryId;
  }

  return null;
}

async function resolveDirectoryIdByPath(path: string) {
  const knownDirectoryId = directoryIdByPath.get(path);

  if (knownDirectoryId) {
    return knownDirectoryId;
  }

  const segments = path.split("/").filter(Boolean);
  let parentLocation: DirectoryLocation = { path: null, directoryId: null };
  let directoryId: string | null = null;

  for (let index = 0; index < segments.length; index += 1) {
    const segmentPath = segments.slice(0, index + 1).join("/");
    const cachedDirectoryId = directoryIdByPath.get(segmentPath);

    if (cachedDirectoryId) {
      directoryId = cachedDirectoryId;
      parentLocation = { path: segmentPath, directoryId };
      continue;
    }

    const entries = await fetchDirectoryEntries(parentLocation.directoryId, parentLocation.path);
    rememberDirectoryIds(entries);
    directoryCache.set(cacheKey(parentLocation), entries);

    const directory = entries.find((entry) => entry.isDirectory && entry.path === segmentPath);

    if (!directory?.directoryId) {
      return null;
    }

    directoryId = directory.directoryId;
    parentLocation = { path: segmentPath, directoryId };
  }

  return directoryId;
}

function buildNavigationStack(path: string, directoryId: string | null): DirectoryLocation[] {
  const segments = path.split("/").filter(Boolean);
  const locations = segments.map((_, index) => {
    const segmentPath = segments.slice(0, index + 1).join("/");

    return {
      path: segmentPath,
      directoryId: segmentPath === path ? directoryId : directoryIdByPath.get(segmentPath) ?? null,
    };
  });

  return [{ path: null, directoryId: null }, ...locations];
}

function rememberDirectoryIds(entries: RootFolder[]) {
  for (const entry of entries) {
    if (entry.isDirectory && entry.directoryId) {
      directoryIdByPath.set(entry.path, entry.directoryId);
    }
  }
}
</script>

<template>
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
    <FilesToolbar
      v-model:search="search"
      :current-directory="currentDirectory"
      :syncing="syncing"
      @navigate-parent="navigateParent"
      @navigate-path="navigatePath"
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
        :hydrating="loadingDirectory"
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
