<script lang="ts">
    import "../app.css";
    import Navbar from "$lib/Navbar.svelte";
    import { browser } from "$app/environment";
    import { domain } from "$lib/stores";

    if (browser) {
        domain.set(window.location.hostname.split(".").slice(-2).join("."));
    } else {
        domain.set("example.com");
    }
</script>

<svelte:head>
    <link rel="icon" href={`https://media.${$domain}/icons/logo.png`} />
    <title>Video</title>
</svelte:head>

<Navbar />

<slot />
