import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Outlet, Link, useMatchRoute } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  LayoutDashboard,
  Play,
  FileText,
  TestTube,
  GitCompare,
  Settings,
  Moon,
  Sun,
  Minus,
  Maximize2,
  Minimize2,
  X,
  type LucideIcon,
} from "lucide-react";
import { useThemeStore } from "@/shared/hooks/use-theme";
import { cn } from "@/shared/lib/utils";
import { ToastContainer } from "@/shared/ui/toast";
import { ErrorBoundary } from "@/shared/ui/error-boundary";
import { LanguageSwitcher } from "@/shared/ui/language-switcher";

interface NavTab {
  to: string;
  key: string;
  icon: LucideIcon;
}

const navTabs: NavTab[] = [
  { to: "/", key: "nav.dashboard", icon: LayoutDashboard },
  { to: "/pipeline", key: "nav.pipeline", icon: Play },
  { to: "/specification", key: "nav.specification", icon: FileText },
  { to: "/gherkin", key: "nav.gherkin", icon: TestTube },
  { to: "/traceability", key: "nav.traceability", icon: GitCompare },
  { to: "/settings", key: "nav.settings", icon: Settings },
];

export function RootLayout() {
  const { t } = useTranslation();
  const { theme, toggleTheme } = useThemeStore();
  const matchRoute = useMatchRoute();
  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    const appWindow = getCurrentWindow();
    appWindow.isMaximized().then(setIsMaximized);
    const unlisten = appWindow.onResized(() => {
      appWindow.isMaximized().then(setIsMaximized);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const handleDrag = useCallback(async (e: React.MouseEvent) => {
    if (
      (e.target as HTMLElement).closest(
        "button, a, input, select, textarea",
      )
    )
      return;
    await getCurrentWindow().startDragging();
  }, []);

  const handleDoubleClick = useCallback(async (e: React.MouseEvent) => {
    if (
      (e.target as HTMLElement).closest(
        "button, a, input, select, textarea",
      )
    )
      return;
    await getCurrentWindow().toggleMaximize();
  }, []);

  return (
    <div className="bg-background text-foreground flex h-screen flex-col overflow-hidden">
      <a
        href="#main-content"
        className="bg-blue text-crust fixed top-0 left-1/2 z-[200] -translate-x-1/2 -translate-y-full rounded-b-lg px-4 py-2 text-sm font-medium transition-transform focus:translate-y-0"
      >
        {t("header.skipToContent")}
      </a>

      {/* Header */}
      <header
        className="border-surface-1 bg-mantle shrink-0 border-b"
        onMouseDown={handleDrag}
        onDoubleClick={handleDoubleClick}
      >
        <div className="flex h-14 items-center justify-between gap-2 px-4 sm:h-16 sm:px-6">
          {/* Branding */}
          <div
            className="flex shrink-0 items-center gap-3"
            onMouseDown={(e) => e.stopPropagation()}
          >
            <div className="bg-blue/25 flex h-9 w-9 items-center justify-center rounded-lg">
              <FileText className="text-blue h-5 w-5" />
            </div>
            <div className="hidden sm:block">
              <h1 className="text-text text-base font-bold">Spec Forge</h1>
              <p className="text-subtext-0 text-xs">{t("header.subtitle")}</p>
            </div>
          </div>

          {/* Navigation Tabs */}
          <nav
            className="bg-crust flex items-center gap-1 rounded-xl p-1"
            onMouseDown={(e) => e.stopPropagation()}
          >
            {navTabs.map((tab) => {
              const isActive =
                tab.to === "/"
                  ? matchRoute({ to: "/", fuzzy: false })
                  : matchRoute({ to: tab.to, fuzzy: true });

              return (
                <Link
                  key={tab.to}
                  to={tab.to}
                  className={cn(
                    "flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200 sm:px-4 sm:py-2.5",
                    isActive
                      ? "bg-blue text-crust shadow-md"
                      : "text-subtext-1 hover:bg-surface-0 hover:text-text",
                  )}
                >
                  <tab.icon className="h-4 w-4" />
                  <span className="hidden lg:inline">{t(tab.key)}</span>
                </Link>
              );
            })}
          </nav>

          {/* Actions + Window Controls */}
          <div
            className="flex items-center gap-1"
            onMouseDown={(e) => e.stopPropagation()}
          >
            <LanguageSwitcher />
            <button
              onClick={toggleTheme}
              className="text-subtext-0 hover:bg-surface-0 hover:text-text flex h-9 w-9 items-center justify-center rounded-lg transition-colors"
              aria-label={t("header.toggleTheme")}
            >
              {theme === "dark" ? (
                <Sun className="h-4 w-4" />
              ) : (
                <Moon className="h-4 w-4" />
              )}
            </button>

            {/* Divider */}
            <div className="bg-surface-1 mx-1 h-5 w-px" />

            {/* Window Controls */}
            <button
              onClick={() => getCurrentWindow().minimize()}
              className="text-subtext-0 hover:bg-surface-0 hover:text-text flex h-8 w-8 items-center justify-center rounded-md transition-colors"
              aria-label={t("header.minimize")}
            >
              <Minus className="h-3.5 w-3.5" />
            </button>
            <button
              onClick={() => getCurrentWindow().toggleMaximize()}
              className="text-subtext-0 hover:bg-surface-0 hover:text-text flex h-8 w-8 items-center justify-center rounded-md transition-colors"
              aria-label={
                isMaximized ? t("header.restore") : t("header.maximize")
              }
            >
              {isMaximized ? (
                <Minimize2 className="h-3.5 w-3.5" />
              ) : (
                <Maximize2 className="h-3.5 w-3.5" />
              )}
            </button>
            <button
              onClick={() => getCurrentWindow().hide()}
              className="text-subtext-0 hover:bg-red/20 hover:text-red flex h-8 w-8 items-center justify-center rounded-md transition-colors"
              aria-label={t("header.close")}
            >
              <X className="h-3.5 w-3.5" />
            </button>
          </div>
        </div>
      </header>

      {/* Main content */}
      <main
        id="main-content"
        className="min-h-0 flex-1 overflow-x-hidden overflow-y-auto p-4 sm:p-6"
      >
        <ErrorBoundary>
          <Outlet />
        </ErrorBoundary>
      </main>

      <ToastContainer />
    </div>
  );
}
