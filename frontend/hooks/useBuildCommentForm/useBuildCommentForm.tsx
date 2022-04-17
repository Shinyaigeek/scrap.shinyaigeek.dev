import { useCallback, useMemo } from "react";
import { useForm } from "react-hook-form";

// TODO

interface CommentForm {
  title: string;
  slug: string;
  content: string;
  published: boolean;
}

export const useBuildCommentForm = function () {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<CommentForm>();

  const onSubmit = useCallback(
    handleSubmit((data) => {
      fetch("http://localhost:8080/threads/create", {
        method: "POST",
        body: JSON.stringify(data),
        headers: {
          "Content-Type": "application/json",
        },
      });
    }),
    []
  );

  const Form = useCallback(() => {
    return (
      <form onSubmit={onSubmit}>
        <input {...register("title")} />
        <input {...register("slug")} />
        <input type="checkbox" {...register("published")} />
        <textarea {...register("content")} />
        <input type="submit" />
      </form>
    );
  }, [token]);

  return [Form];
};
