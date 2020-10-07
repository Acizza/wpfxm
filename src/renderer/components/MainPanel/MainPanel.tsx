import React, { useEffect, useState } from "react";
import IFoundApplications from "../../../shared/ipc/application";
import { IPC } from "../../../shared/ipc/event";
import { IPrefix } from "../../../shared/ipc/prefix";
import { ErrorClosure } from "../../types/error";
import GenericList from "../GenericList/GenericList";
import styles from "./MainPanel.module.scss";

const errors = {
  scanPrefixApps: (message: string) => ({
    context: "Error Scanning Prefix",
    message,
  }),
};

interface MainPanelProps {
  selectedPrefix?: IPrefix;
  onError?: ErrorClosure;
}

function MainPanel(props: MainPanelProps): JSX.Element {
  const [apps, setApps] = useState<string[]>([]);

  useEffect(() => {
    if (!props.selectedPrefix) {
      setApps([]);
      return;
    }

    window.ipc
      .invoke(IPC.ScanPrefixApps, props.selectedPrefix)
      .then((newApps: IFoundApplications) => {
        setApps(newApps.strippedPaths);
        props.onError?.(undefined);
      })
      .catch((err: Error) => {
        props.onError?.(errors.scanPrefixApps(err.message));
      });
  }, [props.selectedPrefix]);

  return (
    <div className={styles.panel}>
      <GenericList items={apps} />
    </div>
  );
}

export default MainPanel;
