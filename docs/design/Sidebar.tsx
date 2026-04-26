import { Link, useLocation } from "@tanstack/react-router";
import { type LucideIcon, FolderOpen, Share2, Clock, UploadCloud, RefreshCw, Settings, HardDrive } from "lucide-react";
import logo from "@/assets/cloudlite-logo.png";
import { cn } from "@/lib/utils";

type NavItem = { to: string; label: string; icon: LucideIcon; badge?: string };

const primary: NavItem[] = [
  { to: "/files", label: "My Files", icon: FolderOpen },
  { to: "/shared", label: "Shared", icon: Share2 },
  { to: "/recent", label: "Recent", icon: Clock },
];

const activity: NavItem[] = [
  { to: "/uploads", label: "Uploads", icon: UploadCloud, badge: "3" },
  { to: "/sync", label: "Sync Status", icon: RefreshCw },
];

const footer: NavItem[] = [{ to: "/settings", label: "Settings", icon: Settings }];

function NavLink({ item }: { item: NavItem }) {
  const { pathname } = useLocation();
  const active =
    item.to === "/files"
      ? pathname === "/files" || pathname === "/"
      : pathname.startsWith(item.to);
  const Icon = item.icon;
  return (
    <Link
      to={item.to}
      className={cn(
        "group flex items-center gap-2.5 rounded-md px-2.5 py-1.5 text-[13px] font-medium transition-colors",
        active
          ? "bg-sidebar-accent text-sidebar-accent-foreground"
          : "text-sidebar-foreground/80 hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground",
      )}
    >
      <Icon className={cn("h-[15px] w-[15px]", active ? "text-primary" : "text-muted-foreground group-hover:text-foreground")} />
      <span className="flex-1 truncate">{item.label}</span>
      {item.badge && (
        <span className="rounded bg-primary/10 px-1.5 text-[10px] font-semibold leading-4 text-primary">
          {item.badge}
        </span>
      )}
    </Link>
  );
}

function Section({ title, items }: { title?: string; items: NavItem[] }) {
  return (
    <div className="space-y-0.5">
      {title && (
        <div className="px-2.5 pb-1 pt-3 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/70">
          {title}
        </div>
      )}
      {items.map((i) => (
        <NavLink key={i.to} item={i} />
      ))}
    </div>
  );
}

export function Sidebar() {
  return (
    <aside className="flex h-full w-[232px] shrink-0 flex-col border-r border-sidebar-border bg-sidebar">
      {/* Brand / server */}
      <div className="flex items-center gap-2.5 px-3 py-3">
        <div className="flex h-7 w-7 items-center justify-center rounded-md bg-primary/10 ring-1 ring-primary/20">
          <img src={logo} alt="CloudLite" className="h-5 w-5" />
        </div>
        <div className="min-w-0 flex-1">
          <div className="truncate text-[13px] font-semibold text-sidebar-foreground">CloudLite</div>
          <div className="flex items-center gap-1 truncate text-[11px] text-muted-foreground">
            <span className="h-1.5 w-1.5 rounded-full bg-success" />
            cloud.acme.dev
          </div>
        </div>
      </div>

      <div className="flex-1 space-y-1 overflow-y-auto px-2 pb-2">
        <Section items={primary} />
        <Section title="Activity" items={activity} />
      </div>

      {/* Storage */}
      <div className="mx-2 mb-2 rounded-lg border border-sidebar-border bg-surface px-3 py-2.5 shadow-xs">
        <div className="mb-1.5 flex items-center justify-between text-[11px]">
          <span className="flex items-center gap-1.5 font-medium text-foreground">
            <HardDrive className="h-3.5 w-3.5 text-muted-foreground" />
            Storage
          </span>
          <span className="text-muted-foreground">62%</span>
        </div>
        <div className="h-1.5 w-full overflow-hidden rounded-full bg-muted">
          <div className="h-full rounded-full bg-primary" style={{ width: "62%" }} />
        </div>
        <div className="mt-1.5 text-[11px] text-muted-foreground">
          124.5 GB of 200 GB used
        </div>
      </div>

      <div className="border-t border-sidebar-border px-2 py-2">
        <Section items={footer} />
        <div className="mt-2 flex items-center gap-2 rounded-md px-2 py-1.5">
          <div className="flex h-7 w-7 items-center justify-center rounded-full bg-primary/15 text-[11px] font-semibold text-primary">
            JD
          </div>
          <div className="min-w-0 flex-1">
            <div className="truncate text-[12px] font-medium text-foreground">Jamie Dolan</div>
            <div className="truncate text-[11px] text-muted-foreground">jamie@acme.dev</div>
          </div>
        </div>
      </div>
    </aside>
  );
}
