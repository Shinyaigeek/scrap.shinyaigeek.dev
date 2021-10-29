import { useBuildThreadForm } from "../../../hooks/useBuildThreadForm/useBuildThreadForm";

export const BuildThreads: React.FC = function () {
  const [Form] = useBuildThreadForm();
  return (
    <div>
      editorf:
      <Form />
    </div>
  );
};
