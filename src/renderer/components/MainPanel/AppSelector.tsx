import React, { useEffect, useState } from "react";
import { ApplicationPath, SelectedApp } from "../../../shared/ipc/application";
import { IPrefix } from "../../../shared/ipc/prefix";
import GenericList from "../GenericList/GenericList";
import AppLauncher from "./AppLauncher/AppLauncher";
import styles from "./AppSelector.module.scss";

interface AppSelectorProps {
  apps: ApplicationPath[];
  selectedPrefix: IPrefix;
}

function AppSelector(props: AppSelectorProps): JSX.Element {
  const [selApp, setSelApp] = useState<SelectedApp | undefined>(undefined);

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
          onItemSelected={onAppSelected}
        />
      </div>
      <AppLauncher app={selApp} />
    </React.Fragment>
  );
}

export default AppSelector;