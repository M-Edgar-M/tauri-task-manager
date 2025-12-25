import { writable } from "svelte/store";
import { listTasks, addTask, deleteTask, type Task } from "../api";

export const tasks = writable<Task[]>([]);

export async function loadTasks() {
  const data = await listTasks();
  tasks.set(data);
}

export async function createTask(title: string) {
  const task = await addTask(title, null, "Medium");
  tasks.update((t) => [...t, task]);
}

export async function removeTask(id: string) {
  const ok = await deleteTask(id);
  if (ok) {
    tasks.update((t) => t.filter((task) => task.id !== id));
  }
}

