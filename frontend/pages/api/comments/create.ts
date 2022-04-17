import { getSession } from "next-auth/react";

const getGitHubIdFromAvatar = (avatar: string) => {
  return avatar.split("/")[4].split("?")[0];
};
export default async (req, res) => {
  const session = await getSession({ req });
  if (session) {
    console.log(session);
    if (session.user) {
      const res = await fetch("http://localhost:8080/comments/create", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        body: JSON.stringify({
          ...req.body,
          author: getGitHubIdFromAvatar(session.user.image),
        }),
      });
      console.log(res)
    }
  } else {
    // Not Signed in
    res.status(401);
  }
  res.end();
};
