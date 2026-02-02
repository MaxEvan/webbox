import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface GenerateProgress {
  step: string;
  progress: number;
}

interface GenerateResult {
  success: boolean;
  path?: string;
  error?: string;
}

export function useGenerateApp() {
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState<GenerateProgress | null>(null);
  const [result, setResult] = useState<GenerateResult | null>(null);

  useEffect(() => {
    const unlisten = listen<GenerateProgress>("generate-progress", (event) => {
      setProgress(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const generate = async (data: {
    name: string;
    url: string;
    iconPath: string;
    outputDir: string;
  }) => {
    setIsGenerating(true);
    setResult(null);
    setProgress({ step: "Starting...", progress: 0 });

    try {
      const outputPath = await invoke<string>("generate_app", {
        request: {
          name: data.name,
          url: data.url,
          icon_path: data.iconPath,
          output_dir: data.outputDir,
        },
      });

      setResult({ success: true, path: outputPath });
    } catch (error) {
      setResult({ success: false, error: String(error) });
    } finally {
      setIsGenerating(false);
      setProgress(null);
    }
  };

  const reset = () => {
    setResult(null);
    setProgress(null);
  };

  const openInFinder = async () => {
    if (result?.path) {
      await invoke("open_in_finder", { path: result.path });
    }
  };

  const launchApp = async () => {
    if (result?.path) {
      await invoke("launch_app", { path: result.path });
    }
  };

  return {
    isGenerating,
    progress,
    result,
    generate,
    reset,
    openInFinder,
    launchApp,
  };
}
