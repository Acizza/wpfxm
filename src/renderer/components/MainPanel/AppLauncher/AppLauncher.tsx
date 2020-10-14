import React, { useMemo, useRef } from "react";
import { LaunchOptions, SelectedApp } from "../../../../shared/ipc/application";
import { IPC } from "../../../../shared/ipc/event";
import styles from "./AppLauncher.module.scss";
import Header from "./Header";
import OutputLog, { useOutputLogger } from "./OutputLog";

interface AppLauncherProps {
  app?: SelectedApp;
}

const maxHeightPcnt = 40;
const maxOutputLines = 500;

function AppLauncher(props: AppLauncherProps): JSX.Element {
  const [outputLines, setOutputPort] = useOutputLogger(maxOutputLines);
  const panelRef = useRef<HTMLDivElement>(null);

  const style = useMemo(() => {
    const elem = panelRef.current;
    let height: string | number = 0;

    if (!elem || !elem.parentElement) return { height };

    const parentHeight = elem.parentElement.scrollHeight;

    if (props.app && parentHeight > 0) {
      // We need to manually calculate the height of each child as simply using
      // the element's scrollHeight will not take child elements with scrollbars into account
      // properly
      const childrenHeight = Array.from(elem.children).reduce(
        (acc, child) => acc + child.scrollHeight,
        0
      );

      const totalHeight = childrenHeight;

      const pcnt = (totalHeight / parentHeight) * 100;
      const value = Math.min(pcnt, maxHeightPcnt);
      height = `${Math.round(value)}%`;
    } else {
      height = 0;
    }

    return { height };
  }, [props.app, outputLines]);

  function launchApp() {
    if (!props.app) return;

    const { port1, port2 } = new MessageChannel();

    const opts: LaunchOptions = {
      prefix: props.app.prefix,
      path: props.app.path.absolute,
      force32Bit: false,
    };

    window.ipc.postMessage(IPC.LaunchProcess, opts, [port1]);
    setOutputPort(port2);
  }

  return (
    <div className={styles.panel} ref={panelRef} style={style}>
      <Header onLaunchClicked={launchApp} />
      {outputLines.length > 0 && <OutputLog lines={outputLines} />}
    </div>
  );
}

export default AppLauncher;
