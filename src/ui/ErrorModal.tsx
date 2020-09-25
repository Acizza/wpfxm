import React from "react";
import styles from "./ErrorModal.module.scss";

export interface Error {
  context: string;
  message: string;
}

function ErrorModal(props: Error): JSX.Element {
  return (
    <div className={styles.errorModal}>
      <span className={styles.context}>{props.context}</span>
      <span className={styles.message}>{props.message}</span>
    </div>
  );
}

export default ErrorModal;
