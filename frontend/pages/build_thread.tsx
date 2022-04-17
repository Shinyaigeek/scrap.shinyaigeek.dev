import Link from "next/link";
import { useEffect } from "react";
import { BuildThreads } from "../components/pages/BuildThreads/BuildThreads";
import { useAuthUser } from "../hooks/useAuthUser/useAuthUser";

interface Props {
  posts: {
    title: string;
    slug: string;
    is_open: boolean;
  }[];
}

function HomePage({ posts }: Props) {
  return (
    <>
      <BuildThreads />
    </>
  );
}

export default HomePage;
