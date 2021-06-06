import { VFC } from "react";
import { useAuthUser } from "../useAuthUser/useAuthUser";

export const Header: VFC = function() {
    const [user, login, signup] = useAuthUser();
    return <header>
        scrap.shinyaigeek.dev <button onClick={login}>login {user && user.email}</button><button onClick={signup}>signup </button>
    </header>
}