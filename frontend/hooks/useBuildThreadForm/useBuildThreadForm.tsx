import { useCallback, useMemo } from "react";
import { useForm } from "react-hook-form";
import { useAuthUser } from "../useAuthUser/useAuthUser";

interface ThreadForm {
  title: string;
  slug: string;
  content: string;
  published: boolean;
}

export const useBuildThreadForm = function () {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<ThreadForm>();

  const onSubmit = useCallback(
    handleSubmit((data) => {
      fetch("/api/threads/create", {
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
        <div className="flex-col">
          <p>title: </p>
          <input {...register("title")} className="w-12" />
        </div>
        <div>
          <p>slug: </p>
          <input {...register("slug")} />
        </div>
        <div>
          <p>publish: </p>
          <input type="checkbox" {...register("published")} />
        </div>
        <p>content</p>
        <textarea {...register("content")} />
        <input type="submit" />
      </form>
    );
  }, []);

  return [Form];
};
