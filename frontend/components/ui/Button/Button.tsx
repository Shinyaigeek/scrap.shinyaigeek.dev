import { FunctionComponent, VFC } from "react";

interface ButtonProps {
  children: React.ReactNode;
}

export const Button: FunctionComponent<ButtonProps> = function ({ children }) {
  return <button>{children}</button>;
};
