import type { ReactNode } from "react";
import { Sidebar } from "./Sidebar";
import { Minus, Square, X, Search } from "lucide-react";

export function AppShell({ children }: { children: ReactNode }) {
  return (
    <div className="app-window flex h-screen w-screen flex-col overflow-hidden">
      {/* Faux Tauri title bar */}
      <div className="app-titlebar flex items-center justify-between px-3">
        <div className="flex items-center gap-2 text-[12px] text-muted-foreground">
          <span className="font-medium text-foreground">CloudLite</span>
          <span className="text-border-strong">·</span>
          <span>cloud.acme.dev</span>
        </div>
        <div className="pointer-events-none flex flex-1 justify-center">
          <div className="pointer-events-auto flex h-6 w-[320px] items-center gap-2 rounded-md border border-border bg-surface-2 px-2 text-[12px] text-muted-foreground shadow-xs">
            <Search className="h-3.5 w-3.5" />
            <span>Search files, folders, shares…</span>
            <span className="ml-auto rounded bg-muted px-1.5 font-mono text-[10px]">⌘K</span>
          </div>
        </div>
        <div className="flex items-center gap-1 text-muted-foreground">
          <button className="flex h-6 w-6 items-center justify-center rounded hover:bg-surface-hover">
            <Minus className="h-3.5 w-3.5" />
          </button>
          <button className="flex h-6 w-6 items-center justify-center rounded hover:bg-surface-hover">
            <Square className="h-3 w-3" />
          </button>
          <button className="flex h-6 w-6 items-center justify-center rounded hover:bg-destructive hover:text-destructive-foreground">
            <X className="h-3.5 w-3.5" />
          </button>
        </div>
      </div>

      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex flex-1 flex-col overflow-hidden bg-background">{children}</main>
      </div>
    </div>
  );
}
