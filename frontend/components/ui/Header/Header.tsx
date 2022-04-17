import { VFC } from "react";
import Link from "next/link";
import { useAuthUser } from "../../../hooks/useAuthUser/useAuthUser";
import { hero } from "./Header.css";

export const Header: VFC = function () {
  const { user, login, signup } = useAuthUser();
  return (
    <header className={hero}>
      scrap.shinyaigeek.dev
      <button onClick={login}>login {user && user.userId}</button>
      <button onClick={signup}>signup </button>
      {user && user.userId}
      {user && user.userId === "Shinyaigeek" && (
        <Link href="/build_thread">
          <a>スレッドを立てる</a>
        </Link>
      )}
    </header>
  );
};
