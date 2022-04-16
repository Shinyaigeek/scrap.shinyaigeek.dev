import Link from "next/link";
import { useAuthUser } from "../hooks/useAuthUser/useAuthUser";

interface Props {
  posts: {
    title: string;
    slug: string;
    is_open: boolean;
  }[];
}

function HomePage({ posts }: Props) {
  const { user } = useAuthUser();
  return (
    <>
      <div>Welcome to Next.js! {user && user.email}</div>
      <ul>
        {posts.map((post) => (
          <li key={post.slug}>
            <Link href="/threads/[slug]" as={`/threads/${post.slug}`}>
              <a>{post.title}</a>
            </Link>
          </li>
        ))}
      </ul>
    </>
  );
}

export const getServerSideProps = async (ctx) => {
  const res = await fetch("http://localhost:8080/threads");
  const posts = await res.json();
  return {
    props: {
      posts,
    },
  };
};

export default HomePage;
