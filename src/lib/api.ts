import { invoke } from "@tauri-apps/api/core";

export type TaskPriority = "Low" | "Medium" | "High";
export type TaskStatus = "Todo" | "InProgress" | "Done";

export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: TaskPriority;
  created_at: number;
  due_at?: number;
}

export function addTask(
  title: string,
  description: string | null,
  priority: TaskPriority,
  dueAt?: number
): Promise<Task> {
  return invoke("add_task", {
    title,
    description,
    priority,
    dueAt,
  });
}

export function listTasks(): Promise<Task[]> {
  return invoke("list_tasks");
}

export function deleteTask(id: string): Promise<boolean> {
  return invoke("delete_task", { id });
}


