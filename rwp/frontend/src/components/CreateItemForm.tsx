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
  const [name, setName] = useState<string>("");

  const handleNameChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const createItem = async () => {
    if (!name.trim()) return;

    const response = await createToDoItemCall(name.trim());
    setName("");
    passBackResponse(response);
  };

  return (
    <div className="inputContainer">
      <input
        type="text"
        id="name"
        placeholder="create to do item"
        value={name}
        onChange={handleNameChange}
      />
      <button className="actionButton" id="create-button" onClick={createItem}>
        Create
      </button>
    </div>
  );
};
