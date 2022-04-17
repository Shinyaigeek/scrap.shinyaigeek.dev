import { BuildThreads } from "../components/pages/BuildThreads/BuildThreads";

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
