import { FunctionComponent } from "react";
import Link from "next/link";
import { useAuthUser } from "../../../hooks/useAuthUser/useAuthUser";

export const Header: FunctionComponent = function () {
  const { user, login } = useAuthUser();
  return (
    <header className="text-red-400">
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
