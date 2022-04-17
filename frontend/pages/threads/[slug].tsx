import { useAuthUser } from "../../hooks/useAuthUser/useAuthUser";
import { useBuildCommentForm } from '../../hooks/useBuildCommentForm/useBuildCommentForm';
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
  const [Form] = useBuildCommentForm(thread.id);
  return (
    <>
      <h1>{title}</h1>
      <p>{content}</p>

      <p>write comment</p>
      <Form />
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
