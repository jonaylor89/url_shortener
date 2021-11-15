<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<script>
    import superagent from "superagent";

    let url = "";
    let request = null;

    function click() {
        request = superagent.post(`/api/shorten?url=${url}`);
    }

    function getUrl(key) {
        return `http://${window.location.host}/${key}`;
    }
</script>
