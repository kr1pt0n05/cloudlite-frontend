import { createFileRoute } from "@tanstack/react-router";
import { AppShell } from "@/components/cloudlite/AppShell";
import { Badge } from "@/components/cloudlite/Badge";
import { ProgressBar } from "@/components/cloudlite/ProgressBar";
import {
  Pause,
  Play,
  X,
  RotateCw,
  CheckCircle2,
  AlertCircle,
  FileText,
  Image as ImageIcon,
  FileVideo,
  FileArchive,
  FileCode,
  Trash2,
} from "lucide-react";
import { cn } from "@/lib/utils";

export const Route = createFileRoute("/uploads")({
  component: UploadsPage,
  head: () => ({ meta: [{ title: "Uploads — CloudLite" }] }),
});

type Status = "uploading" | "staged" | "finalizing" | "done" | "failed" | "paused";

type Item = {
  id: string;
  name: string;
  path: string;
  size: string;
  type: "doc" | "image" | "video" | "archive" | "code";
  status: Status;
  progress: number;
  speed?: string;
  eta?: string;
  error?: string;
};

const typeIcon = { doc: FileText, image: ImageIcon, video: FileVideo, archive: FileArchive, code: FileCode };

const items: Item[] = [
  { id: "u1", name: "shoot-2025-04-26-001.cr3", path: "/Photos/Apr 26", size: "48.2 MB", type: "image", status: "uploading", progress: 72, speed: "12.4 MB/s", eta: "1s" },
  { id: "u2", name: "shoot-2025-04-26-002.cr3", path: "/Photos/Apr 26", size: "47.9 MB", type: "image", status: "uploading", progress: 34, speed: "11.8 MB/s", eta: "3s" },
  { id: "u3", name: "edit-master-v4.mp4", path: "/Video", size: "2.1 GB", type: "video", status: "uploading", progress: 18, speed: "42.1 MB/s", eta: "42s" },
  { id: "u4", name: "release-notes.md", path: "/Docs", size: "12 KB", type: "code", status: "finalizing", progress: 100 },
  { id: "u5", name: "design-tokens.json", path: "/Design", size: "4 KB", type: "code", status: "done", progress: 100 },
  { id: "u6", name: "hero-banner-final.png", path: "/Design", size: "8.1 MB", type: "image", status: "done", progress: 100 },
  { id: "u7", name: "Q4 deck.pdf", path: "/Docs", size: "14 MB", type: "doc", status: "paused", progress: 56 },
  { id: "u8", name: "backup-archive.tar.gz", path: "/Backups", size: "642 MB", type: "archive", status: "failed", progress: 87, error: "Network reset by server" },
  { id: "u9", name: "raw-batch-001.zip", path: "/Photos", size: "1.4 GB", type: "archive", status: "staged", progress: 0 },
  { id: "u10", name: "raw-batch-002.zip", path: "/Photos", size: "1.6 GB", type: "archive", status: "staged", progress: 0 },
];

function StatusBadge({ s }: { s: Status }) {
  switch (s) {
    case "uploading": return <Badge tone="info">Uploading</Badge>;
    case "staged": return <Badge tone="neutral">Staged</Badge>;
    case "finalizing": return <Badge tone="primary">Finalizing</Badge>;
    case "done": return <Badge tone="success"><CheckCircle2 className="h-3 w-3" /> Done</Badge>;
    case "failed": return <Badge tone="danger"><AlertCircle className="h-3 w-3" /> Failed</Badge>;
    case "paused": return <Badge tone="warning">Paused</Badge>;
  }
}

function UploadsPage() {
  const active = items.filter((i) => i.status === "uploading");
  const overall = Math.round(items.reduce((a, b) => a + b.progress, 0) / items.length);

  return (
    <AppShell>
      {/* Header */}
      <div className="border-b border-border bg-surface px-5 py-4">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-[16px] font-semibold tracking-tight text-foreground">Upload session</h1>
            <p className="mt-0.5 text-[12px] text-muted-foreground">
              Started 4 min ago · Batch ID <span className="font-mono">b_8af2c1</span> · {items.length} files · 6.1 GB total
            </p>
          </div>
          <div className="flex items-center gap-2">
            <button className="inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-surface px-2.5 text-[12px] font-medium text-foreground hover:bg-surface-hover">
              <Pause className="h-3.5 w-3.5" /> Pause all
            </button>
            <button className="inline-flex h-8 items-center gap-1.5 rounded-md border border-destructive/30 bg-destructive-soft px-2.5 text-[12px] font-medium text-destructive hover:bg-destructive/10">
              <X className="h-3.5 w-3.5" /> Cancel session
            </button>
          </div>
        </div>

        {/* Overall progress */}
        <div className="mt-4 grid grid-cols-4 gap-3">
          {[
            { label: "Overall", value: `${overall}%`, hint: "3.7 GB of 6.1 GB" },
            { label: "Active", value: `${active.length}`, hint: "uploading now" },
            { label: "Throughput", value: "66.3", hint: "MB/s combined" },
            { label: "ETA", value: "1m 12s", hint: "until completion" },
          ].map((s) => (
            <div key={s.label} className="rounded-lg border border-border bg-surface-2 p-3">
              <div className="text-[11px] uppercase tracking-wider text-muted-foreground">{s.label}</div>
              <div className="mt-1 text-[18px] font-semibold tabular-nums text-foreground">{s.value}</div>
              <div className="text-[11px] text-muted-foreground">{s.hint}</div>
            </div>
          ))}
        </div>
        <div className="mt-3">
          <ProgressBar value={overall} />
        </div>
      </div>

      {/* Queue */}
      <div className="flex-1 overflow-auto">
        <div className="divide-y divide-border">
          {items.map((it) => {
            const Icon = typeIcon[it.type];
            const isErr = it.status === "failed";
            return (
              <div
                key={it.id}
                className={cn(
                  "row-hover flex items-center gap-3 px-5 py-2.5",
                  isErr && "bg-destructive-soft/30",
                )}
              >
                <div className={cn(
                  "flex h-8 w-8 items-center justify-center rounded-md ring-1",
                  isErr ? "bg-destructive-soft ring-destructive/20" : "bg-surface-2 ring-border",
                )}>
                  <Icon className={cn("h-4 w-4", isErr ? "text-destructive" : "text-muted-foreground")} />
                </div>

                <div className="min-w-0 flex-1">
                  <div className="flex items-center gap-2">
                    <span className="truncate text-[13px] font-medium text-foreground">{it.name}</span>
                    <StatusBadge s={it.status} />
                  </div>
                  <div className="mt-0.5 flex items-center gap-2 text-[11px] text-muted-foreground">
                    <span className="truncate">{it.path}</span>
                    <span>·</span>
                    <span className="tabular-nums">{it.size}</span>
                    {it.speed && <><span>·</span><span className="tabular-nums">{it.speed}</span></>}
                    {it.eta && <><span>·</span><span>ETA {it.eta}</span></>}
                    {it.error && <><span>·</span><span className="text-destructive">{it.error}</span></>}
                  </div>
                  {(it.status === "uploading" || it.status === "paused" || it.status === "failed" || it.status === "finalizing") && (
                    <div className="mt-1.5">
                      <ProgressBar
                        value={it.progress}
                        tone={isErr ? "danger" : it.status === "paused" ? "warning" : "primary"}
                      />
                    </div>
                  )}
                </div>

                <div className="flex w-[60px] shrink-0 justify-end text-[12px] tabular-nums text-muted-foreground">
                  {it.status === "done" ? "100%" : it.status === "staged" ? "—" : `${it.progress}%`}
                </div>

                <div className="flex shrink-0 items-center gap-0.5">
                  {it.status === "uploading" && (
                    <button className="rounded p-1.5 text-muted-foreground hover:bg-surface-hover hover:text-foreground" title="Pause">
                      <Pause className="h-3.5 w-3.5" />
                    </button>
                  )}
                  {it.status === "paused" && (
                    <button className="rounded p-1.5 text-muted-foreground hover:bg-surface-hover hover:text-foreground" title="Resume">
                      <Play className="h-3.5 w-3.5" />
                    </button>
                  )}
                  {it.status === "failed" && (
                    <button className="rounded p-1.5 text-muted-foreground hover:bg-surface-hover hover:text-foreground" title="Retry">
                      <RotateCw className="h-3.5 w-3.5" />
                    </button>
                  )}
                  <button className="rounded p-1.5 text-muted-foreground hover:bg-surface-hover hover:text-destructive" title="Remove">
                    {it.status === "done" ? <Trash2 className="h-3.5 w-3.5" /> : <X className="h-3.5 w-3.5" />}
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      </div>

      <div className="flex items-center justify-between border-t border-border bg-surface px-5 py-2 text-[11px] text-muted-foreground">
        <span>Optimized for many small files · Resumable sessions enabled</span>
        <span className="flex items-center gap-1"><CheckCircle2 className="h-3 w-3 text-success" /> Connection stable</span>
      </div>
    </AppShell>
  );
}
