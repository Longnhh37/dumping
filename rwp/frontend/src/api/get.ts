import type { ToDoItem, ToDoItems } from "../interfaces/toDoItems";
import { Url } from "./url";
import { getCall } from "./utils";

export default async function getAll() {
	return getCall<ToDoItems>(new Url().getAll, 200);
}

export async function getToDoItemById(id: number) {
	return getCall<ToDoItem>(new Url().getByIdUrl(id), 200);
}
