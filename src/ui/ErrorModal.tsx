import { faTimes } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useState } from "react";
import styles from "./ErrorModal.module.scss";

export interface Error {
  context: string;
  message: string;
}

function ErrorModal(props: Error): JSX.Element | null {
  const [open, setOpen] = useState(true);

  if (!open) return null;

  return (
    <div className={styles.errorModal}>
      <div className={styles.header}>
        <span className={styles.context}>{props.context}</span>
        <FontAwesomeIcon
          className={styles.closeIcon}
          icon={faTimes}
          onClick={() => setOpen((prev) => !prev)}
        />
      </div>
      <span className={styles.message}>{props.message}</span>
    </div>
  );
}

export default ErrorModal;
