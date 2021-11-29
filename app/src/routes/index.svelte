<script>
    let longUrl = "";
    let shortUrl = "";

    async function click() {
        console.log("URL", longUrl);
        const api =
            "https://6uy7eq7dpi.execute-api.us-east-1.amazonaws.com/prod/set";
        try {
            const res = await fetch(api, {
                method: "POST",
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    value: longUrl,
                }),
            });
            const url = JSON.parse(await res.text()).url;
            shortUrl = `http://${url}`;
        } catch (e) {
            console.log(e);
        }
    }
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

<input bind:value={longUrl} type="text" />
<button on:click={click}>Shorten</button>

<a href={shortUrl}>{shortUrl}</a>
