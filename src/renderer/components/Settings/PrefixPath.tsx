import React, { useContext, useState } from "react";
import styles from "./PrefixPath.module.scss";
import { ConfigContext } from "../../config";
import { IPCSync } from "../../../shared/ipc/event";

function PrefixPath(): JSX.Element {
  const configCxt = useContext(ConfigContext);
  const { path, setPath, error } = useValidatedPath(
    configCxt?.config.prefixPath || ""
  );

  function unfocused(event: React.FocusEvent<HTMLInputElement>) {
    const value = event.target.value;

    if (value.length === 0) return;

    setPath(value);

    if (error) return;

    configCxt?.setValue("prefixPath", value);
  }

  const classes = styles.input + (error ? " error" : "");

  return (
    <React.Fragment>
      <label htmlFor="prefix-path">Prefix Path</label>
      <input
        name="prefix-path"
        className={classes}
        title={error}
        type="text"
        value={path}
        onChange={(ev) => setPath(ev.target.value)}
        onBlur={unfocused}
      />
    </React.Fragment>
  );
}

interface ValidatedPath {
  path: string;
  setPath(path: string): void;
  error: string | undefined;
}

function useValidatedPath(initial: string): ValidatedPath {
  const [path, setPath] = useState(initial);
  const [error, setError] = useState<string | undefined>(undefined);

  function set(path: string) {
    const qualified = window.ipc.sendSync(IPCSync.NormalizePath, path);
    const newError = window.ipc.sendSync(IPCSync.FileExists, qualified)
      ? undefined
      : "Path does not exist";

    setPath(path);
    setError(newError);
  }

  return {
    path,
    setPath: set,
    error,
  };
}

export default PrefixPath;
