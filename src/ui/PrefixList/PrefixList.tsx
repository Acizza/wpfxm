import React, { useState } from "react";
import styles from "./PrefixList.module.scss";

function PrefixList(): JSX.Element {
  const [selected, setSelected] = useState<number | undefined>();

  return (
    <div className={styles.panel}>
      {[...Array(10)].map((_, i) => (
        <Prefix
          key={i}
          index={i}
          selected={selected}
          onClick={() => setSelected(i)}
        />
      ))}
    </div>
  );
}

interface PrefixProps {
  index: number;
  selected: number | undefined;
  onClick?: (event: React.MouseEvent) => void;
}

function Prefix(props: PrefixProps): JSX.Element {
  const selectedClass =
    props.selected === props.index ? styles.prefixSelected : "";

  const classes = `${styles.prefix} ${selectedClass}`;

  return (
    <span className={classes} onClick={props.onClick}>
      Prefix {props.index}
    </span>
  );
}

export default PrefixList;
