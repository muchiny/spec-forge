import { describe, it, expect, beforeEach } from "vitest";
import { useToastStore, toast } from "../toast-store";

describe("useToastStore", () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it("starts with empty toasts", () => {
    expect(useToastStore.getState().toasts).toEqual([]);
  });

  it("add creates a toast with generated id", () => {
    useToastStore.getState().add("Hello", "success");
    const toasts = useToastStore.getState().toasts;
    expect(toasts).toHaveLength(1);
    expect(toasts[0]?.message).toBe("Hello");
    expect(toasts[0]?.variant).toBe("success");
    expect(toasts[0]?.id).toMatch(/^toast-\d+$/);
  });

  it("add defaults to info variant and 4000ms duration", () => {
    useToastStore.getState().add("Info msg");
    const t = useToastStore.getState().toasts[0];
    expect(t?.variant).toBe("info");
    expect(t?.duration).toBe(4000);
  });

  it("add respects custom duration", () => {
    useToastStore.getState().add("Custom", "warning", 8000);
    expect(useToastStore.getState().toasts[0]?.duration).toBe(8000);
  });

  it("dismiss removes the toast by id", () => {
    useToastStore.getState().add("A", "info");
    useToastStore.getState().add("B", "info");
    const id = useToastStore.getState().toasts[0]?.id;
    if (id) useToastStore.getState().dismiss(id);
    const toasts = useToastStore.getState().toasts;
    expect(toasts).toHaveLength(1);
    expect(toasts[0]?.message).toBe("B");
  });

  it("multiple toasts accumulate", () => {
    useToastStore.getState().add("First");
    useToastStore.getState().add("Second");
    useToastStore.getState().add("Third");
    expect(useToastStore.getState().toasts).toHaveLength(3);
  });
});

describe("toast convenience methods", () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it("toast.success creates success variant", () => {
    toast.success("Done!");
    const t = useToastStore.getState().toasts[0];
    expect(t?.variant).toBe("success");
    expect(t?.message).toBe("Done!");
  });

  it("toast.error creates error variant with 6000ms duration", () => {
    toast.error("Failed");
    const t = useToastStore.getState().toasts[0];
    expect(t?.variant).toBe("error");
    expect(t?.duration).toBe(6000);
  });

  it("toast.warning creates warning variant with 5000ms duration", () => {
    toast.warning("Watch out");
    const t = useToastStore.getState().toasts[0];
    expect(t?.variant).toBe("warning");
    expect(t?.duration).toBe(5000);
  });

  it("toast.info creates info variant", () => {
    toast.info("FYI");
    const t = useToastStore.getState().toasts[0];
    expect(t?.variant).toBe("info");
  });
});
