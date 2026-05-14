import { invoke } from "@tauri-apps/api/core";

export type SyncState = "synced" | "syncing" | "pending";

export type RootFolder = {
  id: string;
  path: string;
  name: string;
  isDirectory: boolean;
  folders: number;
  files: number;
  size: string;
  modified: string;
  sync: SyncState;
};

export type DirectoryPath = string | null;

export function normalizeEntryPath(path: string): string {
  return path.replace(/\\/g, "/").replace(/^\/+|\/+$/g, "");
}

export function getParentDirectory(path: string): DirectoryPath {
  const normalized = normalizeEntryPath(path);
  const index = normalized.lastIndexOf("/");

  if (index === -1) {
    return null;
  }

  return normalized.slice(0, index);
}

export function getEntryName(path: string): string {
  const normalized = normalizeEntryPath(path);
  const segments = normalized.split("/");
  return segments[segments.length - 1] ?? normalized;
}

export function createFilesystemEntry(path: string, isDirectory: boolean): RootFolder {
  const normalized = normalizeEntryPath(path);
  const name = getEntryName(normalized);

  return {
    id: normalized,
    path: normalized,
    name,
    isDirectory,
    folders: 0,
    files: isDirectory ? 0 : 1,
    size: "-",
    modified: "Just now",
    sync: "pending",
  };
}

export async function fetchRootFolders(): Promise<RootFolder[]> {
  return [];
}

export function upsertEntry(entries: RootFolder[], entry: RootFolder): RootFolder[] {
  if (entries.some((item) => item.path === entry.path)) {
    return entries;
  }

  return [entry, ...entries];
}

export function getChangelogs(): Promise<void> {
  return invoke<void>("get_change_logs");
}
