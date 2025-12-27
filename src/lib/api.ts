import { invoke } from '@tauri-apps/api/core';

export type TaskPriority = 'Low' | 'Medium' | 'High';
export type TaskStatus = 'Todo' | 'InProgress' | 'Done';

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
  try {
    return invoke('add_task', {
      title,
      description,
      priority,
      dueAt,
    });
  } catch (err) {
    throw err;
  }
}

export function listTasks(): Promise<Task[]> {
  return invoke('list_tasks');
}

export function deleteTask(id: string): Promise<boolean> {
  return invoke('delete_task', { id });
}

export async function updateTaskStatus(taskId: string, status: TaskStatus): Promise<Task> {
  return await invoke('update_task_status', {
    taskId,
    status,
  });
}

export async function updateTask(
  id: string,
  title?: string,
  description?: string | null
): Promise<Task> {
  return await invoke('update_task', {
    id,
    title,
    description,
  });
}
