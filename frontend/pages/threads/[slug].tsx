import { useAuthUser } from "../../hooks/useAuthUser/useAuthUser";
interface Props {
  thread: {
    title: string;
    slug: string;
    is_open: boolean;
    content: string;
    id: number;
  };
}

function Thread({ thread }: Props) {
  const { title, slug, content } = thread;
  const { user } = useAuthUser();
  return (
    <>
      <h1>{title}</h1>
      <p>{content}</p>
    </>
  );
}

export const getServerSideProps = async (ctx) => {
  const res = await fetch(`http://localhost:8080/threads/${ctx.query.slug}`);
  const thread = await res.json();
  return {
    props: {
      thread,
    },
  };
};

export default Thread;
