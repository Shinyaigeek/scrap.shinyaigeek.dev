import { useCallback, useMemo } from "react";
import { useForm } from "react-hook-form";

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
      console.log(data);
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
  }, []);

  return [Form];
};
