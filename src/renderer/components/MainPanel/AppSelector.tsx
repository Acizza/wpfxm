import React, { useEffect, useState } from "react";
import { ApplicationPath, Application } from "../../../shared/ipc/application";
import { IPrefix } from "../../../shared/ipc/prefix";
import { RunningApps } from "../App";
import GenericList from "../GenericList/GenericList";
import AppLauncher from "./AppLauncher/AppLauncher";
import styles from "./AppSelector.module.scss";

interface AppSelectorProps {
  apps: ApplicationPath[];
  runningApps: RunningApps;
  selectedPrefix: IPrefix;
}

function AppSelector(props: AppSelectorProps): JSX.Element {
  const [selApp, setSelApp] = useState<Application | undefined>(undefined);

  function onAppSelected(item: ApplicationPath, selected: boolean) {
    if (!selected) {
      setSelApp(undefined);
      return;
    }

    const sel = {
      prefix: props.selectedPrefix,
      path: item,
    };

    setSelApp(sel);
  }

  useEffect(() => {
    setSelApp(undefined);
  }, [props.selectedPrefix]);

  return (
    <React.Fragment>
      <div className={styles.appList}>
        <GenericList
          items={props.apps}
          display={(p) => p.stripped}
          highlight={(p) => props.runningApps.has(p.absolute)}
          onItemSelected={onAppSelected}
        />
      </div>
      <AppLauncher app={selApp} />
    </React.Fragment>
  );
}

export default AppSelector;
