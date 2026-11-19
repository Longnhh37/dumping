import {
	type NewToDoItem,
	TaskStatus,
	type ToDoItems,
} from "../interfaces/toDoItems";
import { Url } from "./url";
import { postCall } from "./utils";

export async function createToDoItemCall(title: string) {
	const toDoItem: NewToDoItem = {
		title,
		status: TaskStatus.PENDING,
	};
	return postCall<ToDoItems, NewToDoItem>(new Url().create, toDoItem, 201);
}
