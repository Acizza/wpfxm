import { faPowerOff, faWrench } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import styles from "./Header.module.scss";

interface HeaderProps {
  onLaunchClicked?(): void;
}

function Header(props: HeaderProps): JSX.Element {
  return (
    <div className={styles.panel}>
      <button onClick={props.onLaunchClicked}>
        <FontAwesomeIcon icon={faPowerOff} className={styles.icon} />
        Launch
      </button>
      <button>
        <FontAwesomeIcon icon={faWrench} className={styles.icon} />
        Edit
      </button>
    </div>
  );
}

export default Header;
