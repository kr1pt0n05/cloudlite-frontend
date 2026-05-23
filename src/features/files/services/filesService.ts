import { invoke } from "@tauri-apps/api/core";

export type SyncState = "synced" | "syncing" | "pending";

export type RootFolder = {
  id: string;
  directoryId: string | null;
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

type LocalFsDirectory = {
  id: string;
  name: string;
  parent: string | null;
  createdAt: string;
  updatedAt: string | null;
};

type LocalFsFile = {
  id: string;
  name: string;
  directory: string | null;
  checksum: string | null;
  size: number;
  mimeType: string;
  createdAt: string;
  updatedAt: string | null;
};

type DirectoryEntries = {
  directories: LocalFsDirectory[];
  files: LocalFsFile[];
};

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

export function createFilesystemEntry(id: string, path: string, isDirectory: boolean): RootFolder {
  const normalized = normalizeEntryPath(path);
  const name = getEntryName(normalized);

  return {
    id,
    directoryId: isDirectory ? id : null,
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

export async function fetchDirectoryEntries(directoryId: string | null, parentPath: DirectoryPath): Promise<RootFolder[]> {
  const entries = await invoke<DirectoryEntries>("get_directory_entries", { directoryId });
  return [
    ...entries.directories.map((directory) => createDirectoryEntry(directory, parentPath)),
    ...entries.files.map((file) => createFileEntry(file, parentPath)),
  ];
}

export function createDirectoryEntry(directory: LocalFsDirectory, parentPath: DirectoryPath): RootFolder {
  const normalized = normalizeEntryPath([parentPath, directory.name].filter(Boolean).join("/"));

  return {
    id: directory.id,
    directoryId: directory.id,
    path: normalized,
    name: directory.name,
    isDirectory: true,
    folders: 0,
    files: 0,
    size: "-",
    modified: formatModified(directory.updatedAt ?? directory.createdAt),
    sync: "synced",
  };
}

export function createFileEntry(file: LocalFsFile, parentPath: DirectoryPath): RootFolder {
  const path = normalizeEntryPath([parentPath, file.name].filter(Boolean).join("/"));

  return {
    id: file.id,
    directoryId: null,
    path,
    name: file.name,
    isDirectory: false,
    folders: 0,
    files: 1,
    size: formatFileSize(file.size),
    modified: formatModified(file.updatedAt ?? file.createdAt),
    sync: "synced",
  };
}

export async function fetchRootFolders(): Promise<RootFolder[]> {
  return fetchDirectoryEntries(null, null);
}

export function renameFile(fileId: string, filename: string): Promise<string> {
  return invoke<string>("rename_file", { fileId, filename });
}

export function renameDirectory(directoryId: string, name: string): Promise<string> {
  return invoke<string>("rename_directory", { directoryId, name });
}

export function deleteFile(fileId: string): Promise<void> {
  return invoke<void>("delete_file", { fileId });
}

export function deleteDirectory(directoryId: string): Promise<void> {
  return invoke<void>("delete_directory", { directoryId });
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

function formatModified(value: string): string {
  const timestamp = Date.parse(value);

  if (Number.isNaN(timestamp)) {
    return value;
  }

  return new Intl.DateTimeFormat(undefined, {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(timestamp));
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) {
    return "0 B";
  }

  const units = ["B", "KB", "MB", "GB", "TB"];
  const exponent = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  const value = bytes / 1024 ** exponent;

  return `${value >= 10 || exponent === 0 ? value.toFixed(0) : value.toFixed(1)} ${units[exponent]}`;
}
