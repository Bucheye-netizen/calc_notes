<script>
    export let data;
</script>

<h1>{data.title}</h1>

<div class="sub-info">
   BY {data.author.toUpperCase()}
</div>

<hr class="solid">

{@html data.source}