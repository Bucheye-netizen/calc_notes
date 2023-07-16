<script>
	import { goto } from '$app/navigation';
    import { PUBLIC_BACKEND_URL } from '$env/static/public';
    import { BarLoader } from 'svelte-loading-spinners';

    async function login(name, password) {
        try {
            const res = await fetch(
                PUBLIC_BACKEND_URL + "/api/auth/login", 
                {
                    method: "POST",
                    mode: "cors",
                    credentials: "include",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify([name, password])
                }
            ); 
        
            if (!res.ok) { return false; }
        } catch (e) {
            return false;
        }
        goto("/");
    }

    let valid; 
    let name;
    let password;

    function on_submit() {
        valid = login(name, password);
    }
</script>

<h1> Login page </h1> 

<form on:submit={on_submit} >
    <label>
       NAME
        <input bind:value={name} type="text" required>
    </label>
    <br>
    <label>
        PASSWORD
        <input bind:value={password} type="text" required>
    </label>
    <br>
    <div class="button"><button on:click={on_submit}>LOGIN</button></div>

    {#await valid} 
        <div class="loader">
            <BarLoader size="60" color="darkred" unit="px" duration="2s" />
        </div>
    {:then value}
        {#if value == false}
            <p>Failed to login</p>
        {/if}
    {/await}
</form>

<style lang="scss">
    .button {
        text-align: center;
    }

    .loader {
        margin-top: 30px;
        justify-content: center;
        display: flex;
    }

    button {
        border-top: 0;
    }

    form {
        max-width: 300px;
        margin: auto;
    }
    label {
        width: 100%;
        margin: 10px;
        display:inline-block;
    }
    input {
        width: 100%;
        display:inline-block;
    }
</style>