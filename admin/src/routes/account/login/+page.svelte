<script>
    import { PUBLIC_BACKEND_URL} from '$env/static/public'
    console.log("Backend url " + PUBLIC_BACKEND_URL);
    let password;
    let promise;
    let name; 

    async function login() {
        try {
            await fetch(
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
        } catch (e) {
            console.log(e);
        }
    }

    function handleClick() {
        promise = login();
    }

</script>

<h1> Login page </h1> 

<form>
    <label>
       name 
        <input name="name" type="name" bind:value={name}>
    </label>
    <label>
        Password
        <input name="password" type="password" bind:value={password}>
    </label>
    <button on:click={handleClick}>Log in</button>
</form>

{#await promise}
    <p>Logging in...</p>
{:then response}
    <p>{response}</p>
{/await}