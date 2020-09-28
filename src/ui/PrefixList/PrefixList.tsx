import React, { useState } from "react";
import Prefix from "../../core/prefix/prefix";
import styles from "./PrefixList.module.scss";

interface PrefixListProps {
  prefixes: Prefix[];
}

function PrefixList(props: PrefixListProps): JSX.Element {
  const [selected, setSelected] = useSelection();

  return (
    <div className={styles.panel}>
      {props.prefixes.map((pfx, i) => (
        <PrefixEntry
          key={i}
          prefix={pfx}
          selected={i === selected}
          onClick={() => setSelected(i)}
        />
      ))}
    </div>
  );
}

// TODO: The return type must be any[] because of this bug:
// https://github.com/microsoft/TypeScript/issues/36390
function useSelection(initial?: number): any[] {
  const [state, setState] = useState(initial);

  function set(value?: number) {
    setState(value === state ? undefined : value);
  }

  return [state, set];
}

interface PrefixEntryProps {
  prefix: Prefix;
  selected: boolean;
  onClick?: (event: React.MouseEvent) => void;
}

function PrefixEntry(props: PrefixEntryProps): JSX.Element {
  let classes = styles.prefix;

  if (props.selected) classes += ` ${styles.prefixSelected}`;

  return (
    <span className={classes} onClick={props.onClick}>
      {props.prefix.name}
    </span>
  );
}

export default PrefixList;
