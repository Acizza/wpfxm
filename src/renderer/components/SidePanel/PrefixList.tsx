import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import { IPrefix } from "../../../shared/ipc/prefix";
import GenericList from "../GenericList/GenericList";
import styles from "./PrefixList.module.scss";

interface PrefixListProps {
  prefixes: IPrefix[];
  loading: boolean;
  onPrefixSelected?: (pfx: IPrefix, selected: boolean) => void;
}

function PrefixList(props: PrefixListProps): JSX.Element {
  const displayLoading = props.loading || props.prefixes.length === 0;

  const content = displayLoading ? (
    <Loading />
  ) : (
    <GenericList
      items={props.prefixes}
      display={(pfx) => pfx.name}
      onItemSelected={props.onPrefixSelected}
    />
  );

  return <div className={styles.panel}>{content}</div>;
}

function Loading(): JSX.Element {
  return (
    <div className={styles.loadingWrapper}>
      <FontAwesomeIcon className={styles.loadIcon} icon={faCircleNotch} spin />
    </div>
  );
}

export default PrefixList;
