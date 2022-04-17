import { useCallback, useMemo } from "react";
import { useForm } from "react-hook-form";

// TODO

interface CommentForm {
  title: string;
  slug: string;
  content: string;
  published: boolean;
}

export const useBuildCommentForm = function (thread: number) {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<CommentForm>();

  const onSubmit = useCallback(
    handleSubmit((data) => {
      fetch("/api/comments/create", {
        method: "POST",
        body: JSON.stringify({
          ...data,
          thread,
        }),
        headers: {
          "Content-Type": "application/json",
        },
      });
    }),
    [thread]
  );

  const Form = useCallback(() => {
    return (
      <form onSubmit={onSubmit}>
        <textarea {...register("content")} />
        <input type="submit" />
      </form>
    );
  }, [thread]);

  return [Form];
};
