import React from "react";
import styles from "./Header.module.scss";

interface HeaderProps {
  onLaunchClicked?(): void;
}

function Header(props: HeaderProps): JSX.Element {
  return (
    <div className={styles.panel}>
      <button onClick={props.onLaunchClicked}>Launch</button>
      <button>Edit</button>
    </div>
  );
}

export default Header;
