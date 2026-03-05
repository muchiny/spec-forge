import { describe, it, expect, beforeEach, vi } from "vitest";
import { mockInvoke } from "@/test/mocks/tauri";
import { tauriInvoke, onIpcTiming } from "../tauri-client";

describe("tauriInvoke", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("calls invoke with cmd and args", async () => {
    mockInvoke.mockResolvedValue({ ok: true });
    await tauriInvoke("check_llm_status", { key: "val" });
    expect(mockInvoke).toHaveBeenCalledWith("check_llm_status", { key: "val" });
  });

  it("returns the result on success", async () => {
    mockInvoke.mockResolvedValue({ ready: true });
    const result = await tauriInvoke("check_llm_status");
    expect(result).toEqual({ ready: true });
  });

  it("wraps string errors in Error", async () => {
    mockInvoke.mockRejectedValue("LLM offline");
    await expect(tauriInvoke("check_llm_status")).rejects.toThrow(
      "LLM offline",
    );
  });

  it("wraps non-string errors via String()", async () => {
    mockInvoke.mockRejectedValue(42);
    await expect(tauriInvoke("check_llm_status")).rejects.toThrow("42");
  });

  it("calls timing callback on success", async () => {
    const timing = vi.fn();
    onIpcTiming(timing);
    mockInvoke.mockResolvedValue("ok");

    await tauriInvoke("test_cmd");

    expect(timing).toHaveBeenCalledWith("test_cmd", expect.any(Number), true);
    onIpcTiming(() => {}); // cleanup
  });

  it("calls timing callback on error", async () => {
    const timing = vi.fn();
    onIpcTiming(timing);
    mockInvoke.mockRejectedValue("fail");

    await expect(tauriInvoke("test_cmd")).rejects.toThrow();

    expect(timing).toHaveBeenCalledWith("test_cmd", expect.any(Number), false);
    onIpcTiming(() => {}); // cleanup
  });
});
