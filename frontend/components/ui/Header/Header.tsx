import { VFC } from "react";
import Link from "next/link";
import { useAuthUser } from "../../../hooks/useAuthUser/useAuthUser";
import { hero } from "./Header.css";

export const Header: VFC = function () {
  const { user, login } = useAuthUser();
  return (
    <header className={hero}>
      scrap.shinyaigeek.dev
      <button onClick={login}>login {user && user.username}</button>
      {user && user.username}
      {user && user.username === "Shinyaigeek" && (
        <Link href="/build_thread">
          <a>スレッドを立てる</a>
        </Link>
      )}
    </header>
  );
};
