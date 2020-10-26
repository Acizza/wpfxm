import { IpcRendererEvent } from "electron";
import React, { useEffect, useMemo, useState } from "react";
import {
  AppEvent,
  maxAppEvents,
  Application,
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
        const classes =
          line.kind !== OutputKind.Data ? `special ${line.kind}` : line.kind;

        return <span className={`line ${classes}`}>{line.message}</span>;
      }),
    [props.events]
  );

  return (
    <div className={styles.panel}>
      <div className={styles.wrapper}>{renderedLines}</div>
    </div>
  );
}

const enum OutputKind {
  Data = "data",
  Launch = "launch",
  Closed = "closed",
  Error = "error",
}

interface OutputEvent {
  kind: OutputKind;
  message: string;
}

type AppendError = (message: string) => void;

export function useAppOutput(app?: Application): [OutputEvent[], AppendError] {
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

    appendEvent(output);
  }

  function appEventToOutput(event: AppEvent): OutputEvent | null {
    switch (event.kind) {
      case "out":
        return { kind: OutputKind.Data, message: event.data };
      case "launch":
        return null;
      case "close":
        return {
          kind: OutputKind.Closed,
          message: "--- Process Closed ---",
        };
    }
  }

  function appendEvent(line: OutputEvent) {
    setLines((cur) => {
      const slice = cur.length > maxAppEvents ? cur.slice(1) : cur;
      return [...slice, line];
    });
  }

  function appendError(message: string) {
    appendEvent({
      kind: OutputKind.Error,
      message: `--- Error: ${message} ---`,
    });
  }

  return [lines, appendError];
}

export default OutputLog;
