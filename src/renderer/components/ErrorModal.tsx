import { faTimes } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useState } from "react";
import { DisplayError } from "../types/error";
import styles from "./ErrorModal.module.scss";

function ErrorModal(error: DisplayError): JSX.Element | null {
  const [open, setOpen] = useState(true);

  if (!open) return null;

  return (
    <div className={styles.errorModal}>
      <div className={styles.header}>
        <span className={styles.context}>{error.context}</span>
        <FontAwesomeIcon
          className={styles.closeIcon}
          icon={faTimes}
          onClick={() => setOpen((prev) => !prev)}
        />
      </div>
      <span className={styles.message}>{error.message}</span>
    </div>
  );
}

export default ErrorModal;
