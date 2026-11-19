import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { patchCall } from "./utils";
import { Url } from "./url";

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
