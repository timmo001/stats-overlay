"use client";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Event, listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { Settings } from "~/types/settings";
import { Stats } from "~/types/stats";

enum LoadingState {
  NotLoaded = -2,
  Loading = -1,
  Error = 0,
  Loaded = 1,
}

enum KeyboardShortcut {
  Alt_S = "Alt+S",
}

export default function HomePage() {
  const [loadingState, setLoadingState] = useState<LoadingState>(
    LoadingState.NotLoaded,
  );
  const [settings, setSettings] = useState<Settings>();
  const [stats, setStats] = useState<Stats>();

  async function hideWindow(): Promise<void> {
    console.log("Hiding window");
    await getCurrentWindow().hide();
  }

  async function showWindow(): Promise<void> {
    console.log("Showing window");
    const window = getCurrentWindow();
    if (!window) {
      console.error("Window not found");
      return;
    }

    if (await window.isVisible()) {
      console.log("Window already visible");
    } else {
      await window.show();
    }

    await invoke("set_window", {});
    console.log("Window set");
  }

  async function toggleWindow(): Promise<void> {
    console.log("Toggling window");
    const window = getCurrentWindow();
    if (!window) {
      console.error("Window not found");
      return;
    }

    if (await window.isVisible()) {
      await hideWindow();
    } else {
      await showWindow();
    }
  }

  useEffect(() => {
    if (loadingState > LoadingState.NotLoaded) return;
    showWindow();
    setLoadingState(LoadingState.Loading);
    invoke<Settings>("get_settings").then(async (newSettings: Settings) => {
      setSettings(newSettings);

      console.log("Get initial stats");
      const newStats = await invoke<Stats>("get_stats", {});
      setStats(newStats);
      console.log("Initial stats", newStats);

      listen("show", () => showWindow());

      listen<Stats>("stats", (event: Event<Stats>) => {
        // console.log("New stats", event.payload);
        setStats(event.payload);
      });

      listen<KeyboardShortcut>(
        "shortcut-event",
        (event: Event<KeyboardShortcut>) => {
          switch (event.payload) {
            case KeyboardShortcut.Alt_S:
              toggleWindow();
              break;
            default:
              console.warn("Unknown shortcut", event.payload);
              break;
          }
        },
      );

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
  if (loadingState !== LoadingState.Loaded || !settings || !stats) return null;

  const { usage, nvidia } = stats;

  return (
    <main className="flex min-h-screen w-full min-w-full flex-col items-start justify-between">
      <section className="container flex w-full min-w-full flex-row items-start justify-between gap-12 px-4 py-4">
        <h2 className="text-xl font-extrabold tracking-tight">
          {/* <span className="text-indigo-800">Stats</span> Overlay */}
        </h2>
        <h2 className="text-end font-light tracking-tight">
          <span className="ms-1 text-sm">CPU: </span>
          <span className="text-md font-semibold">{usage.cpu}%</span>
          <span className="ms-3 text-sm">MEM: </span>
          <span className="text-md font-semibold">{usage.memory}%</span>
          {nvidia && (
            <>
              <br />
              <span className="ms-1 text-sm">GPU: </span>
              <span className="text-md font-semibold">{nvidia.usage}%</span>
              <span className="text-md ms-2 font-semibold">
                {nvidia.temperature}°C
              </span>
            </>
          )}
        </h2>
      </section>
    </main>
  );
}
