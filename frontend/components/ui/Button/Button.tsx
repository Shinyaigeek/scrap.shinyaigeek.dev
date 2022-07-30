import { FunctionComponent } from "react";

type ButtonProps = JSX.IntrinsicElements["button"];

export const Button: FunctionComponent<ButtonProps> = function ({
  children,
  onClick,
}) {
  return (
    <button className="px-1 py-1 border-cyan-100 border-2 rounded" onClick={onClick}>
      {children}
    </button>
  );
};
