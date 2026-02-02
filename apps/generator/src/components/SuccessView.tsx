interface SuccessViewProps {
  appPath: string;
  onOpenFinder: () => void;
  onLaunch: () => void;
  onReset: () => void;
}

export function SuccessView({ appPath, onOpenFinder, onLaunch, onReset }: SuccessViewProps) {
  return (
    <div className="text-center">
      {/* Success Icon */}
      <div className="w-16 h-16 bg-app-success rounded-full flex items-center justify-center text-3xl mx-auto mb-6">
        ✓
      </div>

      <h2 className="text-2xl font-semibold mb-2">App Created!</h2>

      <p className="text-gray-400 text-sm break-all mb-6 px-4">
        {appPath}
      </p>

      {/* Action Buttons */}
      <div className="flex gap-3 justify-center mb-4">
        <button onClick={onOpenFinder} className="btn">
          Show in Finder
        </button>
        <button onClick={onLaunch} className="btn btn-primary">
          Launch App
        </button>
      </div>

      <button onClick={onReset} className="btn btn-secondary">
        Create Another
      </button>
    </div>
  );
}
