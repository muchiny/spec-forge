import { create } from "zustand";

export type OllamaStatus =
  | "checking"
  | "not_running"
  | "model_missing"
  | "pulling"
  | "ready"
  | "error";

interface OllamaState {
  status: OllamaStatus;
  modelName: string;
  url: string;
  pullProgress: number;
  pullStatus: string;
  error: string | null;

  setChecking: () => void;
  setNotRunning: () => void;
  setModelMissing: (modelName: string, url: string) => void;
  setPulling: (pullStatus: string, pullProgress: number) => void;
  setReady: (modelName: string, url: string) => void;
  setError: (error: string) => void;
  reset: () => void;
}

export const useOllamaStore = create<OllamaState>((set) => ({
  status: "checking",
  modelName: "",
  url: "",
  pullProgress: 0,
  pullStatus: "",
  error: null,

  setChecking: () => set({ status: "checking", error: null }),
  setNotRunning: () => set({ status: "not_running", error: null }),
  setModelMissing: (modelName, url) =>
    set({ status: "model_missing", modelName, url, error: null }),
  setPulling: (pullStatus, pullProgress) =>
    set({ status: "pulling", pullStatus, pullProgress }),
  setReady: (modelName, url) =>
    set({
      status: "ready",
      modelName,
      url,
      error: null,
      pullProgress: 0,
      pullStatus: "",
    }),
  setError: (error) => set({ status: "error", error }),
  reset: () =>
    set({
      status: "checking",
      modelName: "",
      url: "",
      pullProgress: 0,
      pullStatus: "",
      error: null,
    }),
}));
