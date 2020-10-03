import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useState } from "react";
import { IPrefix } from "../../../shared/ipc/prefix";
import styles from "./PrefixList.module.scss";

interface PrefixListProps {
  prefixes: IPrefix[];
  loading: boolean;
  onPrefixSelected?: (pfx: IPrefix, selected: boolean) => void;
}

function PrefixList(props: PrefixListProps): JSX.Element {
  const [selected, setSelected] = useSelection();

  function prefixClicked(pfx: IPrefix, index: number) {
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

  return <ul className={styles.panel}>{content}</ul>;
}

type SetSelected = (value?: number) => boolean;

function useSelection(initial?: number): [number | undefined, SetSelected] {
  const [state, setState] = useState(initial);

  function set(value?: number): boolean {
    const newValue = value === state ? undefined : value;
    setState(newValue);

    return newValue !== undefined;
  }

  return [state, set];
}

interface PrefixEntryProps {
  prefix: IPrefix;
  selected: boolean;
  onClick?: (event: React.MouseEvent) => void;
}

function PrefixEntry(props: PrefixEntryProps): JSX.Element {
  let classes = styles.prefix;

  if (props.selected) classes += ` ${styles.prefixSelected}`;

  return (
    <li className={classes} onClick={props.onClick}>
      {props.prefix.name}
    </li>
  );
}

export default PrefixList;
