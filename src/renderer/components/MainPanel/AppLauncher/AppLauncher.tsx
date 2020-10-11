import React, { CSSProperties, useEffect, useRef, useState } from "react";
import { ApplicationPath } from "../../../../shared/ipc/application";
import styles from "./AppLauncher.module.scss";
import Header from "./Header";

interface AppLauncherProps {
  app?: ApplicationPath;
}

const maxHeightPcnt = 40;

function AppLauncher(props: AppLauncherProps): JSX.Element {
  const [style, setStyle] = useState<CSSProperties>({ height: 0 });
  const panelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const elem = panelRef.current;

    if (!elem || !elem.parentElement) return;

    let height: string | number;

    if (props.app) {
      const pcnt = (elem.scrollHeight / elem.parentElement.scrollHeight) * 100;
      const value = Math.min(pcnt, maxHeightPcnt);
      height = `${Math.round(value)}%`;
    } else {
      height = 0;
    }

    setStyle({ height });
  }, [props.app]);

  return (
    <div className={styles.panel} ref={panelRef} style={style}>
      <Header />
    </div>
  );
}

export default AppLauncher;
