import type { FC } from "react";
import { updateToDoItemCall } from "../api/update";
import { deleteToDoItemCall } from "../api/delete";
import type { ApiResult } from "../api/utils";
import { TaskStatus } from "../interfaces/toDoItems";
import type { ToDoItems } from "../interfaces/toDoItems";

interface ToDoItemProps {
  title: string;
  status: TaskStatus;
  passBackResponse: (response: ApiResult<ToDoItems>) => void;
}

export const ToDoItem: FC<ToDoItemProps> = ({
  title,
  status,
  passBackResponse,
}) => {
  const isPending = status === TaskStatus.PENDING;
  const buttonLabel = isPending ? "edit" : "delete";

  const sendRequest = async () => {
    const response = isPending
      ? await updateToDoItemCall(title, TaskStatus.DONE)
      : await deleteToDoItemCall(title);
    passBackResponse(response);
  };

  return (
    <div className="itemContainer">
      <p>{title}</p>
      <button className="actionButton" onClick={sendRequest}>
        {buttonLabel}
      </button>
    </div>
  );
};
