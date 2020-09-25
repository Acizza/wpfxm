import React, { useState } from "react";
import Prefix from "../../core/prefix/prefix";
import styles from "./PrefixList.module.scss";

interface PrefixListProps {
  prefixes: Prefix[];
}

function PrefixList(props: PrefixListProps): JSX.Element {
  const [selected, setSelected] = useState<number | undefined>();

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
