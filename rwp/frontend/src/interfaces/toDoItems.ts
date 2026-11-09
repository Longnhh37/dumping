export enum TaskStatus {
  PENDING = 'PENDING',
  DONE = 'DONE'
}
export interface ToDoItem {
  id: number;
  title: string;
  status: TaskStatus;
}
export interface ToDoItems {
  pending: ToDoItem[];
  done: ToDoItem[];
}
