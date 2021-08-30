import Link from "next/link";
import { useEffect } from "react";
import { useAuthUser } from "../components/common/useAuthUser/useAuthUser";

interface Props {
  posts: {
    title: string;
    slug: string;
    is_open: boolean;
  }[];
}

function HomePage({ posts }: Props) {
  const { user } = useAuthUser();

  useEffect(() => {
    if (!user) {
      window.location.href = "/";
    }

    if (user.email !== "me@shinyaigeek.dev") {
      window.location.href = "/";
    }
  }, [user]);

  return <>editor</>;
}

export default HomePage;
