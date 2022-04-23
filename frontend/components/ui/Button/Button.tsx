import { FunctionComponent, VFC } from "react";
import { buttonStyle } from "./Button.css";

interface ButtonProps {
  children: React.ReactNode;
}

export const Button: FunctionComponent<ButtonProps> = function ({ children }) {
  return <button className={buttonStyle}>{children}</button>;
};
