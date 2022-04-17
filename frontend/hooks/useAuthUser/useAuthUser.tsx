import { useSession, signIn, signOut } from "next-auth/react";
import { useState, useEffect } from "react";

interface User {
  username: string;
  github_id: string;
  avatar: string;
}

const getGitHubIdFromAvatar = (avatar: string) => {
  return avatar.split("/")[4].split("?")[0];
};

export const useAuthUser: () => {
  user: User | null;
  login: () => Promise<void>;
  logout: () => Promise<void>;
} = () => {
  const { data } = useSession();
  // TODO cache with recoil
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    if (data?.user?.image) {
      const { image } = data.user;
      const githubId = getGitHubIdFromAvatar(image);
      fetch(`https://api.github.com/user/${githubId}`).then((res) =>
        res.json().then(({ login }) => {
          setUser({
            username: login,
            github_id: githubId,
            avatar: image,
          });
        })
      );
    }
  }, [data]);

  const login = async () => {
    signIn();
  };

  const logout = async () => {
    signOut();
  };

  return {
    user,
    login,
    logout,
  };
};
