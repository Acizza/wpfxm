import { faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import styles from "./Header.module.scss";

function Header(): JSX.Element {
  return (
    <div className={styles.header}>
      <span>wpfxm</span>
      <FontAwesomeIcon className={styles.settingsIcon} icon={faCog} />
    </div>
  );
}

export default Header;
