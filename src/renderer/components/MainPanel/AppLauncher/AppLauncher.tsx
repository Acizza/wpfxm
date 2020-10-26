import React, {
  CSSProperties,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { LaunchOptions, Application } from "../../../../shared/ipc/application";
import { IPC } from "../../../../shared/ipc/event";
import { RunningApps } from "../../App";
import styles from "./AppLauncher.module.scss";
import Header from "./Header";
import OutputLog, { useAppOutput } from "./OutputLog";

interface AppLauncherProps {
  app?: Application;
  runningApps: RunningApps;
}

const maxHeightPcnt = 40;

function AppLauncher(props: AppLauncherProps): JSX.Element {
  const [outputEvents, addOutputError] = useAppOutput(props.app);
  const [style, setStyle] = useState<CSSProperties>({});
  const [launchToggleDisabled, setLaunchToggleDisabled] = useState(false);
  const panelRef = useRef<HTMLDivElement>(null);

  // This effect calculates the height of the component.
  // useMemo cannot be used here as we need this to run after a render.
  useEffect(() => {
    setStyle(elementHeight(panelRef.current, props.app !== undefined));
  }, [props.app, outputEvents]);

  const isSelectedRunning = useMemo(() => {
    if (!props.app) return false;
    return props.runningApps.has(props.app.path.absolute);
  }, [props.app, props.runningApps.size]);

  function launchApp() {
    if (!props.app) return;

    setLaunchToggleDisabled(true);

    const opts: LaunchOptions = {
      app: props.app,
      force32Bit: false,
    };

    window.ipc
      .invoke(IPC.LaunchApp, opts)
      .catch((err) => addOutputError(`error launching application: ${err}`))
      .finally(() => {
        setLaunchToggleDisabled(false);
      });
  }

  function closeApp() {
    if (!props.app) return;

    setLaunchToggleDisabled(true);

    window.ipc
      .invoke(IPC.CloseApp, props.app.path.absolute)
      .then((success: boolean) => {
        if (!success) addOutputError("failed to close application");
      })
      .catch((err) => addOutputError(`error closing application: ${err}`))
      .finally(() => setLaunchToggleDisabled(false));
  }

  return (
    <div className={styles.panel} ref={panelRef} style={style}>
      <Header
        onLaunchClicked={launchApp}
        onCloseClicked={closeApp}
        showLaunchButton={!isSelectedRunning}
        disabled={launchToggleDisabled}
      />
      {outputEvents.length > 0 && <OutputLog events={outputEvents} />}
    </div>
  );
}

function elementHeight(
  elem: HTMLElement | null,
  canExpand: boolean
): CSSProperties {
  let height: string | number = 0;

  if (!canExpand || !elem || !elem.parentElement) return { height };

  const parentHeight = elem.parentElement.scrollHeight;

  if (parentHeight === 0) return { height };

  // We need to manually calculate the height of each child as simply using
  // the element's scrollHeight will not take child elements with scrollbars into account
  // properly
  const childrenHeight = Array.from(elem.children).reduce(
    (acc, child) => acc + child.scrollHeight,
    0
  );

  const pcnt = (childrenHeight / parentHeight) * 100;
  const value = Math.min(pcnt, maxHeightPcnt);
  height = `${Math.round(value)}%`;

  return { height };
}

export default AppLauncher;
