import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useEffect, useMemo, useState } from "react";
import {
  ApplicationPath,
  FoundApplications,
} from "../../../shared/ipc/application";
import { IPC } from "../../../shared/ipc/event";
import { IPrefix } from "../../../shared/ipc/prefix";
import { ErrorClosure } from "../../types/error";
import GenericList from "../GenericList/GenericList";
import AppLauncher from "./AppLauncher/AppLauncher";
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
  const [apps, setApps] = useState<ApplicationPath[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!props.selectedPrefix) {
      setApps([]);
      return;
    }

    setLoading(true);

    window.ipc
      .invoke(IPC.ScanPrefixApps, props.selectedPrefix)
      .then((newApps: FoundApplications) => {
        setApps(newApps.paths);
        props.onError?.(undefined);
      })
      .catch((err: Error) => {
        props.onError?.(errors.scanPrefixApps(err.message));
      })
      .finally(() => setLoading(false));
  }, [props.selectedPrefix]);

  const content = useMemo(
    () => {
      if (loading) {
        return <Loading />;
      } else if (!props.selectedPrefix) {
        return <Message kind={MessageKind.SelectPrefix} />;
      } else if (apps.length === 0) {
        return <Message kind={MessageKind.NoApps} />;
      } else {
        return (
          <AppSelector apps={apps} selectedPrefix={props.selectedPrefix} />
        );
      }
    },
    // We don't want to update when our selectedPrefix changes, as it will cause the NoApps message
    // to show for one render once we start loading applications. The effect hook above will set
    // apps and trigger an update when the selected prefix changes.
    [loading, apps]
  );

  return <div className={styles.panel}>{content}</div>;
}

interface AppSelectorProps {
  apps: ApplicationPath[];
  selectedPrefix: IPrefix;
}

function AppSelector(props: AppSelectorProps): JSX.Element {
  const [selApp, setSelApp] = useState<ApplicationPath | undefined>(undefined);

  function onAppSelected(item: ApplicationPath, selected: boolean) {
    setSelApp(selected ? item : undefined);
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

const enum MessageKind {
  SelectPrefix = "Select a prefix",
  NoApps = "No applications found in selected prefix",
}

interface MessageProps {
  kind: MessageKind;
}

function Message(props: MessageProps): JSX.Element {
  return (
    <div className={styles.message}>
      <span>{props.kind}</span>
    </div>
  );
}

function Loading(): JSX.Element {
  return (
    <div className={styles.loading}>
      <FontAwesomeIcon
        className={styles.loadingIcon}
        icon={faCircleNotch}
        spin
      />
    </div>
  );
}

export default MainPanel;
