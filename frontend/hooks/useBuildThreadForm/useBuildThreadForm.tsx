import { useCallback, useMemo } from "react";
import { useForm } from "react-hook-form";

export const useBuildThreadForm = function () {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm();

  const onSubmit = useCallback(
    handleSubmit((data) => {
      console.log(data);
    }),
    []
  );

  const Form = useCallback(() => {
    return (
      <form onSubmit={onSubmit}>
        <input {...register("firstName")} />
        <select {...register("gender")}>
          <option value="female">female</option>
          <option value="male">male</option>
          <option value="other">other</option>
        </select>
        <input type="submit" />
      </form>
    );
  }, []);

  return [Form];
};
