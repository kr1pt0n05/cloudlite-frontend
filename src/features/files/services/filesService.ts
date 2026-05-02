export type SyncState = "synced" | "syncing" | "pending";

export type RootFolder = {
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

let rootFolders: RootFolder[] = [
  { id: "1", name: "Design assets", owner: "Jamie", folders: 12, files: 84, size: "18.4 GB", modified: "Today, 09:14", sync: "synced", shared: true },
  { id: "2", name: "Engineering", owner: "Pat", folders: 18, files: 211, size: "42.7 GB", modified: "Yesterday", sync: "synced" },
  { id: "3", name: "Product strategy", owner: "Jamie", folders: 6, files: 36, size: "4.2 GB", modified: "2 days ago", sync: "syncing", starred: true },
  { id: "4", name: "Client contracts", owner: "Morgan", folders: 4, files: 128, size: "9.8 GB", modified: "Apr 22", sync: "pending", shared: true },
  { id: "5", name: "Invoices", owner: "Finance", folders: 9, files: 302, size: "6.1 GB", modified: "Apr 18", sync: "synced" },
  { id: "6", name: "Archive", owner: "Admin", folders: 31, files: 470, size: "96.3 GB", modified: "Apr 10", sync: "synced" },
];

function cloneFolder(folder: RootFolder): RootFolder {
  return { ...folder };
}

function cloneFolders(folders: RootFolder[]): RootFolder[] {
  return folders.map(cloneFolder);
}

export async function fetchRootFolders(): Promise<RootFolder[]> {
  return cloneFolders(rootFolders);
}

export async function createRootFolder(): Promise<RootFolder> {
  const nextNumber = rootFolders.length + 1;
  const folder: RootFolder = {
    id: String(Date.now()),
    name: `Untitled folder ${nextNumber}`,
    owner: "Jamie",
    folders: 0,
    files: 0,
    size: "0 KB",
    modified: "Just now",
    sync: "pending",
  };

  rootFolders = [folder, ...rootFolders];
  return cloneFolder(folder);
}

export async function renameRootFolder(id: string, name: string): Promise<RootFolder | null> {
  const folder = rootFolders.find((item) => item.id === id);
  if (!folder) {
    return null;
  }

  folder.name = name;
  folder.modified = "Just now";
  return cloneFolder(folder);
}

export async function deleteRootFolder(id: string): Promise<void> {
  rootFolders = rootFolders.filter((folder) => folder.id !== id);
}

export async function deleteRootFolders(ids: Set<string>): Promise<void> {
  rootFolders = rootFolders.filter((folder) => !ids.has(folder.id));
}
