import { faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import styles from "./Header.module.scss";
// @ts-ignore: This import produces an error, but works perfectly when ignored...
import { name, version } from "../../../package.json";

function Header(): JSX.Element {
  return (
    <div className={styles.header}>
      <div>
        <span>{name}</span>
        <span className={styles.version}>{version}</span>
      </div>
      <FontAwesomeIcon className={styles.settingsIcon} icon={faCog} />
    </div>
  );
}

export default Header;
