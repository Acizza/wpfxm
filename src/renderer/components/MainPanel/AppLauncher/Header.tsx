import React from "react";
import styles from "./Header.module.scss";

function Header(): JSX.Element {
  return (
    <div className={styles.panel}>
      <button>Launch</button>
      <button>Edit</button>
    </div>
  );
}

export default Header;
