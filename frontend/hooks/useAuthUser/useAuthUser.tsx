import { initializeApp } from "firebase/app";
import {
  getAuth,
  GithubAuthProvider,
  signInWithPopup,
  getAdditionalUserInfo,
} from "firebase/auth";
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
    const provider = new GithubAuthProvider();
    initializeApp(firebaseConfig);
    const factory = getAuth();

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
      signInWithPopup(factory, provider).then(async (user) => {
        const token = await user.user.getIdToken();

        const { username: githubId } = getAdditionalUserInfo(user);
        dispatch(
          authUserActions.successLoginAction({
            token,
            user: {
              userId: githubId,
              avatar: `https://avatars.githubusercontent.com/${githubId}`,
            },
          })
        );
      });
    }
  };

  const signup = async () => {
    if (factory) {
      dispatch(authUserActions.signupAction());
      signInWithPopup(factory, provider).then(async (user) => {
        console.log(user);
        const token = await user.user.getIdToken();
        const { username: githubId } = getAdditionalUserInfo(user);
        dispatch(
          authUserActions.successSignUpAction({
            token,
            user: {
              userId: githubId,
              avatar: `https://avatars.githubusercontent.com/${githubId}`,
            },
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
