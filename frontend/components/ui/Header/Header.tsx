import { VFC } from "react";
import Link from "next/link";
import { useAuthUser } from "../../../hooks/useAuthUser/useAuthUser";
import { hero } from "./Header.css";

export const Header: VFC = function () {
  const { user, login } = useAuthUser();
  console.log(user)
  return (
    <header className={hero}>
      scrap.shinyaigeek.dev
      <button onClick={login}>login {user?.user && user.user.name}</button>
      {user?.user && user.user.name}
      {user?.user && user.user.name === "Shinyaigeek" && (
        <Link href="/build_thread">
          <a>スレッドを立てる</a>
        </Link>
      )}
    </header>
  );
};
