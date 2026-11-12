import { NewToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { postCall } from "./utils";
import { Url } from "./url";

export async function createToDoItemCall(name: string) {
  const toDoItem: NewToDoItem = {
    name,
    status: TaskStatus.PENDING,
  };
  return postCall<ToDoItems, NewToDoItem>(new Url().create, toDoItem, 201);
}
