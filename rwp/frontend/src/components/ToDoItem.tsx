import { useState, type FC } from "react";

import { updateToDoItemCall } from "../api/update";
import { deleteToDoItemCall } from "../api/delete";

import type { ApiResult } from "../api/utils";
import { TaskStatus } from "../interfaces/toDoItems";
import type { ToDoItems } from "../interfaces/toDoItems";

interface ToDoItemProps {
  id: number;
  title: string;
  status: TaskStatus;
  passBackResponse: (response: ApiResult<ToDoItems>) => void;
  buttonMessage: string;
}

export const ToDoItem: FC<ToDoItemProps> = ({
  id,
  title,
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
        ? await updateToDoItemCall(id, TaskStatus.DONE)
        : await deleteToDoItemCall(id);

      passBackResponse(response);
    } catch (e) {
      console.error(`Failed to process task ${id}:`, e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="itemContainer" data-id={id}>
      <p>{title}</p>

      <button className="actionButton" onClick={sendRequest} disabled={loading}>
        {loading ? "Processing..." : buttonMessage}
      </button>
    </div>
  );
};
