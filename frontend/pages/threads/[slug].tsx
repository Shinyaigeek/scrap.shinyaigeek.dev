import { useBuildCommentForm } from "../../hooks/useBuildCommentForm/useBuildCommentForm";
interface Props {
  thread: {
    title: string;
    slug: string;
    is_open: boolean;
    content: string;
    id: number;
    comments: {
      content: string;
      author: string;
      thread: number;
    }[];
  };
}

function Thread({ thread, }: Props) {
  const { title, slug, content } = thread;
  const [Form] = useBuildCommentForm(thread.id);
  return (
    <main className="bg-white w-4/5 m-auto">
      <h1 className="text-xl text-center">{title}</h1>
      <p>{content}</p>

      <h2>comments</h2>
      {thread.comments.map((comment) => (<div>{comment.content}</div>))}

      <p>write comment</p>
      <Form />
    </main>
  );
}

export const getServerSideProps = async (ctx) => {
  const res = await fetch(
    `http://localhost:3000/api/thread/read?slug=${ctx.query.slug}`
  );
  const thread = await res.json();
  return {
    props: {
      thread,
    },
  };
};

export default Thread;
