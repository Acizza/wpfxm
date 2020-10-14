import React, { useEffect, useState } from "react";
import { EventKind } from "../../../../shared/ipc/application";
import styles from "./OutputLog.module.scss";

interface OutputLogProps {
  lines: Line[];
}

export function OutputLog(props: OutputLogProps): JSX.Element {
  const renderedLines = props.lines.map((line) => (
    <span className={`line ${line.kind}`}>{line.data}</span>
  ));

  return (
    <div className={styles.panel}>
      <div className={styles.wrapper}>{renderedLines}</div>
    </div>
  );
}

type OnMessage = (event: MessageEvent<EventKind>) => void;
type SetPort = (port: MessagePort | undefined) => void;

function useMessagePort(
  onMessage: OnMessage
): [MessagePort | undefined, SetPort] {
  const [port, setPort] = useState<MessagePort | undefined>(undefined);

  useEffect(() => {
    if (!port) return;

    port.onmessage = onMessage;

    return () => {
      port.onmessage = null;
      port.close();
    };
  }, [port]);

  return [port, setPort];
}

const enum LineKind {
  Output = "output",
  Closed = "closed",
}

interface Line {
  kind: LineKind;
  data: string;
}

export function useOutputLogger(maxLines: number): [Line[], SetPort] {
  const [lines, setLines] = useState<Line[]>([]);
  const [, setPort] = useMessagePort(onOutput);

  function appendLine(line: Line) {
    setLines((cur) => {
      const slice = cur.length > maxLines ? cur.slice(1) : cur;
      return [...slice, line];
    });
  }

  function onOutput(event: MessageEvent<EventKind>) {
    const data = event.data;

    switch (data.kind) {
      case "data":
        appendLine({ kind: LineKind.Output, data: data.data });
        break;
      case "closed":
        appendLine({ kind: LineKind.Closed, data: "--- Process Closed ---" });
        break;
    }
  }

  return [lines, setPort];
}

export default OutputLog;
