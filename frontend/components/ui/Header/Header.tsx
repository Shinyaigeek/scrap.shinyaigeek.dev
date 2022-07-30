import { FunctionComponent } from "react";
import Link from "next/link";
import { useAuthUser } from "../../../hooks/useAuthUser/useAuthUser";
import { Button } from '../Button/Button';

export const Header: FunctionComponent = function () {
  const { user, login, logout } = useAuthUser();
  return (
    <header className="text-xl space-x-3 space-y-3 sticky border-b-2 border-gray-400 px-3 py-2">
      scrap.shinyaigeek.dev
      {user ? (
        <Button onClick={logout}>logout</Button>
      ) : (
        <Button onClick={login}>login</Button>
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
