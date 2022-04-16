interface Props {
  thread: {
    title: string;
    slug: string;
    is_open: boolean;
    content: string;
  };
}

function Thread({ thread }: Props) {
  const { title, slug, content } = thread;
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
