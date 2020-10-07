import React from "react";
import { ApplicationPath } from "../../../shared/ipc/application";
import styles from "./AppLauncher.module.scss";

interface AppLauncherProps {
  app?: ApplicationPath;
}

function AppLauncher(props: AppLauncherProps): JSX.Element {
  let classes = styles.panel;

  if (!props.app) classes += ` ${styles.hidden}`;

  return (
    <div className={classes}>
      <span>Launcher</span>
    </div>
  );
}

export default AppLauncher;
