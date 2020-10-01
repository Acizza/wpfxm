import React, { useEffect, useState } from "react";
import Prefix from "../core/prefix/prefix";
import "./App.scss";
import MainPanel from "./MainPanel/MainPanel";
import SidePanel from "./SidePanel/SidePanel";
import ErrorModal, { Error } from "./ErrorModal";
import Settings from "./Settings/Settings";
import { ConfigContext, useGlobalConfig } from "../shared/config";

enum Panel {
  Settings,
  MainPanel,
}

function App() {
  const cfgState = useGlobalConfig();
  const [prefixes, , loading, error] = useScannedPrefixes(
    cfgState.config.prefixPath
  );
  const [panel, togglePanel, resetPanel] = usePanelToggle(Panel.MainPanel);

  let renderedPanel: JSX.Element;

  switch (panel as Panel) {
    case Panel.MainPanel:
      renderedPanel = <MainPanel />;
      break;
    case Panel.Settings:
      renderedPanel = <Settings />;
      break;
  }

  return (
    <main>
      <ConfigContext.Provider value={cfgState}>
        <SidePanel
          prefixes={prefixes}
          loading={loading}
          onToggleSettings={togglePanel}
          onPrefixSelected={(_, selected) => selected && resetPanel()}
        />
        {renderedPanel}
        {error && <ErrorModal {...error} />}
      </ConfigContext.Provider>
    </main>
  );
}

// TODO: The return type must be any[] because of this bug:
// https://github.com/microsoft/TypeScript/issues/36390
function usePanelToggle(initial: Panel): any[] {
  const [panel, setPanel] = useState(initial);

  function toggle() {
    switch (panel) {
      case Panel.MainPanel:
        setPanel(Panel.Settings);
        break;
      case Panel.Settings:
        setPanel(Panel.MainPanel);
        break;
    }
  }

  function reset() {
    setPanel(initial);
  }

  return [panel, toggle, reset];
}

// TODO: The return type must be any[] because of this bug:
// https://github.com/microsoft/TypeScript/issues/36390
function useScannedPrefixes(initialPath?: string): any[] {
  const [prefixes, setPrefixes] = useState<Prefix[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | undefined>(undefined);

  function set(path: string) {
    Prefix.allFromDir(path)
      .then((pfxs) => {
        setPrefixes(pfxs);
        setError(undefined);
      })
      .catch((err) =>
        setError({
          context: "Error Loading Prefixes",
          message: err.message,
        })
      )
      .finally(() => setLoading(false));
  }

  useEffect(() => {
    if (initialPath) set(initialPath);
  }, [initialPath]);

  return [prefixes, set, loading, error];
}

export default App;
