<script lang="ts">
    import type { PageData } from "./$types";
    
    export let data: PageData;
    let note: Note = data;
</script>

<h1>{data.title}</h1>
<hr class="solid">


<label>
    TITLE 
    <input type="text" required bind:value={note.title}>
</label>
<br>
<div style="margin-top:20px"></div>
<label>
    DATE <input type="text" required bind:value={note.pub_date}>
</label>
<h2>Source</h2>
<div class = "text-div">
<textarea bind:value={note.source} rows=30></textarea>
</div>
<div style="text-align: center; width: 100%"><button class="submit"> Save </button></div>

<style>
    textarea {
        width: 100%;
        margin: auto;
    }


    .submit {
        width: 40%;
        margin: auto;
        border-top-width: 0px;
    }
</style>

