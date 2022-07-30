import { FunctionComponent } from "react";
import Link from "next/link";
import { useAuthUser } from "../../../hooks/useAuthUser/useAuthUser";

export const Header: FunctionComponent = function () {
  const { user, login, logout } = useAuthUser();
  return (
    <header className="text-xl space-x-3 space-y-3 border-yellow-300">
      scrap.shinyaigeek.dev
      {user ? (
        <button onClick={logout}>logout {user && user.username}</button>
      ) : (
        <button onClick={login}>login</button>
      )}
      {user && user.username}
      {user && user.username === "Shinyaigeek" && (
        <Link href="/build_thread">
          <a>スレッドを立てる</a>
        </Link>
      )}
    </header>
  );
};
