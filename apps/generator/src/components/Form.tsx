import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { homeDir } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ProgressBar } from "./ProgressBar";

interface FormProps {
  onSubmit: (data: { name: string; url: string; iconPath: string; outputDir: string }) => void;
  isGenerating: boolean;
  progress: { step: string; progress: number } | null;
  error: string | null;
}

interface FormErrors {
  name?: string;
  url?: string;
  icon?: string;
  output?: string;
}

export function Form({ onSubmit, isGenerating, progress, error }: FormProps) {
  const [appName, setAppName] = useState("");
  const [url, setUrl] = useState("");
  const [iconPath, setIconPath] = useState("");
  const [outputDir, setOutputDir] = useState("");
  const [errors, setErrors] = useState<FormErrors>({});

  // Set default output directory
  useEffect(() => {
    homeDir().then((home) => {
      if (home) {
        setOutputDir(`${home}Applications`);
      }
    });
  }, []);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!appName.trim()) {
      newErrors.name = "App name is required";
    } else if (!/^[a-zA-Z0-9\s-]+$/.test(appName)) {
      newErrors.name = "Only letters, numbers, spaces, and hyphens allowed";
    }

    if (!url.trim()) {
      newErrors.url = "URL is required";
    } else {
      try {
        const testUrl = url.startsWith("http") ? url : `https://${url}`;
        const parsed = new URL(testUrl);
        if (!parsed.hostname.includes(".")) {
          newErrors.url = "Please enter a valid URL";
        }
      } catch {
        newErrors.url = "Please enter a valid URL";
      }
    }

    if (!iconPath) {
      newErrors.icon = "Please select an icon image";
    }

    if (!outputDir) {
      newErrors.output = "Please select an output directory";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSelectIcon = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "webp"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      setIconPath(selected);
      setErrors((prev) => ({ ...prev, icon: undefined }));
    }
  };

  const handleSelectOutput = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: outputDir,
    });

    if (selected && typeof selected === "string") {
      setOutputDir(selected);
      setErrors((prev) => ({ ...prev, output: undefined }));
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    let normalizedUrl = url.trim();
    if (!normalizedUrl.startsWith("http://") && !normalizedUrl.startsWith("https://")) {
      normalizedUrl = `https://${normalizedUrl}`;
    }

    onSubmit({
      name: appName.trim(),
      url: normalizedUrl,
      iconPath,
      outputDir,
    });
  };

  const iconPreviewSrc = iconPath ? convertFileSrc(iconPath) : null;

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-6">
      {/* App Name */}
      <div className="flex flex-col gap-2">
        <label htmlFor="appName" className="text-sm font-medium">
          App Name
        </label>
        <input
          id="appName"
          type="text"
          value={appName}
          onChange={(e) => {
            setAppName(e.target.value);
            setErrors((prev) => ({ ...prev, name: undefined }));
          }}
          placeholder="e.g., Notion"
          disabled={isGenerating}
          className="input-field"
        />
        {errors.name && <span className="text-app-error text-sm">{errors.name}</span>}
      </div>

      {/* URL */}
      <div className="flex flex-col gap-2">
        <label htmlFor="url" className="text-sm font-medium">
          Website URL
        </label>
        <input
          id="url"
          type="text"
          value={url}
          onChange={(e) => {
            setUrl(e.target.value);
            setErrors((prev) => ({ ...prev, url: undefined }));
          }}
          placeholder="e.g., notion.so"
          disabled={isGenerating}
          className="input-field"
        />
        {errors.url && <span className="text-app-error text-sm">{errors.url}</span>}
      </div>

      {/* Icon */}
      <div className="flex flex-col gap-2">
        <label className="text-sm font-medium">App Icon</label>
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={handleSelectIcon}
            disabled={isGenerating}
            className="btn"
          >
            {iconPath ? "Change Icon" : "Select Icon"}
          </button>
          {iconPreviewSrc && (
            <div className="flex items-center gap-2">
              <img
                src={iconPreviewSrc}
                alt="Icon preview"
                className="w-10 h-10 rounded-lg object-cover border border-app-border"
              />
              <span className="text-gray-400 text-sm truncate max-w-[150px]">
                {iconPath.split("/").pop()}
              </span>
            </div>
          )}
        </div>
        {errors.icon && <span className="text-app-error text-sm">{errors.icon}</span>}
      </div>

      {/* Output Directory */}
      <div className="flex flex-col gap-2">
        <label className="text-sm font-medium">Output Location</label>
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={handleSelectOutput}
            disabled={isGenerating}
            className="btn"
          >
            Change
          </button>
          <span className="text-gray-400 text-sm truncate flex-1">{outputDir}</span>
        </div>
        {errors.output && <span className="text-app-error text-sm">{errors.output}</span>}
      </div>

      {/* Progress */}
      {isGenerating && progress && (
        <ProgressBar progress={progress.progress} step={progress.step} />
      )}

      {/* Error Message */}
      {error && (
        <div className="bg-app-error/10 border border-app-error rounded-lg p-4 text-sm">
          <strong>Error:</strong> {error}
        </div>
      )}

      {/* Submit Button */}
      <button
        type="submit"
        disabled={isGenerating}
        className="btn btn-primary w-full mt-2"
      >
        {isGenerating ? "Creating..." : "Create App"}
      </button>
    </form>
  );
}
