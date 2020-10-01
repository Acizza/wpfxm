import React, { ChangeEvent, useContext, useState } from "react";
import styles from "./PrefixPath.module.scss";
import * as fs from "fs";
import { normalizeUnixPath } from "../../../main/util";
import { ConfigContext } from "../../config";

function PrefixPath(): JSX.Element {
  const configCxt = useContext(ConfigContext);
  const [value, setValue] = useState(configCxt?.config.prefixPath || "");
  const [error, setError] = useState<string | undefined>(undefined);

  function changed(event: ChangeEvent<HTMLInputElement>) {
    const newValue = event.target.value;
    const path = normalizeUnixPath(newValue);
    const newError = fs.existsSync(path) ? undefined : "Path does not exist";

    setValue(newValue);
    setError(newError);
  }

  function unfocused(event: React.FocusEvent<HTMLInputElement>) {
    if (error || event.target.value.length === 0) return;
    configCxt?.setValue("prefixPath", event.target.value);
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
        value={value}
        onChange={changed}
        onBlur={unfocused}
      />
    </React.Fragment>
  );
}

export default PrefixPath;
