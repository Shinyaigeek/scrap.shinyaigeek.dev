import { FunctionComponent } from "react";

type ButtonProps = JSX.IntrinsicElements["button"];

export const Button: FunctionComponent<ButtonProps> = function ({ children }) {
  return <button>{children}</button>;
};
