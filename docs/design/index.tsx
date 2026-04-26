import { createFileRoute, Link, useRouter } from "@tanstack/react-router";
import logo from "@/assets/cloudlite-logo.png";
import { ChevronRight, Server, ShieldCheck, KeyRound, Plus } from "lucide-react";

export const Route = createFileRoute("/")({
  component: LoginPage,
  head: () => ({
    meta: [
      { title: "Sign in — CloudLite" },
      { name: "description", content: "Connect to your self-hosted CloudLite server." },
    ],
  }),
});

const recent = [
  { name: "Acme Cloud", url: "cloud.acme.dev", user: "jamie@acme.dev", color: "bg-primary" },
  { name: "Personal", url: "files.dolan.io", user: "jd", color: "bg-success" },
  { name: "Studio NAS", url: "10.0.0.42:8443", user: "admin", color: "bg-warning" },
];

function LoginPage() {
  const router = useRouter();
  const go = () => router.navigate({ to: "/files" });

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background p-6">
      <div className="grid w-full max-w-[920px] grid-cols-1 overflow-hidden rounded-2xl border border-border bg-surface shadow-lg md:grid-cols-[1.05fr_1fr]">
        {/* Left: brand panel */}
        <div className="relative hidden flex-col justify-between bg-gradient-to-br from-primary/10 via-surface to-surface p-8 md:flex">
          <div>
            <div className="flex items-center gap-2.5">
              <div className="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/15 ring-1 ring-primary/25">
                <img src={logo} alt="" width={512} height={512} className="h-6 w-6" />
              </div>
              <div>
                <div className="text-[15px] font-semibold tracking-tight text-foreground">CloudLite</div>
                <div className="text-[11px] text-muted-foreground">Self-hosted · Lightweight</div>
              </div>
            </div>
            <h1 className="mt-10 text-[26px] font-semibold leading-tight tracking-tight text-foreground">
              Your files,<br />on your server.
            </h1>
            <p className="mt-3 max-w-[300px] text-[13px] leading-relaxed text-muted-foreground">
              A faster, simpler alternative to heavy self-hosted suites. Built for power users who want
              control without the bloat.
            </p>
          </div>
          <ul className="space-y-2.5 text-[12px] text-foreground/80">
            <li className="flex items-center gap-2"><ShieldCheck className="h-4 w-4 text-primary" /> End-to-end encrypted upload sessions</li>
            <li className="flex items-center gap-2"><Server className="h-4 w-4 text-primary" /> Connect to any CloudLite server</li>
            <li className="flex items-center gap-2"><KeyRound className="h-4 w-4 text-primary" /> OAuth, Keycloak & token auth</li>
          </ul>
        </div>

        {/* Right: form */}
        <div className="flex flex-col gap-5 p-8">
          <div>
            <h2 className="text-[18px] font-semibold tracking-tight text-foreground">Connect to a server</h2>
            <p className="mt-1 text-[12px] text-muted-foreground">Enter your CloudLite URL or pick a recent server.</p>
          </div>

          <div className="space-y-3">
            <label className="block">
              <span className="mb-1.5 block text-[12px] font-medium text-foreground">Server URL</span>
              <div className="flex h-9 items-center overflow-hidden rounded-md border border-input bg-surface focus-within:border-ring focus-within:ring-2 focus-within:ring-ring/20">
                <span className="border-r border-border bg-surface-2 px-2.5 text-[12px] text-muted-foreground">https://</span>
                <input
                  defaultValue="cloud.acme.dev"
                  className="h-full w-full bg-transparent px-2.5 text-[13px] text-foreground outline-none placeholder:text-muted-foreground"
                  placeholder="your-server.example.com"
                />
              </div>
            </label>

            <button
              onClick={go}
              className="inline-flex h-9 w-full items-center justify-center gap-2 rounded-md bg-primary text-[13px] font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
            >
              Continue with OAuth / Keycloak
              <ChevronRight className="h-4 w-4" />
            </button>

            <div className="flex items-center gap-3">
              <div className="h-px flex-1 bg-border" />
              <span className="text-[10px] uppercase tracking-wider text-muted-foreground">or</span>
              <div className="h-px flex-1 bg-border" />
            </div>

            <button
              onClick={go}
              className="inline-flex h-9 w-full items-center justify-center gap-2 rounded-md border border-border bg-surface text-[13px] font-medium text-foreground hover:bg-surface-hover"
            >
              Use access token
            </button>
          </div>

          {/* Recent servers */}
          <div>
            <div className="mb-2 flex items-center justify-between">
              <span className="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">Recent servers</span>
              <button className="flex items-center gap-1 text-[11px] font-medium text-primary hover:underline">
                <Plus className="h-3 w-3" /> Add
              </button>
            </div>
            <div className="overflow-hidden rounded-md border border-border">
              {recent.map((r, i) => (
                <button
                  key={r.url}
                  onClick={go}
                  className={`row-hover flex w-full items-center gap-3 px-3 py-2 text-left ${i > 0 ? "border-t border-border" : ""}`}
                >
                  <div className={`h-7 w-7 rounded-md ${r.color}/15 flex items-center justify-center ring-1 ring-border`}>
                    <Server className="h-3.5 w-3.5 text-foreground/70" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-[12px] font-medium text-foreground">{r.name}</div>
                    <div className="truncate text-[11px] text-muted-foreground">{r.url} · {r.user}</div>
                  </div>
                  <ChevronRight className="h-3.5 w-3.5 text-muted-foreground" />
                </button>
              ))}
            </div>
          </div>

          <div className="mt-auto flex items-center justify-between text-[11px] text-muted-foreground">
            <span>v0.4.2 · Tauri build</span>
            <Link to="/settings" className="hover:text-foreground">Advanced</Link>
          </div>
        </div>
      </div>
    </div>
  );
}
