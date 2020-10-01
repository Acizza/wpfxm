import React from "react";
import PrefixPath from "./PrefixPath";
import styles from "./Settings.module.scss";

function Settings(): JSX.Element {
  function formSubmitted(event: React.FormEvent<HTMLDivElement>) {
    event.preventDefault();
  }

  return (
    <div className={styles.panel} onSubmit={formSubmitted}>
      <form className={styles.form}>
        <PrefixPath />
      </form>
    </div>
  );
}

export default Settings;
