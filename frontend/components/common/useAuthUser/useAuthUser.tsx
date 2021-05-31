import firebase from "firebase/app";
import "firebase/auth";
import { useEffect, useState } from "react";

const firebaseConfig = {
  apiKey: process.env.FIREBASE_API_KEY,
  authDomain: process.env.FIREBASE_AUTHDOMAIN,
};

export const useAuthUser = () => {
  const [token, setToken] = useState<string>("");
  const [user, setUser] = useState<firebase.User>();
  let factory: firebase.auth.Auth;
  const provider = new firebase.auth.GithubAuthProvider();
  useEffect(() => {
    firebase.initializeApp(firebaseConfig);
    factory = firebase.auth();
  }, []);

  const login = async () => {
    if (factory) {
      factory.signInWithPopup(provider).then(user => {
          setUser(user.user);
      });
    }
  };

  const logout = async () => {
    if (factory) {
      factory.signOut();
      setUser(null);
    }
  };

  return [user, login, logout] as const;
};