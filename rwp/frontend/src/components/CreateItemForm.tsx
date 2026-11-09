import { useState } from "react";
import type { ChangeEvent, FC } from "react";
import { createToDoItemCall } from "../api/create";
import type { ApiResult } from "../api/utils";
import type { ToDoItems } from "../interfaces/toDoItems";

interface CreateToDoItemProps {
  passBackResponse: (response: ApiResult<ToDoItems>) => void;
}

export const CreateToDoItem: FC<CreateToDoItemProps> = ({
  passBackResponse,
}) => {
  const [title, setTitle] = useState<string>("");

  const handleTitleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setTitle(e.target.value);
  };

  const createItem = async () => {
    if (!title.trim()) return;

    const response = await createToDoItemCall(title);
    setTitle("");
    passBackResponse(response);
  };

  return (
    <div className="inputContainer">
      <input
        type="text"
        id="name"
        placeholder="create to do item"
        value={title}
        onChange={handleTitleChange}
      />
      <button className="actionButton" id="create-button" onClick={createItem}>
        Create
      </button>
    </div>
  );
};
