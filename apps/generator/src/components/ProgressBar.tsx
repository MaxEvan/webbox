interface ProgressBarProps {
  progress: number;
  step: string;
}

export function ProgressBar({ progress, step }: ProgressBarProps) {
  return (
    <div className="flex flex-col gap-2">
      <div className="h-2 bg-app-secondary rounded-full overflow-hidden">
        <div
          className="h-full bg-app-primary transition-all duration-300 ease-out"
          style={{ width: `${progress}%` }}
        />
      </div>
      <span className="text-sm text-gray-400">{step}</span>
    </div>
  );
}
