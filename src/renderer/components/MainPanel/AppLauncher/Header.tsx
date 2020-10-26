import {
  faPowerOff,
  faTimes,
  faWrench,
  IconDefinition,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import styles from "./Header.module.scss";

interface HeaderProps {
  onLaunchClicked?(): void;
  onCloseClicked?(): void;
  disabled: boolean;
  showLaunchButton: boolean;
}

function Header(props: HeaderProps): JSX.Element {
  const launchToggleBtn = props.showLaunchButton ? (
    <ButtonWithIcon
      icon={faPowerOff}
      label="Launch"
      onClick={props.onLaunchClicked}
      disabled={props.disabled}
    />
  ) : (
    <ButtonWithIcon
      icon={faTimes}
      label="Close"
      onClick={props.onCloseClicked}
      disabled={props.disabled}
    />
  );

  return (
    <div className={styles.panel}>
      {launchToggleBtn}
      <ButtonWithIcon icon={faWrench} label="Edit" />
    </div>
  );
}

interface ButtonWithIconProps {
  icon: IconDefinition;
  label: string;
  [prop: string]: any;
}

function ButtonWithIcon(props: ButtonWithIconProps): JSX.Element {
  const { icon, label, ...rest } = props;

  return (
    <button {...rest}>
      <FontAwesomeIcon icon={icon} className={styles.buttonIcon} />
      {label}
    </button>
  );
}

export default Header;
