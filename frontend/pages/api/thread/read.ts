export default async (req, res) => {
  const threadResponse = await fetch(
    `http://localhost:8080/threads/${req.query.slug}`
  );
  const thread = await threadResponse.json();
  const commentsResponse = await fetch(
    `http://localhost:8080/comments?thread=${thread.id}`
  );
  const comments = await commentsResponse.json();
  res.json({
    ...thread,
    comments,
  });
};
