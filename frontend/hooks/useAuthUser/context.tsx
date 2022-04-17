import { createContext, Dispatch, ReactChild, useReducer } from "react";
import {
  ActionType,
  authUserReducer,
  AuthUserState,
  initialAuthUserState,
} from "./reducer";

export interface User {
  avatar: string;
  userId: string;
}

export const AuthUserContext = createContext<AuthUserState | undefined>(
  undefined
);

export const AuthUserUpdateContext = createContext<Dispatch<ActionType>>(null);

export function AuthUserContextProvider({
  children,
}: {
  children: ReactChild;
}) {
  const [user, dispatch] = useReducer(authUserReducer, initialAuthUserState);

  return (
    <AuthUserContext.Provider value={user}>
      <AuthUserUpdateContext.Provider value={dispatch}>
        {children}
      </AuthUserUpdateContext.Provider>
    </AuthUserContext.Provider>
  );
}

export function AuthUserContextMockProvider({
  children,
}: {
  children: ReactChild;
}) {
  const [user, dispatch] = useReducer(authUserReducer, initialAuthUserState);

  return (
    <AuthUserContext.Provider value={user}>
      <AuthUserUpdateContext.Provider value={dispatch}>
        {children}
      </AuthUserUpdateContext.Provider>
    </AuthUserContext.Provider>
  );
}
