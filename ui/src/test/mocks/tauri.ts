import { vi } from "vitest";

export const mockInvoke = vi.fn();
export const mockListen = vi.fn().mockResolvedValue(vi.fn());

vi.mock("@tauri-apps/api/core", () => ({
  invoke: mockInvoke,
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: mockListen,
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

vi.mock("@tanstack/react-router", async () => {
  const React = await vi.importActual<typeof import("react")>("react");
  return {
    Link: (props: Record<string, unknown>) =>
      React.createElement(
        "a",
        { href: props.to as string, className: props.className as string },
        props.children as React.ReactNode,
      ),
    useNavigate: vi.fn().mockReturnValue(vi.fn()),
    useRouter: vi.fn().mockReturnValue({}),
    useMatch: vi.fn(),
    useLocation: vi.fn().mockReturnValue({ pathname: "/" }),
  };
});

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: vi.fn().mockReturnValue({
    isMaximized: vi.fn().mockResolvedValue(false),
    minimize: vi.fn(),
    toggleMaximize: vi.fn(),
    hide: vi.fn(),
    startDragging: vi.fn(),
    onResized: vi.fn().mockResolvedValue(vi.fn()),
  }),
}));
