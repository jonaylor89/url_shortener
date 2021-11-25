<script>
    let longUrl = "";
    let shortUrl = "";

    async function click() {
        const api = "https://6uy7eq7dpi.execute-api.us-east-1.amazonaws.com/prod/set";
        const key = await fetch(api, {
            method: "POST",
            body: JSON.stringify({
                value: longUrl,
            }),
        });
        shortUrl = `http://${window.location.host}/${key}`;
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

<input bind:value={longUrl} type="text" />
<button on:click={click}>Shorten</button>

<a href={shortUrl}>{shortUrl}</a>
