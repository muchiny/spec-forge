import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import { renderWithProviders } from "@/test/test-utils";
import { useOllamaStore } from "@/shared/stores/use-ollama-store";
import { LlmStatusBadge } from "../llm-status-badge";

vi.mock("../../api/queries", () => ({
  useLlmStatus: vi.fn().mockReturnValue({ data: undefined, isLoading: true }),
  useOllamaSystemStatus: vi
    .fn()
    .mockReturnValue({ data: undefined, isLoading: true }),
}));

vi.mock("../../api/mutations", () => ({
  usePullModel: vi.fn().mockReturnValue({ mutate: vi.fn(), isPending: false }),
  useCancelPull: vi.fn().mockReturnValue({ mutate: vi.fn() }),
  useInitializeLlm: vi.fn().mockReturnValue({ mutate: vi.fn() }),
}));

vi.mock("../../hooks/use-model-pull-progress", () => ({
  useModelPullProgress: vi.fn(),
}));

import { useOllamaSystemStatus } from "../../api/queries";
const mockUseOllamaSystemStatus = vi.mocked(useOllamaSystemStatus);

describe("LlmStatusBadge", () => {
  beforeEach(() => {
    useOllamaStore.setState({
      status: "checking",
      modelName: "",
      url: "",
      pullProgress: 0,
      pullStatus: "",
      error: null,
    });
  });

  it("shows loading state", () => {
    mockUseOllamaSystemStatus.mockReturnValue({
      data: undefined,
      isLoading: true,
    } as ReturnType<typeof mockUseOllamaSystemStatus>);
    renderWithProviders(<LlmStatusBadge />);
    expect(screen.getByTestId("llm-status-loading")).toBeInTheDocument();
  });

  it("shows ready state with model name", () => {
    useOllamaStore.setState({
      status: "ready",
      modelName: "qwen3:8b",
      url: "http://localhost:11434",
    });
    mockUseOllamaSystemStatus.mockReturnValue({
      data: {
        ollama_running: true,
        model_name: "qwen3:8b",
        model_installed: true,
        url: "http://localhost:11434",
      },
      isLoading: false,
    } as ReturnType<typeof mockUseOllamaSystemStatus>);
    renderWithProviders(<LlmStatusBadge />);
    const badge = screen.getByTestId("llm-status-badge");
    expect(badge).toBeInTheDocument();
    expect(screen.getByText("qwen3:8b")).toBeInTheDocument();
  });

  it("shows not running state", () => {
    useOllamaStore.setState({ status: "not_running" });
    mockUseOllamaSystemStatus.mockReturnValue({
      data: {
        ollama_running: false,
        model_name: "qwen3:8b",
        model_installed: false,
        url: "http://localhost:11434",
      },
      isLoading: false,
    } as ReturnType<typeof mockUseOllamaSystemStatus>);
    renderWithProviders(<LlmStatusBadge />);
    expect(screen.getByTestId("llm-status-badge")).toBeInTheDocument();
  });

  it("shows model missing state with install button", () => {
    useOllamaStore.setState({ status: "model_missing", modelName: "qwen3:8b" });
    mockUseOllamaSystemStatus.mockReturnValue({
      data: {
        ollama_running: true,
        model_name: "qwen3:8b",
        model_installed: false,
        url: "http://localhost:11434",
      },
      isLoading: false,
    } as ReturnType<typeof mockUseOllamaSystemStatus>);
    renderWithProviders(<LlmStatusBadge />);
    expect(screen.getByTestId("llm-install-button")).toBeInTheDocument();
  });

  it("shows pulling state with progress", () => {
    useOllamaStore.setState({
      status: "pulling",
      pullProgress: 45,
      pullStatus: "downloading",
    });
    mockUseOllamaSystemStatus.mockReturnValue({
      data: {
        ollama_running: true,
        model_name: "qwen3:8b",
        model_installed: false,
        url: "http://localhost:11434",
      },
      isLoading: false,
    } as ReturnType<typeof mockUseOllamaSystemStatus>);
    renderWithProviders(<LlmStatusBadge />);
    expect(screen.getByText("45%")).toBeInTheDocument();
    expect(screen.getByTestId("llm-cancel-pull")).toBeInTheDocument();
  });
});
