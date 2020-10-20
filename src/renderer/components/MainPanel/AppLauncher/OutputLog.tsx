import { IpcRendererEvent } from "electron";
import React, { useEffect, useMemo, useState } from "react";
import {
  AppEvent,
  maxAppEvents,
  SelectedApp,
} from "../../../../shared/ipc/application";
import { IPC, IPCSync } from "../../../../shared/ipc/event";
import styles from "./OutputLog.module.scss";

interface OutputLogProps {
  events: OutputEvent[];
}

export function OutputLog(props: OutputLogProps): JSX.Element {
  const renderedLines = useMemo(
    () =>
      props.events.map((line) => {
        return <span className={`line ${line.classes}`}>{line.message}</span>;
      }),
    [props.events]
  );

  return (
    <div className={styles.panel}>
      <div className={styles.wrapper}>{renderedLines}</div>
    </div>
  );
}

const enum OutputClasses {
  Data = "data",
  Launch = "special launch",
  Closed = "special closed",
}

interface OutputEvent {
  classes: OutputClasses;
  message: string;
}

export function useAppOutput(app?: SelectedApp): OutputEvent[] {
  const [lines, setLines] = useState<OutputEvent[]>([]);

  // This effect loads existing app events from the main process and starts monitoring for new ones
  useEffect(() => {
    if (!app) return;

    const events: AppEvent[] = window.ipc.sendSync(
      IPCSync.GetAppEvents,
      app.path.absolute
    );

    const outputEvents = (events || [])
      .map(appEventToOutput)
      .filter((e) => e !== null) as OutputEvent[];

    setLines(outputEvents);

    window.ipc.on(IPC.AppEvent, onAppEvent);

    return () => {
      window.ipc.removeListener(IPC.AppEvent, onAppEvent);
    };
  }, [app]);

  function onAppEvent(_: IpcRendererEvent, absPath: string, event: AppEvent) {
    if (!app || absPath !== app.path.absolute) return;

    const output = appEventToOutput(event);
    if (!output) return;

    appendLine(output);
  }

  function appEventToOutput(event: AppEvent): OutputEvent | null {
    switch (event.kind) {
      case "out":
        return { classes: OutputClasses.Data, message: event.data };
      case "launch":
        return null;
      case "close":
        return {
          classes: OutputClasses.Closed,
          message: "--- Process Closed ---",
        };
    }
  }

  function appendLine(line: OutputEvent) {
    setLines((cur) => {
      const slice = cur.length > maxAppEvents ? cur.slice(1) : cur;
      return [...slice, line];
    });
  }

  return lines;
}

export default OutputLog;
