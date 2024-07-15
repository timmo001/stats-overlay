"use client";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { Settings } from "~/types/settings";
import { Stats } from "~/types/stats";

enum LoadingState {
  NotLoaded = -2,
  Loading = -1,
  Error = 0,
  Loaded = 1,
}

export default function HomePage() {
  const [loadingState, setLoadingState] = useState<LoadingState>(
    LoadingState.NotLoaded,
  );
  const [settings, setSettings] = useState<Settings>();
  const [stats, setStats] = useState<Stats>({ cpu: 0, ram: 0, gpu: 0, fps: 0 });

  // async function hideWindow(): Promise<void> {
  //   console.log("Hiding window");
  //   await getCurrentWindow().hide();
  // }

  async function showWindow(): Promise<void> {
    console.log("Showing window");
    const window = getCurrentWindow();
    if (!window) {
      console.error("Window not found");
      return;
    }

    if (await window.isVisible()) {
      console.log("Window already visible");
      return;
    }

    await window.show();
  }

  async function updateStats(): Promise<void> {
    console.log("Update stats");

    const newStats = await invoke<Stats>("get_stats", {});
    setStats(newStats);
    console.log("New stats", newStats);
  }

  useEffect(() => {
    if (loadingState > LoadingState.NotLoaded) return;
    showWindow().then(async () => {
      await invoke("set_window", {});
      console.log("Window set");
    });
    setLoadingState(LoadingState.Loading);
    invoke<Settings>("get_settings").then(async (newSettings: Settings) => {
      setSettings(newSettings);

      await updateStats();

      setLoadingState(LoadingState.Loaded);
    });
  }, []);

  if (loadingState === LoadingState.Error)
    return (
      <main className="flex min-h-screen w-full flex-col items-start justify-start">
        <section className="container flex w-full flex-row items-start justify-center gap-12 px-4 py-4">
          <h2 className="text-2xl font-extrabold tracking-tight text-red-600">
            Error loading data for Stats Overlay
          </h2>
        </section>
      </main>
    );

  // TODO: Replace with shadcn spinner
  if (loadingState !== LoadingState.Loaded || !settings) return null;

  const { cpu, ram, gpu, fps } = stats;

  return (
    <main className="flex min-h-screen w-full flex-col items-start justify-between">
      <section className="container flex w-full flex-row items-start justify-between gap-12 px-4 py-4">
        <h2 className="text-2xl font-extrabold tracking-tight">
          <span className="text-indigo-800">Stats</span> Overlay
        </h2>
        <h2 className="text-2xl font-extrabold tracking-tight">
          CPU: {cpu}% | RAM: {ram}% | GPU: {gpu}% | FPS: {fps}
        </h2>
      </section>
    </main>
  );
}
