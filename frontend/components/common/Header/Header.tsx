import { VFC } from "react";
import { useAuthUser } from "../useAuthUser/useAuthUser";
import { hero } from "./Header.css";

export const Header: VFC = function () {
  const { user, login, signup } = useAuthUser();
  return (
    <header className={hero}>
      scrap.shinyaigeek.dev
      <button onClick={login}>login {user && user.email}</button>
      <button onClick={signup}>signup </button>
      {user && user.uid}
    </header>
  );
};
