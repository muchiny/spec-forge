import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useTauriMutation } from "../use-tauri-mutation";
import { useToastStore } from "@/shared/ui/toast-store";
import { createWrapper } from "@/test/test-utils";

describe("useTauriMutation", () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it("shows success toast on successful mutation", async () => {
    const mutationFn = vi.fn().mockResolvedValue("result");

    const { result } = renderHook(
      () =>
        useTauriMutation({
          mutationFn,
          successMessage: "Done!",
        }),
      { wrapper: createWrapper() },
    );

    await act(async () => {
      result.current.mutate();
    });

    await waitFor(() => {
      const toasts = useToastStore.getState().toasts;
      expect(toasts).toHaveLength(1);
      expect(toasts[0].variant).toBe("success");
      expect(toasts[0].message).toBe("Done!");
    });
  });

  it("shows error toast with err.message by default", async () => {
    const mutationFn = vi.fn().mockRejectedValue(new Error("Network error"));

    const { result } = renderHook(() => useTauriMutation({ mutationFn }), {
      wrapper: createWrapper(),
    });

    await act(async () => {
      result.current.mutate();
    });

    await waitFor(() => {
      const toasts = useToastStore.getState().toasts;
      expect(toasts).toHaveLength(1);
      expect(toasts[0].variant).toBe("error");
      expect(toasts[0].message).toBe("Network error");
    });
  });

  it("uses custom errorMessage string", async () => {
    const mutationFn = vi.fn().mockRejectedValue(new Error("err"));

    const { result } = renderHook(
      () =>
        useTauriMutation({
          mutationFn,
          errorMessage: "Custom error",
        }),
      { wrapper: createWrapper() },
    );

    await act(async () => {
      result.current.mutate();
    });

    await waitFor(() => {
      expect(useToastStore.getState().toasts[0].message).toBe("Custom error");
    });
  });

  it("uses custom errorMessage function", async () => {
    const mutationFn = vi.fn().mockRejectedValue(new Error("err"));

    const { result } = renderHook(
      () =>
        useTauriMutation({
          mutationFn,
          errorMessage: (err) => `Error: ${err.message}`,
        }),
      { wrapper: createWrapper() },
    );

    await act(async () => {
      result.current.mutate();
    });

    await waitFor(() => {
      expect(useToastStore.getState().toasts[0].message).toBe("Error: err");
    });
  });

  it("calls onSuccess callback", async () => {
    const onSuccess = vi.fn();
    const mutationFn = vi.fn().mockResolvedValue("data");

    const { result } = renderHook(
      () => useTauriMutation({ mutationFn, onSuccess }),
      { wrapper: createWrapper() },
    );

    await act(async () => {
      result.current.mutate();
    });

    await waitFor(() => {
      expect(onSuccess).toHaveBeenCalledWith("data", undefined);
    });
  });

  it("uses successMessage as function", async () => {
    const mutationFn = vi.fn().mockResolvedValue("result-data");

    const { result } = renderHook(
      () =>
        useTauriMutation({
          mutationFn,
          successMessage: (data) => `Got: ${data}`,
        }),
      { wrapper: createWrapper() },
    );

    await act(async () => {
      result.current.mutate();
    });

    await waitFor(() => {
      expect(useToastStore.getState().toasts[0].message).toBe(
        "Got: result-data",
      );
    });
  });
});
