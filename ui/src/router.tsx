import { createRouter, createRoute, createRootRoute } from "@tanstack/react-router";
import { RootLayout } from "@/shared/ui/root-layout";
import { DashboardPage } from "@/pages/dashboard/ui/dashboard-page";
import { PipelinePage } from "@/pages/pipeline/ui/pipeline-page";
import { SpecificationPage } from "@/pages/specification/ui/specification-page";
import { GherkinPage } from "@/pages/gherkin/ui/gherkin-page";
import { TraceabilityPage } from "@/pages/traceability/ui/traceability-page";
import { SettingsPage } from "@/pages/settings/ui/settings-page";

const rootRoute = createRootRoute({
  component: RootLayout,
});

const dashboardRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: DashboardPage,
});

const pipelineRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/pipeline",
  component: PipelinePage,
});

const specificationRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/specification",
  component: SpecificationPage,
});

const gherkinRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/gherkin",
  component: GherkinPage,
});

const traceabilityRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/traceability",
  component: TraceabilityPage,
});

const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/settings",
  component: SettingsPage,
});

const routeTree = rootRoute.addChildren([
  dashboardRoute,
  pipelineRoute,
  specificationRoute,
  gherkinRoute,
  traceabilityRoute,
  settingsRoute,
]);

export const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
