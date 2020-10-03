import React from "react";
import { IPrefix } from "../../../shared/ipc/prefix";
import Header from "./Header";
import PrefixList from "./PrefixList";
import styles from "./SidePanel.module.scss";

interface SidePanelProps {
  prefixes: IPrefix[];
  loading: boolean;
  onToggleSettings?: () => void;
  onPrefixSelected?: (pfx: IPrefix, selected: boolean) => void;
}

function SidePanel(props: SidePanelProps): JSX.Element {
  return (
    <div className={styles.sidePanel}>
      <Header onToggleSettings={props.onToggleSettings} />
      <PrefixList
        prefixes={props.prefixes}
        loading={props.loading}
        onPrefixSelected={props.onPrefixSelected}
      />
    </div>
  );
}

export default SidePanel;
