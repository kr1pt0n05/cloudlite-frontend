import { createFileRoute, Link } from "@tanstack/react-router";
import { AppShell } from "@/components/cloudlite/AppShell";
import { Share2 } from "lucide-react";

export const Route = createFileRoute("/shared")({
  component: () => (
    <AppShell>
      <EmptyState
        title="Nothing shared with you yet"
        description="Files and folders shared by other people on cloud.acme.dev will appear here."
      />
    </AppShell>
  ),
  head: () => ({ meta: [{ title: "Shared — CloudLite" }] }),
});

function EmptyState({ title, description }: { title: string; description: string }) {
  return (
    <div className="flex flex-1 items-center justify-center p-10">
      <div className="max-w-sm text-center">
        <div className="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl bg-primary-soft ring-1 ring-primary/15">
          <Share2 className="h-6 w-6 text-primary" />
        </div>
        <h2 className="mt-4 text-[15px] font-semibold text-foreground">{title}</h2>
        <p className="mt-1 text-[12.5px] text-muted-foreground">{description}</p>
        <Link to="/files" className="mt-4 inline-flex h-8 items-center rounded-md bg-primary px-3 text-[12.5px] font-medium text-primary-foreground hover:bg-primary/90">
          Browse my files
        </Link>
      </div>
    </div>
  );
}
