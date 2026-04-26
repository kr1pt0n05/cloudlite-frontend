import { createFileRoute, useRouter } from "@tanstack/react-router";
import { AppShell } from "@/components/cloudlite/AppShell";
import { Badge } from "@/components/cloudlite/Badge";
import { Server, Folder, Gauge, HardDrive, LogOut, ChevronRight, Check } from "lucide-react";
import { useState } from "react";
import { cn } from "@/lib/utils";

export const Route = createFileRoute("/settings")({
  component: SettingsPage,
  head: () => ({ meta: [{ title: "Settings — CloudLite" }] }),
});

const sections = [
  { id: "account", label: "Server & Account", icon: Server },
  { id: "sync", label: "Sync folder", icon: Folder },
  { id: "network", label: "Bandwidth", icon: Gauge },
  { id: "storage", label: "Cache & storage", icon: HardDrive },
] as const;

function Row({ label, hint, children }: { label: string; hint?: string; children: React.ReactNode }) {
  return (
    <div className="flex items-start justify-between gap-6 border-b border-border py-3 last:border-0">
      <div className="min-w-0 flex-1">
        <div className="text-[13px] font-medium text-foreground">{label}</div>
        {hint && <div className="mt-0.5 text-[11px] text-muted-foreground">{hint}</div>}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

function Toggle({ on, onChange }: { on: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      onClick={() => onChange(!on)}
      className={cn(
        "relative h-5 w-9 rounded-full transition-colors",
        on ? "bg-primary" : "bg-muted",
      )}
    >
      <span
        className={cn(
          "absolute top-0.5 h-4 w-4 rounded-full bg-surface shadow-sm transition-transform",
          on ? "translate-x-[18px]" : "translate-x-0.5",
        )}
      />
    </button>
  );
}

function NumberInput({ value, suffix }: { value: string; suffix?: string }) {
  return (
    <div className="flex h-8 w-[140px] items-center overflow-hidden rounded-md border border-input bg-surface">
      <input defaultValue={value} className="h-full w-full bg-transparent px-2.5 text-right text-[13px] tabular-nums outline-none" />
      {suffix && <span className="border-l border-border bg-surface-2 px-2 text-[11px] text-muted-foreground">{suffix}</span>}
    </div>
  );
}

function SettingsPage() {
  const [section, setSection] = useState<(typeof sections)[number]["id"]>("account");
  const [autoSync, setAutoSync] = useState(true);
  const [throttle, setThrottle] = useState(false);
  const [lan, setLan] = useState(true);
  const router = useRouter();

  return (
    <AppShell>
      <div className="flex flex-1 overflow-hidden">
        {/* Settings rail */}
        <div className="w-[200px] shrink-0 border-r border-border bg-surface-2 p-3">
          <div className="px-1.5 pb-2 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">Settings</div>
          <nav className="space-y-0.5">
            {sections.map((s) => {
              const active = section === s.id;
              return (
                <button
                  key={s.id}
                  onClick={() => setSection(s.id)}
                  className={cn(
                    "flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-[12.5px]",
                    active ? "bg-surface text-foreground shadow-xs" : "text-muted-foreground hover:bg-surface hover:text-foreground",
                  )}
                >
                  <s.icon className={cn("h-3.5 w-3.5", active ? "text-primary" : "text-muted-foreground")} />
                  {s.label}
                </button>
              );
            })}
          </nav>
        </div>

        {/* Pane */}
        <div className="flex-1 overflow-auto">
          <div className="mx-auto max-w-[680px] px-8 py-8">
            {section === "account" && (
              <>
                <h2 className="text-[18px] font-semibold tracking-tight text-foreground">Server & Account</h2>
                <p className="mt-1 text-[12px] text-muted-foreground">The server you are connected to and your sign-in details.</p>

                <div className="mt-6 rounded-xl border border-border bg-surface p-4 shadow-xs">
                  <div className="flex items-center gap-3">
                    <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 ring-1 ring-primary/20">
                      <Server className="h-5 w-5 text-primary" />
                    </div>
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <span className="text-[14px] font-semibold text-foreground">cloud.acme.dev</span>
                        <Badge tone="success"><Check className="h-3 w-3" /> Connected</Badge>
                      </div>
                      <div className="text-[11px] text-muted-foreground">CloudLite Server v1.8.4 · Region eu-west-1</div>
                    </div>
                    <button className="text-[12px] font-medium text-primary hover:underline">Switch server</button>
                  </div>
                </div>

                <div className="mt-4 rounded-xl border border-border bg-surface px-4">
                  <Row label="Signed in as" hint="Authenticated via Keycloak (OIDC)">
                    <div className="flex items-center gap-2">
                      <div className="flex h-7 w-7 items-center justify-center rounded-full bg-primary/15 text-[11px] font-semibold text-primary">JD</div>
                      <div className="text-right">
                        <div className="text-[12.5px] font-medium text-foreground">Jamie Dolan</div>
                        <div className="text-[11px] text-muted-foreground">jamie@acme.dev</div>
                      </div>
                    </div>
                  </Row>
                  <Row label="Two-factor authentication" hint="Required by server policy">
                    <Badge tone="success">Enabled</Badge>
                  </Row>
                  <Row label="Auto-launch on startup">
                    <Toggle on={autoSync} onChange={setAutoSync} />
                  </Row>
                </div>

                <div className="mt-6 flex justify-end">
                  <button
                    onClick={() => router.navigate({ to: "/" })}
                    className="inline-flex h-9 items-center gap-2 rounded-md border border-destructive/30 bg-destructive-soft px-3 text-[12.5px] font-medium text-destructive hover:bg-destructive/10"
                  >
                    <LogOut className="h-3.5 w-3.5" /> Sign out & disconnect
                  </button>
                </div>
              </>
            )}

            {section === "sync" && (
              <>
                <h2 className="text-[18px] font-semibold tracking-tight text-foreground">Local sync folder</h2>
                <p className="mt-1 text-[12px] text-muted-foreground">Files in this folder are kept in sync with your server.</p>
                <div className="mt-6 rounded-xl border border-border bg-surface px-4">
                  <Row label="Sync folder" hint="Files inside this folder are tracked by CloudLite.">
                    <div className="flex items-center gap-2">
                      <code className="rounded-md bg-muted px-2 py-1 font-mono text-[11.5px] text-foreground">~/CloudLite/Acme</code>
                      <button className="rounded-md border border-border bg-surface px-2.5 py-1 text-[11.5px] font-medium hover:bg-surface-hover">Change…</button>
                    </div>
                  </Row>
                  <Row label="Sync hidden files" hint="Includes dotfiles like .env and .git">
                    <Toggle on={false} onChange={() => {}} />
                  </Row>
                  <Row label="Selective sync" hint="Choose which folders to keep locally">
                    <button className="inline-flex items-center gap-1 rounded-md border border-border bg-surface px-2.5 py-1 text-[11.5px] font-medium hover:bg-surface-hover">
                      Configure <ChevronRight className="h-3 w-3" />
                    </button>
                  </Row>
                </div>
              </>
            )}

            {section === "network" && (
              <>
                <h2 className="text-[18px] font-semibold tracking-tight text-foreground">Bandwidth & concurrency</h2>
                <p className="mt-1 text-[12px] text-muted-foreground">Tune transfers for your network. CloudLite is optimized for many small files.</p>
                <div className="mt-6 rounded-xl border border-border bg-surface px-4">
                  <Row label="Limit upload bandwidth">
                    <Toggle on={throttle} onChange={setThrottle} />
                  </Row>
                  <Row label="Max upload speed" hint="0 = unlimited">
                    <NumberInput value="50" suffix="MB/s" />
                  </Row>
                  <Row label="Max download speed" hint="0 = unlimited">
                    <NumberInput value="0" suffix="MB/s" />
                  </Row>
                  <Row label="Concurrent uploads" hint="Higher values speed up small-file batches">
                    <NumberInput value="16" suffix="conn" />
                  </Row>
                  <Row label="Concurrent downloads">
                    <NumberInput value="8" suffix="conn" />
                  </Row>
                  <Row label="LAN sync" hint="Transfer directly between devices on the same network">
                    <Toggle on={lan} onChange={setLan} />
                  </Row>
                </div>
              </>
            )}

            {section === "storage" && (
              <>
                <h2 className="text-[18px] font-semibold tracking-tight text-foreground">Cache & storage</h2>
                <p className="mt-1 text-[12px] text-muted-foreground">Manage local cache and on-demand file behavior.</p>
                <div className="mt-6 rounded-xl border border-border bg-surface px-4">
                  <Row label="Cache size limit">
                    <NumberInput value="5" suffix="GB" />
                  </Row>
                  <Row label="Files on demand" hint="Free up space by storing only file shortcuts locally">
                    <Toggle on={true} onChange={() => {}} />
                  </Row>
                  <Row label="Cache used" hint="2,184 cached files">
                    <span className="text-[13px] tabular-nums text-foreground">3.1 GB</span>
                  </Row>
                  <Row label="Clear local cache" hint="Cached files will be re-downloaded on demand">
                    <button className="rounded-md border border-border bg-surface px-2.5 py-1 text-[11.5px] font-medium hover:bg-surface-hover">Clear now</button>
                  </Row>
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    </AppShell>
  );
}
