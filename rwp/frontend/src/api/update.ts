import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { patchCall } from "./utils";
import { Url } from "./url";

export async function updateToDoItemCall(
  name: string,
  status: TaskStatus,
) {
  const toDoItem: ToDoItem = {
    name,
    status,
  };
  return patchCall<ToDoItems, ToDoItem>(new Url().update, toDoItem, 200);
}
