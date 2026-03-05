import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { mockListen } from "@/test/mocks/tauri";
import { useTauriEvent } from "../use-tauri-event";

describe("useTauriEvent", () => {
  let capturedCallback: ((event: { payload: unknown }) => void) | null = null;
  const mockUnlisten = vi.fn();

  beforeEach(() => {
    capturedCallback = null;
    mockUnlisten.mockReset();
    mockListen.mockReset();
    mockListen.mockImplementation(
      (_event: string, cb: (event: { payload: unknown }) => void) => {
        capturedCallback = cb;
        return Promise.resolve(mockUnlisten);
      },
    );
  });

  it("calls listen with event name on mount", () => {
    renderHook(() => useTauriEvent("pipeline-progress", vi.fn()));
    expect(mockListen).toHaveBeenCalledWith(
      "pipeline-progress",
      expect.any(Function),
    );
  });

  it("calls unlisten on unmount", async () => {
    const { unmount } = renderHook(() =>
      useTauriEvent("pipeline-progress", vi.fn()),
    );
    unmount();

    // Allow promise to resolve
    await vi.waitFor(() => {
      expect(mockUnlisten).toHaveBeenCalled();
    });
  });

  it("passes event payload to handler", () => {
    const handler = vi.fn();
    renderHook(() => useTauriEvent("test-event", handler));

    act(() => {
      capturedCallback?.({ payload: { stage: "ReadingInput" } });
    });

    expect(handler).toHaveBeenCalledWith({ stage: "ReadingInput" });
  });

  it("uses latest handler via ref pattern", () => {
    const handler1 = vi.fn();
    const handler2 = vi.fn();

    const { rerender } = renderHook(
      ({ handler }) => useTauriEvent("test-event", handler),
      { initialProps: { handler: handler1 } },
    );

    // Update handler without re-subscribing
    rerender({ handler: handler2 });

    // Should not re-subscribe (listen called only once)
    expect(mockListen).toHaveBeenCalledTimes(1);

    // Should call the latest handler
    act(() => {
      capturedCallback?.({ payload: "data" });
    });

    expect(handler1).not.toHaveBeenCalled();
    expect(handler2).toHaveBeenCalledWith("data");
  });
});
