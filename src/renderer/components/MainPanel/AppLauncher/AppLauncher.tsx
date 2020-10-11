import React, { useMemo, useRef } from "react";
import { ApplicationPath } from "../../../../shared/ipc/application";
import styles from "./AppLauncher.module.scss";
import Header from "./Header";

interface AppLauncherProps {
  app?: ApplicationPath;
}

const maxHeightPcnt = 40;

function AppLauncher(props: AppLauncherProps): JSX.Element {
  const panelRef = useRef<HTMLDivElement>(null);

  const style = useMemo(() => {
    const elem = panelRef.current;
    let height: string | number = 0;

    if (!elem || !elem.parentElement) return { height };

    const parentHeight = elem.parentElement.scrollHeight;

    if (props.app && parentHeight > 0) {
      const pcnt = (elem.scrollHeight / parentHeight) * 100;
      const value = Math.min(pcnt, maxHeightPcnt);
      height = `${Math.round(value)}%`;
    } else {
      height = 0;
    }

    return { height };
  }, [props.app]);

  return (
    <div className={styles.panel} ref={panelRef} style={style}>
      <Header />
    </div>
  );
}

export default AppLauncher;
