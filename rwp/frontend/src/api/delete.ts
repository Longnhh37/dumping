import type { ToDoItems } from "../interfaces/toDoItems";
import { Url } from "./url";
import { deleteCall } from "./utils";

export async function deleteToDoItemCall(id: number) {
	return deleteCall<ToDoItems>(new Url().deleteUrl(id), 200);
}
