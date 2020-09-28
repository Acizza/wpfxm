import React from "react";
import Prefix from "../../core/prefix/prefix";
import Header from "./Header";
import PrefixList from "./PrefixList";
import styles from "./SidePanel.module.scss";

interface SidePanelProps {
  prefixes: Prefix[];
}

function SidePanel(props: SidePanelProps): JSX.Element {
  return (
    <div className={styles.sidePanel}>
      <Header />
      <PrefixList prefixes={props.prefixes} />
    </div>
  );
}

export default SidePanel;
