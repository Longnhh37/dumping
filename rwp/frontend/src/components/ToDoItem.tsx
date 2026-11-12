import { useState, type FC } from "react";

import { updateToDoItemCall } from "../api/update";
import { deleteToDoItemCall } from "../api/delete";

import type { ApiResult } from "../api/utils";
import { TaskStatus } from "../interfaces/toDoItems";
import type { ToDoItems } from "../interfaces/toDoItems";

interface ToDoItemProps {
  name: string;
  status: TaskStatus;
  passBackResponse: (response: ApiResult<ToDoItems>) => void;
  buttonMessage: string;
}

export const ToDoItem: FC<ToDoItemProps> = ({
  name,
  status,
  passBackResponse,
  buttonMessage,
}) => {
  const [loading, setLoading] = useState(false);

  const isPending = status === TaskStatus.PENDING;

  const sendRequest = async () => {
    if (loading) return;

    setLoading(true);

    try {
      const response = isPending
        ? await updateToDoItemCall(name, TaskStatus.DONE)
        : await deleteToDoItemCall(name);

      passBackResponse(response);
    } catch (e) {
      console.error(`Failed to process task ${name}:`, e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="itemContainer" data-name={name}>
      <p>{name}</p>

      <button className="actionButton" onClick={sendRequest} disabled={loading}>
        {loading ? "Processing..." : buttonMessage}
      </button>
    </div>
  );
};
