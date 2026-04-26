import { createFileRoute } from "@tanstack/react-router";
import { AppShell } from "@/components/cloudlite/AppShell";
import { FileText, Image as ImageIcon, FileVideo, FileSpreadsheet, FileCode } from "lucide-react";

export const Route = createFileRoute("/recent")({
  component: RecentPage,
  head: () => ({ meta: [{ title: "Recent — CloudLite" }] }),
});

const groups = [
  {
    label: "Today",
    items: [
      { name: "Q4 Roadmap.pdf", path: "/Docs", time: "10 min ago", Icon: FileText },
      { name: "hero-banner-final.png", path: "/Design", time: "1h ago", Icon: ImageIcon },
      { name: "release-notes.md", path: "/Engineering", time: "2h ago", Icon: FileCode },
    ],
  },
  {
    label: "Yesterday",
    items: [
      { name: "demo-recording.mp4", path: "/Marketing", time: "Yesterday, 18:24", Icon: FileVideo },
      { name: "customers-export.csv", path: "/Sales", time: "Yesterday, 14:02", Icon: FileSpreadsheet },
    ],
  },
  {
    label: "This week",
    items: [
      { name: "tokens.json", path: "/Design", time: "Mon, 09:11", Icon: FileCode },
      { name: "infra-notes.txt", path: "/Engineering", time: "Mon, 08:40", Icon: FileText },
    ],
  },
];

function RecentPage() {
  return (
    <AppShell>
      <div className="flex-1 overflow-auto px-6 py-6">
        <div className="mx-auto max-w-[860px]">
          <h1 className="text-[18px] font-semibold tracking-tight text-foreground">Recent activity</h1>
          <p className="mt-0.5 text-[12px] text-muted-foreground">Files you've recently opened, edited, or uploaded.</p>

          <div className="mt-6 space-y-6">
            {groups.map((g) => (
              <div key={g.label}>
                <div className="mb-2 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">{g.label}</div>
                <div className="overflow-hidden rounded-xl border border-border bg-surface">
                  {g.items.map((it, i) => (
                    <div
                      key={it.name}
                      className={`row-hover flex items-center gap-3 px-4 py-2.5 ${i > 0 ? "border-t border-border" : ""}`}
                    >
                      <div className="flex h-8 w-8 items-center justify-center rounded-md bg-surface-2 ring-1 ring-border">
                        <it.Icon className="h-4 w-4 text-muted-foreground" />
                      </div>
                      <div className="min-w-0 flex-1">
                        <div className="truncate text-[13px] font-medium text-foreground">{it.name}</div>
                        <div className="truncate text-[11px] text-muted-foreground">{it.path}</div>
                      </div>
                      <div className="text-[11px] tabular-nums text-muted-foreground">{it.time}</div>
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </AppShell>
  );
}
