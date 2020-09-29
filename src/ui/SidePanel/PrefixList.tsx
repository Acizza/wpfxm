import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useState } from "react";
import Prefix from "../../core/prefix/prefix";
import styles from "./PrefixList.module.scss";

interface PrefixListProps {
  prefixes: Prefix[];
  loading: boolean;
  onPrefixSelected?: (pfx: Prefix, selected: boolean) => void;
}

function PrefixList(props: PrefixListProps): JSX.Element {
  const [selected, setSelected] = useSelection();

  function prefixClicked(pfx: Prefix, index: number) {
    const isSelected: boolean = setSelected(index);
    props.onPrefixSelected?.(pfx, isSelected);
  }

  const content = props.loading ? (
    <FontAwesomeIcon className={styles.loadIcon} icon={faCircleNotch} spin />
  ) : (
    props.prefixes.map((pfx, i) => (
      <PrefixEntry
        key={i}
        prefix={pfx}
        selected={i === selected}
        onClick={() => prefixClicked(pfx, i)}
      />
    ))
  );

  return <div className={styles.panel}>{content}</div>;
}

// TODO: The return type must be any[] because of this bug:
// https://github.com/microsoft/TypeScript/issues/36390
function useSelection(initial?: number): any[] {
  const [state, setState] = useState(initial);

  function set(value?: number): boolean {
    const newValue = value === state ? undefined : value;
    setState(newValue);

    return newValue !== undefined;
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
