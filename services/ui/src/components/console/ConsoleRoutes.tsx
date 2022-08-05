import { lazy } from "solid-js";
import { Route, Navigate } from "solid-app-router";
import { Operation, Resource } from "./config/types";

import consoleConfig from "./config/console";

const ConsolePage = lazy(() => import("./ConsolePage"));

const ConsoleRoutes = (props) => {
  const config = consoleConfig;

  const consolePage = (config) => {
    return (
      <ConsolePage
        user={props.user}
        config={config}
        pathname={props.pathname}
        handleTitle={props.handleTitle}
        handleRedirect={props.handleRedirect}
      />
    );
  };

  return (
    <>
      {/* Console Routes */}
      <Route path="/" element={<Navigate href={"/console/projects"} />} />
      {/* Console Projects Routes */}
      <Route
        path="/projects"
        element={consolePage(config?.[Resource.PROJECTS]?.[Operation.LIST])}
      />
      <Route
        path="/projects/add"
        element={consolePage(config?.[Resource.PROJECTS]?.[Operation.ADD])}
      />
      <Route
        path="/projects/:project_slug"
        element={consolePage(config?.[Resource.PROJECTS]?.[Operation.VIEW])}
      />
      <Route
        path="/projects/:project_slug/perf"
        element={consolePage(config?.[Resource.PROJECTS]?.[Operation.PERF])}
      />
      <Route
        path="/projects/:project_slug/reports"
        element={consolePage(config?.[Resource.REPORTS]?.[Operation.LIST])}
      />
      <Route
        path="/projects/:project_slug/reports/add"
        element={consolePage(config?.[Resource.REPORTS]?.[Operation.ADD])}
      />
      <Route
        path="/projects/:project_slug/reports/:reports"
        element={consolePage(config?.[Resource.REPORTS]?.[Operation.VIEW])}
      />
      <Route
        path="/projects/:project_slug/branches"
        element={consolePage(config?.[Resource.BRANCHES]?.[Operation.LIST])}
      />
      <Route
        path="/projects/:project_slug/branches/add"
        element={consolePage(config?.[Resource.BRANCHES]?.[Operation.ADD])}
      />
      <Route
        path="/projects/:project_slug/branches/:branch_slug"
        element={consolePage(config?.[Resource.BRANCHES]?.[Operation.VIEW])}
      />
      <Route
        path="/projects/:project_slug/testbeds"
        element={consolePage(config?.[Resource.TESTBEDS]?.[Operation.LIST])}
      />
      <Route
        path="/projects/:project_slug/testbeds/add"
        element={consolePage(config?.[Resource.TESTBEDS]?.[Operation.ADD])}
      />
      <Route
        path="/projects/:project_slug/testbeds/:testbed_slug"
        element={consolePage(config?.[Resource.TESTBEDS]?.[Operation.VIEW])}
      />
      <Route
        path="/projects/:project_slug/connections"
        element={consolePage(config?.[Resource.CONNECTIONS]?.[Operation.LIST])}
      />
      <Route
        path="/projects/:project_slug/connections/:connection_uuid"
        element={consolePage(config?.[Resource.CONNECTIONS]?.[Operation.VIEW])}
      />
      <Route
        path="/projects/:project_slug/settings"
        element={consolePage(
          config?.[Resource.PROJECT_SETTINGS]?.[Operation.VIEW]
        )}
      />
      <Route
        path="/user/account"
        element={consolePage(config?.[Resource.USER_ACCOUNT]?.[Operation.VIEW])}
      />
      <Route
        path="/user/settings"
        element={consolePage(
          config?.[Resource.USER_SETTINGS]?.[Operation.VIEW]
        )}
      />
    </>
  );
};

export default ConsoleRoutes;
