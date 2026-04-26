import { createFileRoute } from "@tanstack/react-router";
import { AppShell } from "@/components/cloudlite/AppShell";
import { Badge } from "@/components/cloudlite/Badge";
import {
  ChevronRight,
  Folder,
  FileText,
  Image as ImageIcon,
  FileArchive,
  FileCode,
  FileVideo,
  FileSpreadsheet,
  Upload,
  FolderPlus,
  Filter,
  Search,
  MoreHorizontal,
  CheckCircle2,
  CloudOff,
  RefreshCw,
  Star,
  Share2,
  Download,
  Trash2,
  Eye,
  Pencil,
} from "lucide-react";
import { useState } from "react";
import { cn } from "@/lib/utils";

export const Route = createFileRoute("/files")({
  component: FilesPage,
  head: () => ({ meta: [{ title: "My Files — CloudLite" }] }),
});

type SyncState = "synced" | "syncing" | "pending" | "error" | "offline";

type Row = {
  id: string;
  name: string;
  type: "folder" | "doc" | "image" | "archive" | "code" | "video" | "sheet";
  size: string;
  modified: string;
  modifiedBy?: string;
  sync: SyncState;
  shared?: boolean;
  starred?: boolean;
};

const rows: Row[] = [
  { id: "1", name: "Design assets", type: "folder", size: "—", modified: "2h ago", sync: "synced", shared: true },
  { id: "2", name: "Engineering", type: "folder", size: "—", modified: "Yesterday", sync: "synced" },
  { id: "3", name: "Q4 Roadmap.pdf", type: "doc", size: "2.4 MB", modified: "10 min ago", modifiedBy: "Jamie", sync: "syncing", starred: true },
  { id: "4", name: "release-notes.md", type: "code", size: "12 KB", modified: "1h ago", modifiedBy: "Pat", sync: "synced" },
  { id: "5", name: "hero-banner-final.png", type: "image", size: "8.1 MB", modified: "Today, 09:14", sync: "synced", shared: true },
  { id: "6", name: "customers-export.csv", type: "sheet", size: "412 KB", modified: "3 days ago", sync: "pending" },
  { id: "7", name: "demo-recording.mp4", type: "video", size: "1.2 GB", modified: "Apr 18", sync: "error" },
  { id: "8", name: "backups-2025-04.zip", type: "archive", size: "642 MB", modified: "Apr 15", sync: "offline" },
  { id: "9", name: "infra-notes.txt", type: "doc", size: "3 KB", modified: "Apr 12", sync: "synced" },
  { id: "10", name: "logo-variants", type: "folder", size: "—", modified: "Apr 10", sync: "synced", starred: true },
];

const typeIcon: Record<Row["type"], { Icon: typeof Folder; color: string }> = {
  folder: { Icon: Folder, color: "text-primary" },
  doc: { Icon: FileText, color: "text-info" },
  image: { Icon: ImageIcon, color: "text-success" },
  archive: { Icon: FileArchive, color: "text-warning" },
  code: { Icon: FileCode, color: "text-foreground/70" },
  video: { Icon: FileVideo, color: "text-destructive" },
  sheet: { Icon: FileSpreadsheet, color: "text-success" },
};

function SyncBadge({ state }: { state: SyncState }) {
  switch (state) {
    case "synced":
      return <Badge tone="success"><CheckCircle2 className="h-3 w-3" /> Synced</Badge>;
    case "syncing":
      return <Badge tone="info"><RefreshCw className="h-3 w-3 animate-spin" /> Syncing</Badge>;
    case "pending":
      return <Badge tone="warning">Pending</Badge>;
    case "error":
      return <Badge tone="danger">Failed</Badge>;
    case "offline":
      return <Badge tone="neutral"><CloudOff className="h-3 w-3" /> Offline</Badge>;
  }
}

function FilesPage() {
  const [selected, setSelected] = useState<Set<string>>(new Set(["3", "5"]));
  const [contextRow, setContextRow] = useState<string | null>(null);

  const toggle = (id: string) => {
    setSelected((s) => {
      const n = new Set(s);
      n.has(id) ? n.delete(id) : n.add(id);
      return n;
    });
  };

  return (
    <AppShell>
      {/* Toolbar */}
      <div className="flex items-center gap-2 border-b border-border bg-surface px-4 py-2.5">
        <nav className="flex items-center gap-1 text-[13px]">
          <button className="rounded px-1.5 py-0.5 font-medium text-foreground hover:bg-surface-hover">My Files</button>
          <ChevronRight className="h-3.5 w-3.5 text-muted-foreground" />
          <button className="rounded px-1.5 py-0.5 text-muted-foreground hover:bg-surface-hover hover:text-foreground">Projects</button>
          <ChevronRight className="h-3.5 w-3.5 text-muted-foreground" />
          <span className="rounded px-1.5 py-0.5 font-medium text-foreground">Acme rebrand</span>
        </nav>

        <div className="ml-4 flex h-8 flex-1 items-center gap-2 rounded-md border border-input bg-surface-2 px-2.5 text-[12px]">
          <Search className="h-3.5 w-3.5 text-muted-foreground" />
          <input className="h-full flex-1 bg-transparent outline-none placeholder:text-muted-foreground" placeholder="Search in this folder…" />
        </div>

        <button className="inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-surface px-2.5 text-[12px] font-medium text-foreground hover:bg-surface-hover">
          <Filter className="h-3.5 w-3.5" /> Filter
        </button>
        <button className="inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-surface px-2.5 text-[12px] font-medium text-foreground hover:bg-surface-hover">
          <FolderPlus className="h-3.5 w-3.5" /> New folder
        </button>
        <button className="inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-[12px] font-medium text-primary-foreground shadow-sm hover:bg-primary/90">
          <Upload className="h-3.5 w-3.5" /> Upload
        </button>
      </div>

      {/* Selection bar */}
      {selected.size > 0 && (
        <div className="flex items-center gap-3 border-b border-border bg-primary-soft px-4 py-2 text-[12px] text-primary">
          <span className="font-medium">{selected.size} selected</span>
          <button className="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10"><Download className="h-3.5 w-3.5" /> Download</button>
          <button className="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10"><Share2 className="h-3.5 w-3.5" /> Share</button>
          <button className="inline-flex items-center gap-1 rounded px-1.5 py-0.5 hover:bg-primary/10"><Trash2 className="h-3.5 w-3.5" /> Delete</button>
          <button onClick={() => setSelected(new Set())} className="ml-auto text-[11px] hover:underline">Clear</button>
        </div>
      )}

      {/* Table */}
      <div className="relative flex-1 overflow-auto">
        <table className="w-full border-separate border-spacing-0 text-[13px]">
          <thead className="sticky top-0 z-10 bg-surface-2 text-[11px] uppercase tracking-wider text-muted-foreground">
            <tr>
              <th className="w-8 border-b border-border px-3 py-2 text-left">
                <input type="checkbox" className="h-3.5 w-3.5 rounded border-border accent-primary" />
              </th>
              <th className="border-b border-border px-3 py-2 text-left font-semibold">Name</th>
              <th className="w-[110px] border-b border-border px-3 py-2 text-right font-semibold">Size</th>
              <th className="w-[170px] border-b border-border px-3 py-2 text-left font-semibold">Modified</th>
              <th className="w-[130px] border-b border-border px-3 py-2 text-left font-semibold">Sync</th>
              <th className="w-12 border-b border-border" />
            </tr>
          </thead>
          <tbody>
            {rows.map((r) => {
              const { Icon, color } = typeIcon[r.type];
              const isSel = selected.has(r.id);
              const isCtx = contextRow === r.id;
              return (
                <tr
                  key={r.id}
                  onClick={() => toggle(r.id)}
                  onContextMenu={(e) => {
                    e.preventDefault();
                    setContextRow(isCtx ? null : r.id);
                  }}
                  className={cn(
                    "group cursor-default border-b border-border/60",
                    isSel ? "bg-primary-soft/60" : "hover:bg-surface-hover",
                  )}
                >
                  <td className="px-3 py-2">
                    <input
                      type="checkbox"
                      checked={isSel}
                      onChange={() => toggle(r.id)}
                      onClick={(e) => e.stopPropagation()}
                      className="h-3.5 w-3.5 rounded border-border accent-primary"
                    />
                  </td>
                  <td className="px-3 py-2">
                    <div className="flex items-center gap-2.5">
                      <Icon className={cn("h-4 w-4 shrink-0", color)} />
                      <span className="truncate font-medium text-foreground">{r.name}</span>
                      {r.starred && <Star className="h-3.5 w-3.5 fill-warning text-warning" />}
                      {r.shared && <Share2 className="h-3.5 w-3.5 text-muted-foreground" />}
                    </div>
                  </td>
                  <td className="px-3 py-2 text-right tabular-nums text-muted-foreground">{r.size}</td>
                  <td className="px-3 py-2 text-muted-foreground">
                    {r.modified}
                    {r.modifiedBy && <span className="text-foreground/60"> · {r.modifiedBy}</span>}
                  </td>
                  <td className="px-3 py-2"><SyncBadge state={r.sync} /></td>
                  <td className="px-3 py-2 text-right">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        setContextRow(isCtx ? null : r.id);
                      }}
                      className="rounded p-1 text-muted-foreground opacity-0 hover:bg-surface-hover hover:text-foreground group-hover:opacity-100"
                    >
                      <MoreHorizontal className="h-4 w-4" />
                    </button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>

        {/* Context menu */}
        {contextRow && (
          <div className="absolute right-12 top-32 z-20 w-52 overflow-hidden rounded-lg border border-border bg-popover py-1 shadow-pop">
            {[
              { Icon: Eye, label: "Open" },
              { Icon: Download, label: "Download" },
              { Icon: Share2, label: "Share…" },
              { Icon: Pencil, label: "Rename" },
              { Icon: Star, label: "Add to starred" },
            ].map((it) => (
              <button
                key={it.label}
                className="flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-[12px] text-popover-foreground hover:bg-surface-hover"
              >
                <it.Icon className="h-3.5 w-3.5 text-muted-foreground" />
                {it.label}
              </button>
            ))}
            <div className="my-1 h-px bg-border" />
            <button className="flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-[12px] text-destructive hover:bg-destructive-soft">
              <Trash2 className="h-3.5 w-3.5" /> Move to trash
            </button>
          </div>
        )}
      </div>

      {/* Status bar */}
      <div className="flex items-center justify-between border-t border-border bg-surface px-4 py-1.5 text-[11px] text-muted-foreground">
        <span>{rows.length} items · {selected.size} selected</span>
        <div className="flex items-center gap-3">
          <span className="flex items-center gap-1"><CheckCircle2 className="h-3 w-3 text-success" /> All changes saved</span>
          <span>Last sync 2 min ago</span>
        </div>
      </div>
    </AppShell>
  );
}
