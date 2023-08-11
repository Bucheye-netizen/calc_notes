<script lang="ts">
	import { goto } from '$app/navigation';
    import type { PageData } from './$types';

    export let data: PageData;

    function edit(title: string) {
        goto("/edit/notes/" + title);
    }
</script>

<!-- Todo: Increase indentation -->

<h1>Admin dashboard</h1>

<hr class="solid">

<h2>Notes</h2>
<table>
  <tr>
    <th>Title</th>
    <th>Author</th>
    <th>Pub date</th>
  </tr>
  {#each data.values as note}
    <tr>
      <button on:click={() => edit(note.title)}>Edit</button>
      <td>{note.title}</td>
      <td>{note.author}</td>
      <td>{note.pub_date}</td>
    </tr>   
  {/each}
</table>

<style>
  button {
    margin-left: 10px;
    border: none; 
    color: maroon;
  }
</style>