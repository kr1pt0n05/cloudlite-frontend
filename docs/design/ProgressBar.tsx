import { cn } from "@/lib/utils";

type Tone = "primary" | "success" | "warning" | "danger";

const tones: Record<Tone, string> = {
  primary: "bg-primary",
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-destructive",
};

export function ProgressBar({
  value,
  tone = "primary",
  className,
  indeterminate,
}: {
  value?: number;
  tone?: Tone;
  className?: string;
  indeterminate?: boolean;
}) {
  return (
    <div className={cn("h-1.5 w-full overflow-hidden rounded-full bg-muted", className)}>
      <div
        className={cn(
          "h-full rounded-full transition-[width] duration-300",
          tones[tone],
          indeterminate && "animate-pulse",
        )}
        style={{ width: indeterminate ? "40%" : `${Math.min(100, Math.max(0, value ?? 0))}%` }}
      />
    </div>
  );
}
