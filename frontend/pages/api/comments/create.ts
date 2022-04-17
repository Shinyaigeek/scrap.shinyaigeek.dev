import { getSession } from "next-auth/react";

export default async (req, res) => {
  const session = await getSession({ req });
  if (session) {
      console.log(session)
    if (session.user) {
        await fetch("http://localhost:8080/threads/create", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Accept: "application/json",
          },
          body: JSON.stringify(req.body),
        });
    }
  } else {
    // Not Signed in
    res.status(401);
  }
  res.end();
};
