import { describe, it, expect, beforeEach } from "vitest";
import { usePreferencesStore } from "../use-preferences-store";

describe("usePreferencesStore", () => {
  beforeEach(() => {
    usePreferencesStore.setState({ specViewTab: 0 });
  });

  it("has default specViewTab 0", () => {
    expect(usePreferencesStore.getState().specViewTab).toBe(0);
  });

  it("setSpecViewTab updates the active tab", () => {
    usePreferencesStore.getState().setSpecViewTab(2);
    expect(usePreferencesStore.getState().specViewTab).toBe(2);
  });

  it("setSpecViewTab can switch back to 0", () => {
    usePreferencesStore.getState().setSpecViewTab(3);
    usePreferencesStore.getState().setSpecViewTab(0);
    expect(usePreferencesStore.getState().specViewTab).toBe(0);
  });
});
