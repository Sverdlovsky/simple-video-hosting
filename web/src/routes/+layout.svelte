<script lang="ts">
  import "../app.css";
  import Navbar from "$lib/Navbar.svelte";

  const title: string = window.location.hostname.split(".")[0];
</script>

<svelte:head>
  <link rel="icon" href="/svg/favicon.png" />
  <title>{title}</title>
</svelte:head>

<Navbar />

<slot />
