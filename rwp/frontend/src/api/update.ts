import type { TaskStatus, ToDoItem, ToDoItems } from "../interfaces/toDoItems";
import { Url } from "./url";
import { patchCall } from "./utils";

export async function updateToDoItemCall(
	id: number,
	title: string,
	status: TaskStatus,
) {
	const toDoItem: ToDoItem = {
		id,
		title,
		status,
	};
	return patchCall<ToDoItems, ToDoItem>(new Url().update, toDoItem, 200);
}
