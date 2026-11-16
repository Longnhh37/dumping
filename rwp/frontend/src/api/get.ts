import type { ToDoItem, ToDoItems } from "../interfaces/toDoItems";
import { getCall } from "./utils";
import { Url } from "./url";

export default async function getAll() {
  return getCall<ToDoItems>(new Url().getAll, 200);
}

export async function getToDoItemByName(name: string) {
  return getCall<ToDoItem>(new Url().getByNameUrl(name), 200);
}
