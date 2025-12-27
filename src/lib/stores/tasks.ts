import { writable } from 'svelte/store';
import { listTasks, addTask, deleteTask, updateTaskStatus, type Task } from '../api';

export const tasks = writable<Task[]>([]);

export async function loadTasks() {
  const data = await listTasks();
  tasks.set(data);
}

export async function createTask(title: string) {
  try {
    const task = await addTask(title, null, 'Medium');
    tasks.update((t) => [...t, task]);
  } catch (err: any) {
    alert(err); // this is temprorary
  }
}

export async function removeTask(id: string) {
  const ok = await deleteTask(id);
  if (ok) {
    tasks.update((t) => t.filter((task) => task.id !== id));
  }
}

export async function setUpdateStatus(taskId: string, status: TaskStatus) {
  try {
    const updated = await updateStatus(taskId, status);
    tasks.update((list) => list.map((t) => (t.id === updated.id ? updated : t)));
  } catch (err) {
    console.log('Failed to update task status:', err);
  }
}

export async function editTask(
  id: string,
  updates: { title?: string; description?: string | null }
) {
  try {
    const updated = await updateTask(id, updates.title, updates.description);

    tasks.update((list) => list.map((t) => (t.id === updated.id ? updated : t)));
  } catch (err) {
    console.error('Failed to update task:', err);
  }
}
