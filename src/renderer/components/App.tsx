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
import { AppEvent } from "../../shared/ipc/application";
import { IpcRendererEvent } from "electron";
import { useForceUpdate } from "../util";

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
  const [runningApps, pfxsWithRunningApps] = useRunningApps();

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
        <MainPanel
          selectedPrefix={selPrefix}
          runningApps={runningApps}
          onError={onError}
        />
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
          pfxsWithRunningApps={pfxsWithRunningApps}
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

type AbsolutePath = string;
export type RunningApps = Set<AbsolutePath>;
type PrefixesWithRunningApps = Set<string>;

/** Hook to capture running applications and the prefixes containing them.
 *
 * The prefixes are stored separately from the running application list to make data manipulation easier.
 */
function useRunningApps(): [RunningApps, PrefixesWithRunningApps] {
  // Since our state uses objects, we need to force updates whenever we change it
  const forceUpdate = useForceUpdate();
  const [runningApps, setRunningApps] = useState<RunningApps>(new Set());
  const [pfxsWithRunningApps, setPfxsWithRunningApps] = useState<
    PrefixesWithRunningApps
  >(new Set());

  useEffect(() => {
    window.ipc.on(IPC.AppEvent, onAppEvent);

    function onAppEvent(
      _: IpcRendererEvent,
      _absPath: string,
      event: AppEvent
    ) {
      switch (event.kind) {
        case "out":
          return;
        case "launch":
          setRunningApps((cur) => cur.add(event.app.path.absolute));
          setPfxsWithRunningApps((cur) => cur.add(event.app.prefix.name));
          forceUpdate();
          break;
        case "close":
          setRunningApps((cur) => {
            cur.delete(event.app.path.absolute);
            return cur;
          });
          setPfxsWithRunningApps((cur) => {
            cur.delete(event.app.prefix.name);
            return cur;
          });

          forceUpdate();
          break;
      }
    }

    return () => {
      window.ipc.removeListener(IPC.AppEvent, onAppEvent);
    };
  }, []);

  return [runningApps, pfxsWithRunningApps];
}

export default App;
