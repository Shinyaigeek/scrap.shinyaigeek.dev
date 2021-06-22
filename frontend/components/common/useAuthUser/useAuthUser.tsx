import firebase from "firebase/app";
import "firebase/auth";
import {
  createContext,
  FC,
  ReactNode,
  useContext,
  useEffect,
  useState,
  VFC,
} from "react";
import { AuthUserContext, AuthUserUpdateContext } from "./context";
import { authUserActions } from "./reducer";

const firebaseConfig = {
  apiKey: process.env.FIREBASE_API_KEY,
  authDomain: process.env.FIREBASE_AUTHDOMAIN,
};

const [provider, factory] = (() => {
  if (typeof window !== "undefined") {
    const provider = new firebase.auth.GithubAuthProvider();
    firebase.initializeApp(firebaseConfig);
    const factory = firebase.auth();

    return [provider, factory] as const;
  }

  return [undefined, undefined] as const;
})();

export const useAuthUser = () => {
  const user = useContext(AuthUserContext);
  const dispatch = useContext(AuthUserUpdateContext);

  const login = async () => {
    if (factory) {
      dispatch(authUserActions.loginAction());
      factory.signInWithPopup(provider).then(async (user) => {
        console.log(user);
        const token = await user.user.getIdToken();
        const asdf = await fetch(`http://localhost:8080/signin`, {
          headers: {
            Authorization: token,
          },
          mode: "cors",
        });
        const j = await asdf.json();
        dispatch(
          authUserActions.successLoginAction({
            token,
            user: user.user,
          })
        );
      });
    }
  };

  const signup = async () => {
    if (factory) {
      dispatch(authUserActions.signupAction());
      factory.signInWithPopup(provider).then(async (user) => {
        console.log(user);
        const token = await user.user.getIdToken();
        const asdf = await fetch(`http://localhost:8080/signup`, {
          headers: {
            Authorization: token,
          },
          mode: "cors",
          method: "POST",
        });
        const j = await asdf.json();
        dispatch(
          authUserActions.successSignUpAction({
            token,
            user: user.user,
          })
        );
      });
    }
  };

  const logout = async () => {
    if (factory) {
      factory.signOut();
      dispatch(authUserActions.logoutAction());
    }
  };

  return {
    user: user.user,
    token: user.token,
    login,
    signup,
    logout,
  };
};
