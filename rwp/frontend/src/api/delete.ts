import { ToDoItems } from "../interfaces/toDoItems";
import { deleteCall } from "./utils";
import { Url } from "./url";

export async function deleteToDoItemCall(id: number) {
  return deleteCall<ToDoItems>(new Url().deleteUrl(id), 200);
}
