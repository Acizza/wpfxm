import React, { CSSProperties, useEffect, useRef, useState } from "react";
import { LaunchOptions, SelectedApp } from "../../../../shared/ipc/application";
import { IPC } from "../../../../shared/ipc/event";
import styles from "./AppLauncher.module.scss";
import Header from "./Header";
import OutputLog, { useAppOutput } from "./OutputLog";

interface AppLauncherProps {
  app?: SelectedApp;
}

const maxHeightPcnt = 40;

function AppLauncher(props: AppLauncherProps): JSX.Element {
  const outputEvents = useAppOutput(props.app);
  const [style, setStyle] = useState<CSSProperties>({});
  const panelRef = useRef<HTMLDivElement>(null);

  // This effect calculates the height of the component.
  // useMemo cannot be used here as we need this to run after a render.
  useEffect(() => {
    setStyle(elementHeight(panelRef.current, props.app !== undefined));
  }, [props.app, outputEvents]);

  function launchApp() {
    if (!props.app) return;

    const opts: LaunchOptions = {
      app: props.app,
      force32Bit: false,
    };

    window.ipc.postMessage(IPC.LaunchProcess, opts, []);
  }

  return (
    <div className={styles.panel} ref={panelRef} style={style}>
      <Header onLaunchClicked={launchApp} />
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
