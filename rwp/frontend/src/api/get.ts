import type { ToDoItem, ToDoItems } from "../interfaces/toDoItems";
import { getCall } from "./utils";
import { Url } from "./url";

export default async function getAll() {
	return getCall<ToDoItems>(new Url().getAll, 200);
}

export async function getToDoItemById(id: number) {
	return getCall<ToDoItem>(new Url().getByIdUrl(id), 200);
}
