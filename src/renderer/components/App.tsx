import React, { useEffect, useState } from "react";
import "./App.scss";
import MainPanel from "./MainPanel/MainPanel";
import SidePanel from "./SidePanel/SidePanel";
import ErrorModal from "./ErrorModal";
import Settings from "./Settings/Settings";
import { ConfigContext, useGlobalConfig } from "../config";
import { IPC } from "../../shared/ipc/event";
import { IPrefix } from "../../shared/ipc/prefix";
import { DisplayError, ErrorClosure } from "../types/error";

const enum Panel {
  Settings,
  MainPanel,
}

const errors = {
  noPrefixes: {
    context: "No Prefixes Found",
    message:
      "You can change the path to look for prefixes in by clicking the settings button on the top left.",
  },
  prefixLoading: (message: string) => ({
    context: "Error Loading Prefixes",
    message,
  }),
};

function App() {
  const cfgState = useGlobalConfig();
  const [error, setError] = useState<DisplayError | undefined>(undefined);
  const [panel, togglePanel, resetPanel] = usePanelToggle(Panel.MainPanel);
  const [selPrefix, setSelPrefix] = useState<IPrefix | undefined>(undefined);
  const scannedPfxs = useScannedPrefixes({
    initialPath: cfgState.config.prefixPath,
    onError,
  });

  function onError(error: DisplayError | undefined) {
    setError(error);
  }

  function onPrefixSelected(pfx: IPrefix, selected: boolean) {
    if (selected) {
      resetPanel();
      setSelPrefix(pfx);
    } else {
      setSelPrefix(undefined);
    }
  }

  let renderedPanel: JSX.Element;

  switch (panel as Panel) {
    case Panel.MainPanel:
      renderedPanel = (
        <MainPanel selectedPrefix={selPrefix} onError={onError} />
      );
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
          onPrefixSelected={onPrefixSelected}
        />
        {renderedPanel}
        {error && <ErrorModal {...error} />}
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
}

interface ScannedPrefixesProps {
  initialPath?: string;
  onError: ErrorClosure;
}

function useScannedPrefixes(props: ScannedPrefixesProps): ScannedPrefixes {
  const [prefixes, setPrefixes] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  function set(path: string) {
    window.ipc
      .invoke(IPC.ScanPrefixes, path)
      .then((pfxs) => {
        setPrefixes(pfxs);

        const err = pfxs.length === 0 ? errors.noPrefixes : undefined;

        props.onError(err);
      })
      .catch((err: Error) => props.onError(errors.prefixLoading(err.message)))
      .finally(() => setLoading(false));
  }

  useEffect(() => {
    if (props.initialPath) set(props.initialPath);
  }, [props.initialPath]);

  return {
    prefixes,
    setPrefixes: set,
    loading,
  };
}

export default App;
