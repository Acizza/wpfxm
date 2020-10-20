import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { IpcRendererEvent } from "electron";
import React, { useEffect, useState } from "react";
import { AppEvent } from "../../../shared/ipc/application";
import { IPC } from "../../../shared/ipc/event";
import { IPrefix } from "../../../shared/ipc/prefix";
import { useForceUpdate } from "../../util";
import GenericList from "../GenericList/GenericList";
import styles from "./PrefixList.module.scss";

interface PrefixListProps {
  prefixes: IPrefix[];
  loading: boolean;
  onPrefixSelected?(pfx: IPrefix, selected: boolean): void;
}

function PrefixList(props: PrefixListProps): JSX.Element {
  // We need to force updates after changing pfxsWithRunningApps, since we're using a Map
  const forceUpdate = useForceUpdate();
  const [runningApps, setRunningApps] = useState<Map<string, number>>(
    new Map()
  );

  useEffect(() => {
    window.ipc.on(IPC.AppEvent, onAppEvent);

    function onAppEvent(
      _: IpcRendererEvent,
      _absPath: string,
      event: AppEvent
    ) {
      switch (event.kind) {
        case "out":
          return;
        case "launch":
          setRunningApps((cur) => {
            const existing = cur.get(event.prefix.name);
            return cur.set(event.prefix.name, (existing || 0) + 1);
          });
          forceUpdate();
          break;
        case "close":
          setRunningApps((cur) => {
            const existing = cur.get(event.prefix.name);

            if (!existing || existing <= 1) {
              cur.delete(event.prefix.name);
            } else {
              cur.set(event.prefix.name, existing - 1);
            }

            return cur;
          });
          forceUpdate();
          break;
      }
    }

    return () => {
      window.ipc.removeListener(IPC.AppEvent, onAppEvent);
    };
  }, []);

  const displayLoading = props.loading || props.prefixes.length === 0;

  const content = displayLoading ? (
    <Loading />
  ) : (
    <GenericList
      items={props.prefixes}
      display={(pfx) => pfx.name}
      highlight={(p) => runningApps.has(p.name)}
      onItemSelected={props.onPrefixSelected}
    />
  );

  return <div className={styles.panel}>{content}</div>;
}

function Loading(): JSX.Element {
  return (
    <div className={styles.loadingWrapper}>
      <FontAwesomeIcon className={styles.loadIcon} icon={faCircleNotch} spin />
    </div>
  );
}

export default PrefixList;
