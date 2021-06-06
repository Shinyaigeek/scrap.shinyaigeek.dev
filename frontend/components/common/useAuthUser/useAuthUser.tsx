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
      factory.signInWithPopup(provider).then(async (user) => {
        console.log(user);
        const token = await user.user.getIdToken();
        const asdf = await fetch(`http://localhost:8080/signin`, {
          headers: {
            Authorization: token
          },
          mode: "cors"
        })
        const j = await asdf.json();
        console.log(j)
        setUser(user.user);
      });
    }
  };

  const signup = async () => {
    if (factory) {
      factory.signInWithPopup(provider).then(async (user) => {
        console.log(user);
        const token = await user.user.getIdToken();
        const asdf = await fetch(`http://localhost:8080/signup`, {
          headers: {
            Authorization: token
          },
          mode: "cors",
          method: "POST"
        })
        const j = await asdf.json();
        console.log(j)
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

  return [user, login, signup, logout] as const;
};
