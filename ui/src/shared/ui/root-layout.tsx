import { Outlet, Link, useRouterState } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import {
  LayoutDashboard,
  Play,
  FileText,
  TestTube,
  GitCompare,
  Settings,
} from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { ToastContainer } from "@/shared/ui/toast";
import { ErrorBoundary } from "@/shared/ui/error-boundary";

const navItems = [
  { path: "/", icon: LayoutDashboard, labelKey: "nav.dashboard" },
  { path: "/pipeline", icon: Play, labelKey: "nav.pipeline" },
  { path: "/specification", icon: FileText, labelKey: "nav.specification" },
  { path: "/gherkin", icon: TestTube, labelKey: "nav.gherkin" },
  { path: "/traceability", icon: GitCompare, labelKey: "nav.traceability" },
  { path: "/settings", icon: Settings, labelKey: "nav.settings" },
] as const;

export function RootLayout() {
  const { t } = useTranslation();
  const routerState = useRouterState();
  const currentPath = routerState.location.pathname;

  return (
    <div className="bg-background text-foreground flex h-screen overflow-hidden">
      {/* Sidebar */}
      <nav className="bg-mantle border-surface-1 flex w-56 flex-col border-r">
        <div className="border-surface-1 flex h-14 items-center gap-2 border-b px-4">
          <div className="bg-blue flex h-8 w-8 items-center justify-center rounded-lg">
            <FileText className="text-crust h-4 w-4" />
          </div>
          <span className="text-text text-sm font-bold">Spec Forge</span>
        </div>
        <div className="flex-1 space-y-1 p-2">
          {navItems.map((item) => {
            const isActive = currentPath === item.path;
            const Icon = item.icon;
            return (
              <Link
                key={item.path}
                to={item.path}
                className={cn(
                  "flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors",
                  isActive
                    ? "bg-surface-0 text-blue font-medium"
                    : "text-subtext-0 hover:bg-surface-0 hover:text-text",
                )}
              >
                <Icon className="h-4 w-4" />
                {t(item.labelKey)}
              </Link>
            );
          })}
        </div>
      </nav>

      {/* Main content */}
      <main className="min-h-0 flex-1 overflow-y-auto p-6">
        <ErrorBoundary>
          <Outlet />
        </ErrorBoundary>
      </main>

      <ToastContainer />
    </div>
  );
}
