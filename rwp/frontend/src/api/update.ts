import { ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { patchCall } from "./utils";
import { Url } from "./url";

export async function updateToDoItemCall(id: number, status: TaskStatus) {
  const toDoItem = {
    id,
    status: status,
  };
  return patchCall<ToDoItems>(new Url().update, toDoItem, 200);
}
