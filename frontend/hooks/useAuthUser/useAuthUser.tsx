import { useSession, signIn, signOut } from "next-auth/react"

export const useAuthUser = () => {
  const { data } = useSession();

  const login = async () => {
    signIn()
  };

  const logout = async () => {
    signOut()
  };

  return {
    user: data,
    login,
    logout,
  };
};
