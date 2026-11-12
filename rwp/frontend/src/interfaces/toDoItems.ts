export enum TaskStatus {
  PENDING = 'PENDING',
  DONE = 'DONE'
}

export interface NewToDoItem {
  name: string,
  status: TaskStatus,
}

export interface ToDoItem {
  name: string;
  status: TaskStatus;
}

export interface ToDoItems {
  pending: ToDoItem[];
  done: ToDoItem[];
}
