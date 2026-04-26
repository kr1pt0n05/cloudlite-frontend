import { createFileRoute } from "@tanstack/react-router";
import { AppShell } from "@/components/cloudlite/AppShell";
import { Badge } from "@/components/cloudlite/Badge";
import { ProgressBar } from "@/components/cloudlite/ProgressBar";
import {
  CheckCircle2,
  AlertTriangle,
  RefreshCw,
  Folder,
  ArrowUp,
  ArrowDown,
  HardDrive,
  Clock,
  Pause,
  FileText,
  Image as ImageIcon,
} from "lucide-react";

export const Route = createFileRoute("/sync")({
  component: SyncPage,
  head: () => ({ meta: [{ title: "Sync Status — CloudLite" }] }),
});

const conflicts = [
  { name: "Q4 Roadmap.pdf", path: "/Docs", reason: "Modified locally and on server" },
  { name: "tokens.json", path: "/Design", reason: "File deleted on server, modified locally" },
];

const queue = [
  { name: "shoot-001.cr3", dir: "up", size: "48 MB" },
  { name: "shoot-002.cr3", dir: "up", size: "47 MB" },
  { name: "edit-master-v4.mp4", dir: "up", size: "2.1 GB" },
  { name: "team-photo.jpg", dir: "down", size: "4.2 MB" },
  { name: "weekly-notes.md", dir: "down", size: "8 KB" },
];

function SyncPage() {
  return (
    <AppShell>
      <div className="flex-1 overflow-auto p-6">
        <div className="mx-auto flex max-w-[980px] flex-col gap-5">
          <div>
            <h1 className="text-[18px] font-semibold tracking-tight text-foreground">Sync status</h1>
            <p className="mt-0.5 text-[12px] text-muted-foreground">Live status of your local CloudLite folder.</p>
          </div>

          {/* Health card */}
          <div className="overflow-hidden rounded-xl border border-border bg-surface shadow-sm">
            <div className="flex items-start gap-4 p-5">
              <div className="flex h-12 w-12 items-center justify-center rounded-full bg-success-soft ring-1 ring-success/20">
                <CheckCircle2 className="h-6 w-6 text-success" />
              </div>
              <div className="flex-1">
                <div className="flex items-center gap-2">
                  <h2 className="text-[15px] font-semibold text-foreground">Up to date</h2>
                  <Badge tone="success">Healthy</Badge>
                </div>
                <p className="mt-1 text-[12px] text-muted-foreground">
                  All changes in your local folder are reflected on the server.
                </p>
                <div className="mt-3 grid grid-cols-3 gap-4 text-[12px]">
                  <div className="flex items-center gap-2 text-muted-foreground">
                    <Clock className="h-3.5 w-3.5" /> Last sync <span className="font-medium text-foreground">2 min ago</span>
                  </div>
                  <div className="flex items-center gap-2 text-muted-foreground">
                    <RefreshCw className="h-3.5 w-3.5" /> Next check <span className="font-medium text-foreground">in 28s</span>
                  </div>
                  <div className="flex items-center gap-2 text-muted-foreground">
                    <Folder className="h-3.5 w-3.5" /> 12,438 files watched
                  </div>
                </div>
              </div>
              <div className="flex flex-col gap-2">
                <button className="inline-flex h-8 items-center gap-1.5 rounded-md bg-primary px-3 text-[12px] font-medium text-primary-foreground hover:bg-primary/90">
                  <RefreshCw className="h-3.5 w-3.5" /> Sync now
                </button>
                <button className="inline-flex h-8 items-center gap-1.5 rounded-md border border-border bg-surface px-3 text-[12px] font-medium text-foreground hover:bg-surface-hover">
                  <Pause className="h-3.5 w-3.5" /> Pause
                </button>
              </div>
            </div>
            <div className="border-t border-border bg-surface-2 px-5 py-2.5 text-[12px]">
              <span className="text-muted-foreground">Local folder</span>{" "}
              <span className="font-mono text-foreground">~/CloudLite/Acme</span>
              <button className="ml-2 text-primary hover:underline">Change…</button>
            </div>
          </div>

          {/* Stats grid */}
          <div className="grid grid-cols-3 gap-4">
            <div className="rounded-xl border border-border bg-surface p-4 shadow-xs">
              <div className="flex items-center justify-between">
                <span className="flex items-center gap-1.5 text-[11px] uppercase tracking-wider text-muted-foreground">
                  <ArrowUp className="h-3.5 w-3.5" /> Pending uploads
                </span>
                <Badge tone="info">3</Badge>
              </div>
              <div className="mt-2 text-[22px] font-semibold tabular-nums text-foreground">2.2 GB</div>
              <div className="mt-2"><ProgressBar value={62} tone="primary" /></div>
              <div className="mt-1.5 text-[11px] text-muted-foreground">62% of current batch</div>
            </div>

            <div className="rounded-xl border border-border bg-surface p-4 shadow-xs">
              <div className="flex items-center justify-between">
                <span className="flex items-center gap-1.5 text-[11px] uppercase tracking-wider text-muted-foreground">
                  <ArrowDown className="h-3.5 w-3.5" /> Pending downloads
                </span>
                <Badge tone="neutral">2</Badge>
              </div>
              <div className="mt-2 text-[22px] font-semibold tabular-nums text-foreground">4.2 MB</div>
              <div className="mt-2"><ProgressBar value={20} tone="success" /></div>
              <div className="mt-1.5 text-[11px] text-muted-foreground">Mostly small files</div>
            </div>

            <div className="rounded-xl border border-border bg-surface p-4 shadow-xs">
              <div className="flex items-center justify-between">
                <span className="flex items-center gap-1.5 text-[11px] uppercase tracking-wider text-muted-foreground">
                  <HardDrive className="h-3.5 w-3.5" /> Storage usage
                </span>
                <Badge tone="warning">62%</Badge>
              </div>
              <div className="mt-2 text-[22px] font-semibold tabular-nums text-foreground">124.5 GB</div>
              <div className="mt-2"><ProgressBar value={62} tone="warning" /></div>
              <div className="mt-1.5 text-[11px] text-muted-foreground">of 200 GB on cloud.acme.dev</div>
            </div>
          </div>

          {/* Conflicts */}
          <div className="overflow-hidden rounded-xl border border-warning/30 bg-warning-soft/40">
            <div className="flex items-center gap-2 border-b border-warning/20 px-4 py-2.5">
              <AlertTriangle className="h-4 w-4 text-warning" />
              <h3 className="text-[13px] font-semibold text-foreground">{conflicts.length} sync conflicts need your attention</h3>
            </div>
            <div className="divide-y divide-warning/15 bg-surface">
              {conflicts.map((c) => (
                <div key={c.name} className="flex items-center gap-3 px-4 py-2.5">
                  <FileText className="h-4 w-4 text-muted-foreground" />
                  <div className="min-w-0 flex-1">
                    <div className="text-[13px] font-medium text-foreground">{c.name}</div>
                    <div className="text-[11px] text-muted-foreground">{c.path} · {c.reason}</div>
                  </div>
                  <button className="rounded-md border border-border bg-surface px-2.5 py-1 text-[11px] font-medium hover:bg-surface-hover">Keep local</button>
                  <button className="rounded-md border border-border bg-surface px-2.5 py-1 text-[11px] font-medium hover:bg-surface-hover">Keep server</button>
                  <button className="rounded-md bg-primary px-2.5 py-1 text-[11px] font-medium text-primary-foreground hover:bg-primary/90">Resolve…</button>
                </div>
              ))}
            </div>
          </div>

          {/* Queue */}
          <div className="overflow-hidden rounded-xl border border-border bg-surface">
            <div className="flex items-center justify-between border-b border-border px-4 py-2.5">
              <h3 className="text-[13px] font-semibold text-foreground">Transfer queue</h3>
              <span className="text-[11px] text-muted-foreground">{queue.length} items</span>
            </div>
            <div className="divide-y divide-border">
              {queue.map((q) => (
                <div key={q.name} className="row-hover flex items-center gap-3 px-4 py-2 text-[12px]">
                  <div className={`flex h-6 w-6 items-center justify-center rounded ${q.dir === "up" ? "bg-info-soft text-info" : "bg-success-soft text-success"}`}>
                    {q.dir === "up" ? <ArrowUp className="h-3 w-3" /> : <ArrowDown className="h-3 w-3" />}
                  </div>
                  {q.dir === "up" ? <ImageIcon className="h-3.5 w-3.5 text-muted-foreground" /> : <FileText className="h-3.5 w-3.5 text-muted-foreground" />}
                  <span className="flex-1 truncate font-medium text-foreground">{q.name}</span>
                  <span className="tabular-nums text-muted-foreground">{q.size}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
