import React, { useEffect, useState } from "react";
import "./App.scss";
import MainPanel from "./MainPanel/MainPanel";
import SidePanel from "./SidePanel/SidePanel";
import ErrorModal, { Error } from "./ErrorModal";
import Settings from "./Settings/Settings";
import { ConfigContext, useGlobalConfig } from "../config";
import { IPC } from "../../shared/ipc/event";

const enum Panel {
  Settings,
  MainPanel,
}

function App() {
  const cfgState = useGlobalConfig();
  const scannedPfxs = useScannedPrefixes(cfgState.config.prefixPath);
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
          prefixes={scannedPfxs.prefixes}
          loading={scannedPfxs.loading}
          onToggleSettings={togglePanel}
          onPrefixSelected={(_, selected) => selected && resetPanel()}
        />
        {renderedPanel}
        {scannedPfxs.error && <ErrorModal {...scannedPfxs.error} />}
      </ConfigContext.Provider>
    </main>
  );
}

type Toggle = () => void;
type Reset = () => void;

function usePanelToggle(initial: Panel): [Panel, Toggle, Reset] {
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

interface ScannedPrefixes {
  prefixes: any[];
  setPrefixes(path: string): void;
  loading: boolean;
  error: Error | undefined;
}

function useScannedPrefixes(initialPath?: string): ScannedPrefixes {
  const [prefixes, setPrefixes] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | undefined>(undefined);

  function set(path: string) {
    window.ipc
      .invoke(IPC.ScanPrefixes, path)
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

  return {
    prefixes,
    setPrefixes: set,
    loading,
    error,
  };
}

export default App;
