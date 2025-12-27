<script lang="ts">
  import { onMount } from 'svelte';
  import { tasks, loadTasks, createTask, removeTask } from '$lib/stores/tasks';

  let title = '';

  onMount(loadTasks);
</script>

<h1>Tasks</h1>

<input
  placeholder="New task"
  bind:value={title}
  on:keydown={(e) => {
    if (e.key === 'Enter' && title) {
      createTask(title);
      title = '';
    }
  }}
/>

<ul>
  {#each $tasks as task}
    <li>
      <strong>{task.title}</strong>
      <button on:click={() => removeTask(task.id)}>✕</button>
    </li>
  {/each}
</ul>
